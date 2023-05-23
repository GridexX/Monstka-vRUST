
use monstka_lib::{config::MonstkaConfig, kafka_engine::{service::KafkaEngine}};
use anyhow::{Ok, Result};
use clap::Parser;
use log::{info, trace};


/// Monstka CLI options
#[derive(Parser)]
#[clap(
    version = "0.1",
    author = "GridexX - Arsène Fougerouse",
    about = "A Kafka converter in Rust"
)]
pub struct MonstkaOpts {
    /// Config file path
    #[clap(short, long, default_value = "/home/gridexx/Documents/Cours/Kafka/rust_kafka/config.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()>{

    // Initialize logger
    env_logger::init();

    info!("Starting Montska");

    // Parse CLI options
    let options = MonstkaOpts::parse();

    // Load config file
    let config = MonstkaConfig::load(options.config.as_str())?;

    trace!(
        "config file loaded successfully with content: {:#?}",
        config
    );


    let mut kafka_engine = KafkaEngine::new(config.uri, config.topic, config.group_id, config.partitions, config.random_word_generator_api_url)?;
    
    kafka_engine.consumes_messages_and_send_sentence().await?;

    Ok(())
}
