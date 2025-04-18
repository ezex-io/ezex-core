use crate::topic::TopicMessage;
// use async_std::task::sleep;
use diesel::{
    Connection,
    PgConnection,
    RunQueryDsl,
};
use redis::{
    Commands,
    streams::{
        StreamPendingReply,
        StreamReadReply,
    },
};
use redis_stream_bus::client::Stream;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::{
    collections::{
        BTreeMap,
        HashMap,
    },
    env,
    time::Duration,
};

pub struct PostgresTestDB {
    pub conn: PgConnection,
    pub database_url: String,
    pub db_name: String,
}

impl PostgresTestDB {
    fn postgres_database_url() -> String {
        env::var("POSTGRES_DATABASE_URL")
            .unwrap_or_else(|_e| String::from("postgres://postgres:postgres@localhost:5432"))
    }

    pub fn new() -> Self {
        let database_url = Self::postgres_database_url();

        let mut conn =
            PgConnection::establish(&database_url).expect("Cannot connect to postgres database.");

        let db_name = format!("test_db_{}", rand::random::<u16>());

        // Create a new database for the test
        let query = diesel::sql_query(format!("CREATE DATABASE {};", db_name).as_str());
        query.execute(&mut conn).unwrap();

        Self {
            conn,
            database_url,
            db_name,
        }
    }

    pub fn con_string(&self) -> String {
        format!("{}/{}", self.database_url, self.db_name)
    }

    pub fn drop_database(&mut self) {
        let disconnect_users = format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}';",
            self.db_name
        );
        diesel::sql_query(disconnect_users.as_str())
            .execute(&mut self.conn)
            .as_ref()
            .unwrap();

        let query =
            diesel::sql_query(format!("DROP DATABASE IF EXISTS {};", self.db_name).as_str());
        query.execute(&mut self.conn).unwrap();
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
