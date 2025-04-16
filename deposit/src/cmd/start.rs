use clap::{
    Args,
    Parser,
    command,
};
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
    database::postgres::{
        config::Config as PostgresConfig,
        postgres::PostgresDB,
    },
    deposit::Deposit,
    grpc::{
        config::Config as GRPCConfig,
        server,
    },
    kms,
    redis::RedisBus,
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

#[derive(Debug, Clone, Parser)]
pub struct StartArgs {
    #[command(flatten, next_help_heading = "grpc")]
    pub grpc_config: GRPCConfig,
    #[command(flatten, next_help_heading = "postgres")]
    pub postgres_config: PostgresConfig,
    #[command(flatten, next_help_heading = "redis")]
    pub redis_config: RedisConfig,
    #[command(flatten, next_help_heading = "logger")]
    pub logger_config: LoggerConfig,
}

impl StartArgs {
    pub async fn execute(&self) {
        logger::init_logger(&self.logger_config);
        common::utils::exit_on_panic();

        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        let mut redis = RedisClient::from_config(&self.redis_config).unwrap();
        let (mut read_tx, read_rx) = channel(100);
        let add_tx = redis.xadd_sender();
        let ack_tx = redis.xack_sender();

        let redis_handle = task::spawn(async move {
            let keys: Vec<&str> = vec![topic::deposit::address::Generate::name];
            redis.run(&keys, &mut read_tx).await.unwrap();
        });

        let grpc_config = self.grpc_config.clone();
        let pq = PostgresDB::from_config(&self.postgres_config).unwrap();
        let grpc_handle = task::spawn(async move {
            server::start_server(&grpc_config, pq).await.unwrap();
        });

        let pq = PostgresDB::from_config(&self.postgres_config).unwrap();
        let kms = kms::ezex::ezexKms::new().unwrap();
        let deposit = Deposit::new(pq, kms);
        let bus_handle = task::spawn(async move {
            let bus = RedisBus::new(deposit);
            bus.run(read_rx, add_tx, ack_tx).await;
        });

        log::info!("Deposit started...");
        while running.load(Ordering::SeqCst) {
            thread::sleep(std::time::Duration::from_secs(1));
        }

        redis_handle.abort();
        bus_handle.abort();
        grpc_handle.abort();
    }
}
