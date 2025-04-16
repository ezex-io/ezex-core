use crate::TestContext;
use common::consts::asset;
use deposit_vault::api::grpc::deposit::{
    AddressRequest,
    VersionRequest,
};

pub async fn test_grpc_version(ctx: &mut TestContext) {
    let request = VersionRequest {};
    let res = ctx.grpc_client.version(request).await.unwrap();
    assert_eq!(res.get_ref().version, env!("CARGO_PKG_VERSION"));
}

pub async fn test_get_address(ctx: &mut TestContext) {
    let request = AddressRequest {
        user_id: "alice".to_string(),
        coin: asset::name::TBTC.to_string(),
    };
    let res = ctx.grpc_client.get_address(request).await.unwrap();
    assert_eq!(
        res.get_ref().deposit_address,
        "2N4sexvpWpMUjoVHHFXuAUitngG8pwb2sKf"
    );
}

pub async fn test_address_not_exist(ctx: &mut TestContext) {
    let request = AddressRequest {
        user_id: "bob".to_string(),
        coin: asset::name::TBTC.to_string(),
    };
    let res = ctx.grpc_client.get_address(request).await;
    assert!(res.is_err());
}
