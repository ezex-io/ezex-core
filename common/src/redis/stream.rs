use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct StreamFields {
    pub module: String,
    pub correlation_id: Option<String>,
    pub message: String,
}

impl StreamFields {
    pub fn encode(&self) -> Result<HashMap<String, redis::Value>, serde_redis::encode::Error> {
        // serde_redis::encode::to_map(self)
        todo!()
    }
}

pub fn decode(
    fields: &HashMap<String, redis::Value>,
) -> Result<StreamFields, serde_redis::decode::Error> {
    let mut vec = Vec::new();
    for (k, v) in fields {
        vec.push(redis::Value::BulkString(k.as_bytes().to_vec()));
        vec.push(v.clone());
    }

    let bulk = redis::Value::Array(vec);
    let de = serde_redis::decode::Deserializer::new(bulk);
    Deserialize::deserialize(de)
}
