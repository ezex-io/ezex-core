pub const NAME: &str = "deposit";

pub mod address {
    use crate::topic::TopicMessage;
    use procedural::Topic;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize, Debug, Topic)]
    #[topic_name("deposit:address:generate")]
    pub struct Generate {
        pub user_id: String,
        pub coin: String,
        pub wallet_id: String,
    }

    #[derive(Clone, Serialize, Deserialize, Debug, Topic)]
    #[topic_name("deposit:address:generated")]
    pub struct Generated {
        pub user_id: String,
        pub coin: String,
        pub chain_id: String,
        pub wallet_id: String,
        pub deposit_address: String,
    }
}

pub mod transaction {
    use crate::topic::TopicMessage;
    use procedural::Topic;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize, Debug, Topic, Eq, PartialEq)]
    #[topic_name("deposit:transaction:claim")]
    pub struct Claim {
        pub onchain_tx_id: String,
        pub coin: String,
        pub wallet_id: String,
    }

    #[derive(Clone, Serialize, Deserialize, Debug, Topic, Eq, PartialEq)]
    #[topic_name("deposit:transaction:unconfirmed")]
    pub struct Unconfirmed {
        pub deposit_id: String,
        pub onchain_tx_id: String,
        pub chain_id: String,
        pub user_id: String,
        pub coin: String,
        pub amount: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone, Topic)]
    #[topic_name("deposit:transaction:confirmed")]
    pub struct Confirmed {
        pub deposit_id: String,
        pub onchain_tx_id: String,
        pub chain_id: String,
        pub user_id: String,
        pub coin: String,
        pub amount: String,
    }
}

