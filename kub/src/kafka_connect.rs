use serde_json::Value;
use crate::utils::{create_http_client, get_kafka_connect_uri};

pub fn is_ready(timeout_ms: &u64) -> bool {
    println!("Checking if Kafka Connect is ready {:?}", timeout_ms);

    let client = create_http_client(*timeout_ms);

    let response = client.get(get_kafka_connect_uri())
        .send()
        .expect("Failed to send request");

    let success = response.status().is_success();
    let response_text = response.text().expect("Failed to read response");

    println!("\nResponse: {}", response_text);

    if success {
        println!("\nKafka Connect is ready");
        true
    } else{
        println!("\nKafka Connect is not ready");
        false
    }
}

pub fn get_connectors(timeout_ms: &u64) -> Vec<String> {
    println!("Listing Kafka Connect connectors {:?}", timeout_ms);

    let client = create_http_client(*timeout_ms);

    let response = client.get(&format!("{}/connectors", get_kafka_connect_uri()))
        .send()
        .expect("Failed to send request");

    let success = response.status().is_success();
    // let response_text = response.text().expect("Failed to read response");
    // println!("\nResponse: {}", response_text);

    let mut connector_names: Vec<String> = Vec::new();
    if success {
        println!("\nSuccessfully listed Kafka Connect connectors");

        let json_response: Value = response.json()
            .expect("Failed to parse response as JSON");
        // println!("\njson_response: {}", json_response);

        if let Some(array) = json_response.as_array() {
            for item in array {
                println!("{}", item);
                connector_names.push(item.to_string());
            }
        }
    } else{
        println!("\nFailed to list Kafka Connect connectors");
    }

    if connector_names.is_empty() {
        println!("No Connectors found");
    } else {
        for connector_name in &connector_names {
            println!("Connector: {}", connector_name);
        }
    }
    return connector_names
}

pub fn is_connector(connectors: &Vec<String>, timeout_ms: &u64) -> bool {
    println!("Checking if Kafka Connect connectors exist {:?} {:?}", connectors, timeout_ms);

    let vec = get_connectors(timeout_ms);

    // check if all connectors exist
    let mut missing_connectors: Vec<String> = Vec::new();
    for connector in connectors {
        if !vec.contains(&connector) {
            println!("Connector {} does not exist", connector);
            missing_connectors.push(connector.to_string());
        }
    }
    if missing_connectors.is_empty() {
        println!("All Connectors exist");
        true
    } else {
        for connector_name in &missing_connectors {
            println!("Missing Connector: {}", connector_name);
        }
        false
    }
}