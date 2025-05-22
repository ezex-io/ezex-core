use anyhow::Result;
use clap::{Args, Parser, command};
use common::{
    logger::{self, config::Config as LoggerConfig},
    register_config,
};
use ezex_deposit::{
    database::postgres::{config::Config as PostgresConfig, postgres::PostgresDB},
    deposit::DepositHandler,
    event_bus::redis::RedisBus,
    grpc::{config::Config as GRPCConfig, server},
    kms::{config::Config as KmsConfig, kms::DepositKms},
};
use procedural::EnvPrefix;
use redis_stream_bus::config::Config as RedisStreamConfig;
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
};
use tokio::task;

#[derive(Debug, Clone, Args, EnvPrefix)]
#[env_prefix = "EZEX_DEPOSIT"]
#[group(id = "redis")]
pub struct RedisConfig {
    #[arg(long = "redis-connection-string", env = "REDIS_CONNECTION_STRING")]
    pub connection_string: String,
    #[arg(long = "redis-group-name", env = "REDIS_GROUP_NAME")]
    pub group_name: String,
    #[arg(long = "redis-consumer-name", env = "REDIS_CONSUMER_NAME")]
    pub consumer_name: String,
}

// Register this config
register_config!(RedisConfig);

// Conversion to the third-party type
impl From<RedisConfig> for RedisStreamConfig {
    fn from(config: RedisConfig) -> Self {
        RedisStreamConfig {
            connection_string: config.connection_string,
            consumer_name: config.consumer_name,
            group_name: config.group_name,
        }
    }
}

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
            eprintln!("Error Details: {err:#}");
            std::process::exit(1);
        }
    }

    async fn execute_inner(&self) -> Result<()> {
        // Initialize all configs (registers them in the registry)
        common::config_registry::init_all_configs();

        // Apply all prefixes (calls prepend_envs() for all registered configs)
        common::config_registry::global_registry()
            .lock()
            .unwrap()
            .apply_all_prefixes();

        logger::init_logger(&self.logger_config);
        common::utils::exit_on_panic();

        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        let redis_config: redis_stream_bus::config::Config = self.redis_config.clone().into();
        let redis = RedisBus::new(&redis_config)?;
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
