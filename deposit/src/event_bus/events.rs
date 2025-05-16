pub const NAME: &str = "deposit";

pub mod general {
    use common::event::EventMessage;
    use procedural::Event;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize, Debug, Event)]
    #[event_key("deposit:general:started")]
    pub struct Started {
        pub version: String,
        // TODO: Maybe start time?
    }
}

pub mod address {
    use common::event::EventMessage;
    use procedural::Event;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Serialize, Deserialize, Debug, Event)]
    #[event_key("deposit:address:generated")]
    pub struct Generated {
        pub user_id: String,
        pub chain_id: String,
        pub asset_id: String,
        pub address: String,
    }
}

pub mod transaction {
    use common::event::EventMessage;
    use procedural::Event;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone, Event)]
    #[event_key("deposit:transaction:confirmed")]
    pub struct Confirmed {
        pub deposit_id: String,
        pub onchain_tx_id: String,
        pub chain_id: String,
        pub user_id: String,
        pub coin: String,
        pub amount: String,
    }
}
