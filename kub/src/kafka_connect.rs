use std::time::Duration;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use crate::utils::{get_kafka_connect_uri};

pub fn is_ready(timeout_ms: &u64) -> bool {
    println!("Checking if Kafka Connect is ready {:?}", timeout_ms);
    let client = Client::builder()
        .timeout(Duration::from_millis(*timeout_ms))
        .build()
        .expect("Failed to build client");

    let response = client.get(get_kafka_connect_uri())
        .send()
        .expect("Failed to send request");

    match response.status() {
        StatusCode::OK => {
            println!("Kafka Connect is ready");
            true
        },
        _ => {
            println!("Kafka Connect is not ready");
            false
        },
    }
}

pub fn is_connector(connectors: &Vec<String>, timeout_ms: &u64) -> bool {
    println!("Checking if Kafka Connect connectors exist {:?} {:?}", connectors, timeout_ms);

    // let client = create_kafka_connect_client(timeout_ms);

    // for connector in connectors {
    //     let response = client.get(&format!("http://localhost:8083/connectors/{}", connector))
    //         .send();
    //
    //     if response.status().is_success() {
    //         println!("Connector {} exists", connector);
    //         true
    //     } else {
    //         println!("Connector {} does not exist", connector);
    //         false
    //     }
    // }
    false
}