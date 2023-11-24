use crate::utils::{create_http_client, get_kafka_schemaregistry_uri};

pub fn is_ready(timeout_ms: &u64) -> bool {
    println!("Checking if Kafka Schema Registry is ready - timeout={:?}ms", timeout_ms);

    let client = create_http_client(*timeout_ms);

    let response = client.get(get_kafka_schemaregistry_uri() + "/config")
        .send()
        .expect("Failed to send request");

    let success = response.status().is_success();
    let response_text = response.text().expect("Failed to read response");

    println!("\nResponse: {}", response_text);

    if success && response_text.contains("compatibilityLevel") {
        println!("\nKafka Schema Registry is ready");
        true
    } else{
        println!("\nKafka Schema Registry is not ready");
        false
    }
}

pub fn add_schemas(schemas: &Vec<String>, timeout_ms: &u64) -> bool {
    println!("Adding schemas to Kafka Schema Registry: {:?} {:?}", schemas, timeout_ms);

    // let client = create_http_client(*timeout_ms);

    // todo - https://github.com/gklijs/schema_registry_converter/discussions/102

    // for schema in schemas {
    //     let response = client.post(get_kafka_schemaregistry_uri() + "/subjects/" + schema + "/versions")
    //         .json(&serde_json::json!({
    //             "schema": "{\"type\": \"string\"}" // Replace with your actual schema
    //         }))
    //         .send()
    //         .expect("Failed to send request");
    //
    //     match response.status() {
    //         StatusCode::OK => {
    //             println!("Schema {} added successfully", schema);
    //         },
    //         _ => {
    //             println!("Failed to add schema {}", schema);
    //             return false;
    //         },
    //     }
    // }

    true
}
