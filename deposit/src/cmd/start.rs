use anyhow::Result;
use clap::{
    Parser,
    command,
};
use common::{
    logger,
    logger::config::Config as LoggerConfig,
};
use ezex_deposit::{
    database::postgres::{
        config::Config as PostgresConfig,
        postgres::PostgresDB,
    },
    deposit::DepositHandler,
    event_bus::redis::RedisBus,
    grpc::{
        config::Config as GRPCConfig,
        server,
    },
    kms::{
        config::Config as KmsConfig,
        kms::DepositKms,
    },
};
use redis_stream_bus::config::Config as RedisConfig;
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
    #[command(flatten, next_help_heading = "kms")]
    pub kms_config: KmsConfig,
    #[command(flatten, next_help_heading = "logger")]
    pub logger_config: LoggerConfig,
}

impl StartArgs {
    pub async fn execute(&self) {
        if let Err(err) = self.execute_inner().await {
            eprintln!("Error Details: {:#}", err);
            std::process::exit(1);
        }
    }

    async fn execute_inner(&self) -> Result<()> {
        logger::init_logger(&self.logger_config);
        common::utils::exit_on_panic();

        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        let redis = RedisBus::new(&self.redis_config)?;
        let pq = PostgresDB::new(&self.postgres_config)?;
        let kms = DepositKms::new(&self.kms_config)?;

        let deposit = DepositHandler::new(Box::new(pq), Box::new(kms), Box::new(redis));

        let grpc_config = self.grpc_config.clone();
        let grpc_handle = task::spawn(async move {
            server::start_server(&grpc_config, deposit).await.unwrap();
        });


        log::info!("Deposit started...");
        while running.load(Ordering::SeqCst) {
            thread::sleep(std::time::Duration::from_secs(1));
        }

        grpc_handle.abort();

        Ok(())
    }
}
