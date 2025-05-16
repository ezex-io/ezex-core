use crate::TestContext;
use chains_rs::{asset::Asset, pactus::PactusPACAsset};
use common::event::{TopicMessage, deposit};

async fn send_and_receive_generate_message(
    ctx: &mut TestContext,
    user_id: &str,
    expected_address: &str,
) {
    let asset = PactusPACAsset;
    let msg = deposit::address::Generate {
        user_id: "alice".to_string(),
        coin: "PAC".to_string(),
        wallet_id: "60def63ab9390d000630211559c1544d".to_string(),
    };
    ctx.redis.add_message(&msg, "deposit");
    ctx.redis.should_ack(msg.topic()).await;
}

pub async fn test_error_message(ctx: &mut TestContext) {
    let msg = deposit::address::Generate {
        user_id: "bob".to_string(),
        coin: "PAC".to_string(),
        wallet_id: "60def63ab9390d000630211559c1544d".to_string(),
    };
    ctx.redis.add_message(&msg, "deposit");
    ctx.redis.should_ack(msg.topic()).await;

    // let (_, msg) = ctx
    //     .redis
    //     .xread_sync::<general::internal::Error>(general::internal::Error::name, "0-0");
    // assert_eq!(
    //     msg.message,
    //     "Error: \"Attempt to use IP-restricted token from an unauthorized IP address\""
    // );
}
