use crate::TestContext;
use chains_rs::{asset::Asset, pactus::PactusPACAsset};
use common::topic::{TopicMessage, deposit};

async fn send_and_receive_generate_message(
    ctx: &mut TestContext,
    user_id: &str,
    expected_address: &str,
) {
    let asset = PactusPACAsset;
    let msg = deposit::address::Generate {
        user_id: user_id.to_string(),
        coin: asset.name().to_string(),
        wallet_id: "60def63ab9390d000630211559c1544d".to_string(),
    };

    ctx.redis.add_message(&msg, "deposit");
    ctx.redis.should_ack(msg.topic()).await;

    let (_, generated) = ctx
        .redis
        .xread_sync::<deposit::address::Generated>(deposit::address::Generated::name, "0-0");

    assert_eq!(generated.deposit_address, expected_address);
    assert_eq!(generated.coin, asset.name().to_string());
    assert_eq!(generated.user_id, user_id);
    assert_eq!(generated.wallet_id, msg.wallet_id);
}

pub async fn test_generate_valid_address(ctx: &mut TestContext) {
    send_and_receive_generate_message(ctx, "alice", "0x5a0b54d5dc17e0aadc383d2db43b0a0d3e029c4c")
        .await;
}

pub async fn test_generate_error_address(ctx: &mut TestContext) {
    send_and_receive_generate_message(ctx, "bob", "0x0000000000000000000000000000000000000000")
        .await;
}
