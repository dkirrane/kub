// use std::env;
// use std::process;
// use kub::Config;
// use kub::run;

use std::collections::HashMap;
// use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};

pub mod utils;
mod kafka;
mod kafka_connect;
mod schema_registry;
mod kafka_topic;

use crate::Commands::{KafkaConnectorsExists, KafkaConnectReady, KafkaReady, SchemaRegistryReady, SchemaRegistryRegisterSchemas, TopicDelete, TopicExists, TopicList};

// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(global = true, short = 't', long = "timeout", default_value = "10000", help = "Time (in ms) to wait for the service to be ready.")]
    timeout_ms: u64,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Checks if Kafka is ready")]
    KafkaReady(KafkaReadyArgs),

    #[command(name = "kc-ready", about = "Checks if Kafka Connect is ready")]
    KafkaConnectReady(KafkaConnectReadyArgs),

    #[command(name = "kc-connector-exists", about = "Checks if Kafka Connect Connector(s) exists")]
    KafkaConnectorsExists(KafkaConnectorsExistArgs),

    #[command(name = "sr-ready", about = "Checks if Kafka Schema Registry is ready")]
    SchemaRegistryReady(SchemaRegistryReadyArgs),

    #[command(name = "sr-register-schemas", about = "Register schema(s) in Schema Registry")]
    SchemaRegistryRegisterSchemas(SchemaRegistryRegisterSchemasArgs),

    #[command(name = "topic-exists", alias = "ensure-topic", about = "Checks if the supplied topic names exist")]
    TopicExists(TopicExistsArgs),

    #[command(name = "topic-list", alias = "topics-list", about = "List topics")]
    TopicList(TopicListArgs),

    #[command(name = "topic-delete", alias = "topics-delete", about = "Delete topics")]
    TopicDelete(TopicDeleteArgs),
}

#[derive(Args, Debug)]
struct KafkaReadyArgs {
    #[arg(short = 'b', long = "min-brokers", required = true, help = "Minimum number of brokers to wait for.")]
    min_broker_count: u8,
}

#[derive(Args, Debug)]
struct KafkaConnectReadyArgs {}

#[derive(Args, Debug)]
struct KafkaConnectorsExistArgs {
    #[arg(short = 'n', long = "names", required = true, help = "List of connector names to check for the existence of")]
    names: Vec<String>,
}

#[derive(Args, Debug)]
struct SchemaRegistryReadyArgs {}

#[derive(Args, Debug)]
struct SchemaRegistryRegisterSchemasArgs {
    #[arg(short = 'n', long = "schemas", required = true, help = "List of Avro schemas to be registered on Schema Registry")]
    schemas: Vec<String>,
}

#[derive(Args, Debug)]
struct TopicExistsArgs {
    #[arg(long = "topics", required = true, help = "Checks if the supplied topic names exist")]
    topics: Vec<String>,

    #[arg(short = 'c', long = "create-if-not-exists", default_value = "false", help = "Create the topics if they do not exist")]
    create_if_not_exists: bool,

    #[arg(short = 'p', long = "partitions", default_value = "1", help = "The number of partitions that the topic should have")]
    partitions: u8,

    #[arg(short = 'r', long = "replication_factor", default_value = "1", help = "The number of replicas each partition has")]
    replication_factor: u8,

    #[arg(long = "config", value_delimiter = ',', help = "The configuration to use on the new topic")]
    // configs: HashMap<String, String>,
    config: Option<Vec<String>>,
}


#[derive(Args, Debug)]
struct TopicListArgs {
    #[arg(long = "regex", default_value = "", help = "List topics matching the supplied regex")]
    regex: String
}

#[derive(Args, Debug)]
struct TopicDeleteArgs {
    #[arg(long = "regex", required = true, help = "Delete topics matching the supplied regex")]
    regex: String
}

fn main() {
    let cli = Cli::parse();
    println!("{cli:?}");
    let command = &cli.command;
    let timeout_ms = &cli.timeout_ms;

    match command {
        KafkaReady(args) => {
            println!("kafka-ready {:?}", args.min_broker_count);
            kafka::is_ready(args.min_broker_count, timeout_ms);
        }
        KafkaConnectReady(args) => {
            println!("kc-ready {:?}", args);
            kafka_connect::is_ready(timeout_ms);
        }
        KafkaConnectorsExists(args) => {
            println!("kc-connector-exists {:?}", args);
            kafka_connect::is_connector(&args.names, timeout_ms);
        }
        SchemaRegistryReady(args) => {
            println!("sr-ready {:?}", args);
            schema_registry::is_ready(timeout_ms);
        }
        SchemaRegistryRegisterSchemas(args) => {
            println!("sr-register-schemas {:?}", args);
            schema_registry::add_schemas(&args.schemas, timeout_ms);
        }
        TopicExists(args) => {
            println!("topic-exists {:?}", args);
            if args.create_if_not_exists {
                let configs = args.config.clone().unwrap();

                let config_map: HashMap<String, String> = configs
                    .iter()
                    .map(|s| s.split_once("=").unwrap())
                    .map(|(k, v)| (k.to_owned(), v.to_owned()))
                    .collect();

                println!("{:?}", config_map);

                kafka_topic::create(&args.topics, args.partitions, args.replication_factor, &config_map, timeout_ms);
            } else {
                kafka_topic::exists(&args.topics, timeout_ms);
            }
        }
        TopicList(args) => {
            println!("topic-list {:?}", args);
            kafka_topic::list_topics(&args.regex, timeout_ms);
        }
        TopicDelete(args) => {
            println!("topic-delete {:?}", args);
            kafka_topic::delete_topics(&args.regex, timeout_ms);
        }
    }
}