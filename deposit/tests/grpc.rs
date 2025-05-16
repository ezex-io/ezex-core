use crate::TestContext;
use ezex_deposit::grpc::deposit::{GetAddressRequest, VersionRequest};

pub async fn test_grpc_version(ctx: &mut TestContext) {
    let request = VersionRequest {};
    let res = ctx.grpc_client.version(request).await.unwrap();
    assert_eq!(res.get_ref().version, env!("CARGO_PKG_VERSION"));
}

pub async fn test_get_address(ctx: &mut TestContext) {
    let request = GetAddressRequest {
        user_id: "alice".to_string(),
        chain_id: "Pactus".to_string(),
        asset_id: "PAC".to_string(),
    };
    let res = ctx.grpc_client.get_address(request).await.unwrap();
    assert!(!res.get_ref().has_address);
    assert_eq!(res.get_ref().address, "");
}
