use common::logger::config::Config as LoggerConfig;
use common::redis_bus::{RedisBusTrait, RedisClient, RedisConfig, StreamBus};
use common::{logger, topic};
use deposit_vault::api::grpc::config::Config as GRPCConfig;
use deposit_vault::api::grpc::server;
use deposit_vault::config::Config as VaultConfig;
use deposit_vault::database::postgres::config::Config as PostgresConfig;
use deposit_vault::database::postgres::postgres::PostgresDB;
use deposit_vault::redis_bus::RedisBus;
use deposit_vault::vault::Vault;
use futures::channel::mpsc::channel;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use structopt::StructOpt;
use tokio::task;

#[derive(Debug, StructOpt)]
#[structopt(name = "start")]
pub struct StartCmd {
    #[structopt(flatten)]
    pub grpc_config: GRPCConfig,
    #[structopt(flatten)]
    pub postgres_config: PostgresConfig,
    #[structopt(flatten)]
    pub redis_config: RedisConfig,
    #[structopt(flatten)]
    pub vault_config: VaultConfig,
    #[structopt(flatten)]
    pub logger_config: LoggerConfig,
}

impl StartCmd {
    pub async fn execute(&self) {
        logger::init_logger(&self.logger_config);
        common::utils::exit_on_panic();

        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        let redis = RedisClient::from_config(&self.redis_config).unwrap();
        let (read_tx, read_rx) = channel(100);
        let add_tx = redis.xadd_sender();
        let ack_tx = redis.xack_sender();

        let redis_handle = task::spawn(async move {
            let keys: Vec<&str> = vec![topic::deposit::address::Generate::name];
            redis.run(&keys, read_tx).await;
        });

        let grpc_config = self.grpc_config.clone();
        let pq = PostgresDB::from_config(&self.postgres_config).unwrap();
        let grpc_handle = task::spawn(async move {
            server::start_server(&grpc_config, pq).await.unwrap();
        });

        let pq = PostgresDB::from_config(&self.postgres_config).unwrap();

        let bus_handle = task::spawn(async move {
            let bus = RedisBus::new(vault);
            bus.run(read_rx, add_tx, ack_tx).await;
        });

        log::info!("Vault started...");
        while running.load(Ordering::SeqCst) {
            thread::sleep(std::time::Duration::from_secs(1));
        }

        redis_handle.abort();
        bus_handle.abort();
        grpc_handle.abort();
    }
}
