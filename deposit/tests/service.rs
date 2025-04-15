mod grpc;
mod topics;

use assert_cmd::prelude::*;
use async_std::task::sleep;
use common::test_tools::{
    self,
    *,
};
use deposit_vault::api::grpc::deposit::vault_service_client::VaultServiceClient;
use httpmock::prelude::*;
use std::{
    process::{
        Child,
        Command,
        Stdio,
    },
    time::Duration,
};
use tonic::transport::Channel;

pub struct TestContext {
    /// pq_db is the instance of Postgres Database.
    /// It should be alive otherwise it will be dropped at the end of `setup` function.
    pub pq_db: PostgresTestDB,
    pub grpc_client: VaultServiceClient<Channel>,
    pub redis: RedisTestClient,
    pub child: Child,
}
impl TestContext {
    pub async fn setup() -> TestContext {
        let pq_db = PostgresTestDB::new();

        let database_url = pq_db.con_string();
        let redis_con_string = test_tools::redis_test_con_string();
        let grpc_address = format!("127.0.0.1:{}", test_tools::pick_unused_port());

        let redis_group = "deposit-vault-group-1";
        let redis = RedisTestClient::new(&redis_con_string, redis_group);

        let mut cmd = Command::cargo_bin("deposit-vault").unwrap();
        let child = cmd
            .arg("start")
            .env("LOG_LEVEL", "debug")
            .env("GRPC_ADDRESS", grpc_address.clone())
            .env("REDIS_CONNECTION_STRING", redis_con_string)
            .env("REDIS_CONSUMER", "deposit-vault-consumer-1")
            .env("REDIS_GROUP_NAME", redis_group)
            .env("DATABASE_URL", database_url)
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process");

        // Make sure gRPC server is up (in 5 seconds)
        let mut counter: i32 = 0;
        let client_res = loop {
            counter += 1;

            let client_res =
                VaultServiceClient::connect(format!("http://{}", grpc_address.clone())).await;
            if counter < 50 && client_res.is_err() {
                sleep(Duration::from_millis(100)).await;
                continue;
            } else {
                break client_res;
            }
        };
        let grpc_client = client_res.unwrap();

        TestContext {
            pq_db,
            grpc_client,
            redis,
            child,
        }
    }
}

#[tokio::test]
#[serial_test::serial]
async fn test_deposit_vault() {
    let mut ctx = TestContext::setup().await;
    grpc::test_grpc_version(&mut ctx).await;
    topics::test_generate_address(&mut ctx).await;
    grpc::test_get_address(&mut ctx).await;
    grpc::test_address_not_exist(&mut ctx).await;
    sleep(Duration::from_secs(5)).await;
    // send SIGINT to the child
    nix::sys::signal::kill(
        nix::unistd::Pid::from_raw(ctx.child.id() as i32),
        nix::sys::signal::Signal::SIGINT,
    )
    .expect("cannot send ctrl-c");

    // wait for child to terminate
    ctx.child.wait().unwrap();

    println!("Test is finished");
}
