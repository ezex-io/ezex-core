use crate::TestContext;
use common::{
    consts::*,
    test_tools,
    topic::{TopicMessage, deposit, general},
};
use httpmock::Method::POST;

pub async fn test_generate_address(ctx: &mut TestContext) {
    let msg = deposit::address::Generate {
        user_id: "alice".to_string(),
        coin: asset::name::TBTC.to_string(),
        wallet_id: "60def63ab9390d000630211559c1544d".to_string(),
    };
    ctx.redis.add_message(&msg, module::name::DEPOSIT);
    ctx.redis.should_ack(msg.topic()).await;
    //delete the mock from mock server.
    mock.delete();
}

pub async fn test_error_message(ctx: &mut TestContext) {
    let msg = deposit::address::Generate {
        user_id: "bob".to_string(),
        coin: asset::name::TBTC.to_string(),
        wallet_id: "60def63ab9390d000630211559c1544d".to_string(),
    };
    ctx.redis.add_message(&msg, module::name::DEPOSIT);
    ctx.redis.should_ack(msg.topic()).await;

    let (_, msg) = ctx
        .redis
        .xread_sync::<general::internal::Error>(general::internal::Error::name, "0-0");
    assert_eq!(
        msg.message,
        "Error: \"Attempt to use IP-restricted token from an unauthorized IP address\""
    );
    //delete the mock from mock server.
    mock.delete();
}
