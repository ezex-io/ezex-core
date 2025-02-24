use erased_serde::{Serialize, Serializer};
use serde::de::Error;
use std::any::Any;
use std::fmt::Debug;

pub mod deposit;

pub trait TopicMessage: Send + Sync + Debug + Serialize {
    fn topic(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;

    fn to_json_string(&self) -> std::result::Result<String, serde_json::Error> {
        let mut str = vec![];
        let mut serializer = serde_json::Serializer::new(&mut str);
        self.erased_serialize(&mut <dyn Serializer>::erase(&mut serializer))
            .map_err(serde_json::Error::custom)?;

        Ok(std::str::from_utf8(&str)
            .map_err(serde_json::Error::custom)?
            .to_string())
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_json_encoding() {
        let e1 = super::deposit::transaction::Claim {
            onchain_tx_id: "id_1".to_string(),
            coin: "coin_1".to_string(),
            wallet_id: "wallet_1".to_string(),
        };

        let t: Box<dyn super::TopicMessage> = Box::new(e1.clone());
        let j = t.to_json_string().unwrap();
        let e2: super::deposit::transaction::Claim = serde_json::from_str(&j).unwrap();
        assert_eq!(e1, e2);
    }
}
