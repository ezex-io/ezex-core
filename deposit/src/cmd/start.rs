use clap::Args;
use common::{
    logger,
    logger::config::Config as LoggerConfig,
    redis::redis_bus::{
        RedisBusTrait,
        RedisClient,
        RedisConfig,
        StreamBus,
    },
    topic,
};
use ezex_deposit::{
    api::grpc::{
        config::Config as GRPCConfig,
        server,
    },
    config::Config as VaultConfig,
    database::postgres::{
        config::Config as PostgresConfig,
        postgres::PostgresDB,
    },
    deposit::Deposit,
    redis_bus::RedisBus,
};
use futures::channel::mpsc::channel;
use std::{
    sync::{
        Arc,
        atomic::{
            AtomicBool,
            Ordering,
        },
    },
    thread,
};
use tokio::task;

#[derive(Debug, Args)]
pub struct StartCmd {
    pub grpc_config: GRPCConfig,
    pub postgres_config: PostgresConfig,
    pub redis_config: RedisConfig,
    pub vault_config: VaultConfig,
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
            redis.run(&keys, &mut read_tx).await;
        });

        let grpc_config = self.grpc_config;
        let pq = PostgresDB::from_config(&self.postgres_config).unwrap();
        let grpc_handle = task::spawn(async move {
            server::start_server(&grpc_config, pq).await.unwrap();
        });

        let pq = PostgresDB::from_config(&self.postgres_config).unwrap();

        let bus_handle = task::spawn(async move {
            let bus = RedisBus::new(deposit);
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
