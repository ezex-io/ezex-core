use crate::topic::TopicMessage;
use async_std::task::sleep;
use diesel::{Connection, PgConnection, RunQueryDsl};
use redis::{
    streams::{StreamPendingReply, StreamReadReply},
    Commands,
};
use redis_stream_bus::client::Stream;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{
    collections::{BTreeMap, HashMap},
    env,
    time::Duration,
};

pub fn redis_test_con_string() -> String {
    env::var("REDIS_CONNECTION_STRING").unwrap_or_else(|_e| String::from("redis://localhost:6379"))
}

pub fn postgres_test_db_url() -> String {
    env::var("DATABASE_URL")
        .unwrap_or_else(|_e| String::from("postgres://postgres:postgres@localhost:5432"))
}

pub fn pick_unused_port() -> u16 {
    portpicker::pick_unused_port().unwrap()
}


pub struct RedisTestClient {
    client: redis::Client,
    group: String,
}

impl RedisTestClient {
    pub fn new(connection_string: &str, group: &str) -> Self {
        let client = redis::Client::open(connection_string).unwrap();
        let mut con = client.get_connection().unwrap();
        redis::cmd("FLUSHALL").execute(&mut con);

        RedisTestClient {
            client,
            group: group.to_string(),
        }
    }

    pub fn add_message(&mut self, msg: &dyn TopicMessage, module_name: &str) -> String {
        let fields = crate::stream::StreamFields {
            module: module_name.to_string(),
            correlation_id: None,
            message: msg.to_json_string().unwrap(),
        };

        let stream = Stream::new(msg.topic(), None, fields.encode().unwrap());

        let mut map = BTreeMap::new();
        for (k, v) in stream.fields {
            if let redis::Value::Data(d) = v {
                map.insert(k, d);
            }
        }

        self.client
            .xadd_map::<_, _, BTreeMap<String, Vec<u8>>, String>(msg.topic(), "*", map)
            .unwrap()
    }

    pub async fn should_ack<'a>(&mut self, key: &str) {
        // Wait for 5 seconds
        let mut counter: i32 = 0;
        loop {
            counter += 1;
            let res: StreamPendingReply = self.client.xpending(key, self.group.clone()).unwrap();
            if let StreamPendingReply::Empty = res {
                return;
            } else {
                assert!(
                    !(counter == 50),
                    "Expected StreamPendingReply::Empty but got Data. key: {}",
                    key
                );
                sleep(Duration::from_millis(100)).await;
            }
        }
    }

    // ID for unread message can be set to "$"
    // To read messages from the beginning set ID to "0-0"
    pub fn xread_sync<T: DeserializeOwned>(&mut self, key: &str, id: &str) -> (Stream, T) {
        let res = self
            .client
            .xread::<_, _, StreamReadReply>(&[key], &[id])
            .unwrap();

        let stream_key = &res.keys[0];
        let stream_id = &stream_key.ids[0];
        let stream = Stream::new(
            &stream_key.key,
            Some(stream_id.id.clone()),
            stream_id.map.clone(),
        );
        let fields = crate::stream::decode(&stream.fields).unwrap();
        let msg: T = serde_json::from_str(&fields.message).unwrap();
        (stream, msg)
    }

    pub fn xlen(&mut self, key: &str) -> i32 {
        self.client.xlen::<&str, i32>(key).unwrap()
    }
}

pub struct PostgresTestDB {
    pub conn: PgConnection,
    pub base_url: String,
    pub db_name: String,
}

impl PostgresTestDB {
    pub fn new() -> Self {
        let base_url = postgres_test_db_url();

        let conn =
            PgConnection::establish(&base_url).expect("Cannot connect to postgres database.");

        let db_name = format!("test_db_{}", rand::random::<u16>());

        // Create a new database for the test
        let query = diesel::sql_query(format!("CREATE DATABASE {};", db_name).as_str());
        query.execute(&conn).unwrap();

        Self {
            conn,
            base_url,
            db_name,
        }
    }

    pub fn con_string(&self) -> String {
        format!("{}/{}", self.base_url, self.db_name)
    }

    pub fn drop_database(&mut self) {
        let disconnect_users = format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}';",
            self.db_name
        );
        diesel::sql_query(disconnect_users.as_str())
            .execute(&self.conn)
            .as_ref()
            .unwrap();

        let query =
            diesel::sql_query(format!("DROP DATABASE IF EXISTS {};", self.db_name).as_str());
        query.execute(&self.conn).unwrap();
    }
}

impl Drop for PostgresTestDB {
    fn drop(&mut self) {
        self.drop_database();
    }
}

impl Default for PostgresTestDB {
    fn default() -> Self {
        Self::new()
    }
}
