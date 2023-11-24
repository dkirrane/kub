use std::fs::read_to_string;
use std::time::Duration;

use rdkafka::admin::AdminClient;
use rdkafka::client::DefaultClientContext;
use rdkafka::ClientConfig;
use rdkafka::consumer::BaseConsumer;
use reqwest::blocking::Client;

const SECRET_CSI_DIR: &str = "/mnt/secrets-store";

pub fn create_http_client(timeout_ms: u64) -> Client {
    Client::builder()
        .timeout(Duration::from_millis(timeout_ms))
        .build()
        .expect("Failed to build client")
}

// pub(crate) fn create_config() -> &mut ClientConfig {
//     let client_config = ClientConfig::new()
//         .set("bootstrap.servers", get_kafka_service_uri())
//         .set("security.protocol", "SASL_SSL")
//         .set("sasl.mechanisms", "PLAIN")
//         .set("sasl.username", get_kafka_admin_user())
//         .set("sasl.password", get_kafka_admin_password())
//         .set("ssl.ca.location", get_kafka_ca_cert_path())
//         .set("client.id", "kub")
//         .set("debug", "all");
//     client_config
// }

pub(crate) fn create_admin_client() -> AdminClient<DefaultClientContext> {
    let bootstrap_servers = get_kafka_service_uri();
    let admin_client: AdminClient<_> = ClientConfig::new()
        .set("bootstrap.servers", &bootstrap_servers)
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", "PLAIN")
        .set("sasl.username", get_kafka_admin_user())
        .set("sasl.password", get_kafka_admin_password())
        .set("ssl.ca.location", get_kafka_ca_cert_path())
        .set("client.id", "kub")
        .set("debug", "all")
        .create()
        .expect("AdminClient creation failed");
    println!("AdminClient created for {}", &bootstrap_servers);
    admin_client
}

pub(crate) fn create_consumer() -> BaseConsumer {
    let bootstrap_servers = get_kafka_service_uri();
    let consumer = ClientConfig::new()
        .set("bootstrap.servers", &bootstrap_servers)
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanisms", "PLAIN")
        .set("sasl.username", get_kafka_admin_user())
        .set("sasl.password", get_kafka_admin_password())
        .set("ssl.ca.location", get_kafka_ca_cert_path())
        .set("client.id", "kub")
        .set("debug", "all")
        .create()
        .expect("Failed to create consumer");
    println!("Consumer created for {}", &bootstrap_servers);
    consumer
}

fn get_secret(secret: &str) -> String {
    read_to_string(format!("{}/{}", SECRET_CSI_DIR, secret))
        .expect(&format!("Failed to read {} secret", secret))
}

fn get_kafka_service_uri() -> String {
    get_secret("kafka-service-uri")
}

fn get_kafka_admin_user() -> String {
    get_secret("kafka-admin-username")
}

fn get_kafka_admin_password() -> String {
    get_secret("kafka-admin-password")
}

fn get_kafka_ca_cert_path() -> String {
    format!("{}/{}", SECRET_CSI_DIR, "kafka-ca-cert")
}

pub(crate) fn get_kafka_schemaregistry_uri() -> String {
    get_secret("kafka-schemaregistry-uri")
}

pub(crate) fn get_kafka_connect_uri() -> String {
    get_secret("kafka-connect-uri")
}

