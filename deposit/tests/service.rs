mod grpc;

use assert_cmd::prelude::*;

use common::testsuite::{self, postgres::PostgresTestDB, redis::RedisTestClient};
use ezex_deposit::grpc::deposit::deposit_service_client::DepositServiceClient;
use std::{
    process::{Child, Command, Stdio},
    time::Duration,
};
use tokio::time::sleep;
use tonic::transport::Channel;

pub struct TestContext {
    /// pq_db is the instance of Postgres Database.
    /// It should be alive otherwise it will be dropped at the end of `setup` function.
    pub pq_db: PostgresTestDB,
    pub grpc_client: DepositServiceClient<Channel>,
    pub redis: RedisTestClient,
    pub child: Child,
}

impl TestContext {
    pub async fn setup() -> TestContext {
        let pq_db = PostgresTestDB::new();

        let database_url = pq_db.con_string();
        let grpc_address = format!("127.0.0.1:{}", testsuite::pick_unused_port());

        let redis_group = "deposit-group-1";
        let redis = RedisTestClient::new(redis_group);
        let redis_con_string = redis.connection_string();

        let mut cmd = Command::cargo_bin("ezex-deposit").unwrap();
        let child = cmd
            .arg("start")
            .env("LOG_LEVEL", "debug")
            .env("GRPC_ADDRESS", grpc_address.clone())
            .env("REDIS_CONNECTION_STRING", redis_con_string)
            .env("REDIS_CONSUMER_NAME", "deposit-consumer-1")
            .env("REDIS_GROUP_NAME", redis_group)
            .env("POSTGRES_DATABASE_URL", database_url)
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to spawn child process");

        let mut counter: i32 = 0;
        let client_res = loop {
            counter += 1;

            let client_res =
                DepositServiceClient::connect(format!("http://{}", grpc_address.clone())).await;
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
async fn test_deposit() {
    let mut ctx = TestContext::setup().await;
    
    grpc::test_grpc_version(&mut ctx).await;
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
