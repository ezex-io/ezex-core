use crate::topic::TopicMessage;
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
use tokio::time::sleep;

pub struct RedisTestClient {
    client: redis::Client,
    group: String,
}

impl RedisTestClient {
    fn redis_connection_string() -> String {
        env::var("REDIS_CONNECTION_STRING")
            .unwrap_or_else(|_e| String::from("redis://localhost:6379"))
    }

    pub fn new(group: &str) -> Self {
        let connection_string = Self::redis_connection_string();
        let client = redis::Client::open(connection_string).unwrap();
        let mut con = client.get_connection().unwrap();
        redis::cmd("FLUSHALL").exec(&mut con).unwrap();

        RedisTestClient {
            client,
            group: group.to_string(),
        }
    }

    pub fn add_message(&mut self, msg: &dyn TopicMessage, module_name: &str) -> String {
        todo!()
        // let fields = crate::stream::StreamFields {
        //     module: module_name.to_string(),
        //     correlation_id: None,
        //     message: msg.to_json_string().unwrap(),
        // };

        // let stream = Stream::new(msg.topic(), None, fields.encode().unwrap());

        // let mut map = BTreeMap::new();
        // for (k, v) in stream.fields {
        //     if let redis::Value::Data(d) = v {
        //         map.insert(k, d);
        //     }
        // }

        // self.client
        //     .xadd_map::<_, _, BTreeMap<String, Vec<u8>>, String>(msg.topic(), "*", map)
        //     .unwrap()
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
        todo!()
        // let res = self
        //     .client
        //     .xread::<_, _, StreamReadReply>(&[key], &[id])
        //     .unwrap();

        // let stream_key = &res.keys[0];
        // let stream_id = &stream_key.ids[0];
        // let stream = Stream::new(
        //     &stream_key.key,
        //     Some(stream_id.id.clone()),
        //     stream_id.map.clone(),
        // );
        // let fields = crate::stream::decode(&stream.fields).unwrap();
        // let msg: T = serde_json::from_str(&fields.message).unwrap();
        // (stream, msg)
    }

    pub fn xlen(&mut self, key: &str) -> i32 {
        self.client.xlen::<&str, i32>(key).unwrap()
    }
}
