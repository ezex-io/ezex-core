use redis::{RedisError, Value};
use serde::{Deserialize, Serialize};
use serde_redis::decode;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct StreamFields {
    pub module: String,
    pub correlation_id: Option<String>,
    pub message: String,
}

impl StreamFields {
    pub fn encode(&self) -> Result<HashMap<String, Value>, RedisError> {
        let mut map = HashMap::new();

        map.insert(
            "module".to_string(),
            Value::BulkString(self.module.clone().into_bytes()),
        );
        map.insert(
            "correlation_id".to_string(),
            match &self.correlation_id {
                Some(id) => Value::BulkString(id.clone().into_bytes()),
                None => Value::Nil,
            },
        );
        map.insert(
            "message".to_string(),
            Value::BulkString(self.message.clone().into_bytes()),
        );

        Ok(map)
    }
}

pub fn decode(fields: &HashMap<String, Value>) -> Result<StreamFields, decode::Error> {
    let mut vec = Vec::new();
    for (k, v) in fields {
        vec.push(Value::BulkString(k.as_bytes().to_vec()));
        vec.push(v.clone());
    }

    let bulk = Value::Array(vec);
    // Convert to Cow<'static, Value> to match the AsValueVec<'static> implementation
    let de = decode::Deserializer::new(std::borrow::Cow::Owned(bulk));
    Deserialize::deserialize(de)
}
