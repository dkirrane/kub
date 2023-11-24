use std::collections::HashMap;
use std::time::Duration;

use rdkafka::admin::AdminOptions;
use rdkafka::consumer::Consumer;
use regex::Regex;

use crate::utils;

pub fn create(topic_names: &Vec<String>, partitions: u8, replicas: u8, config: &HashMap<String, String>, timeout_ms: &u64) -> bool {
    println!("Checking if Kafka Topics exist {:?} {:?} {:?} {:?} {:?}", topic_names, partitions, replicas, config, timeout_ms);

    true
}

pub fn list_topics(regex: &String, timeout_ms: &u64) {
    println!("Listing Kafka Topics");

    let consumer = utils::create_consumer();

    // Fetch metadata from the broker
    let metadata = consumer
        .fetch_metadata(None, Duration::from_millis(*timeout_ms))
        .expect("Failed to fetch metadata");

    // Print information about each topic
    for topic in metadata.topics() {
        let topic_name = topic.name();
        if regex.is_empty() {
            println!("Topic: {}", topic_name);
        } else if string_matches_regex(topic_name, regex) {
            println!("Topic: {}", topic_name);
        }
    }
}

pub fn delete_topics(regex: &String, timeout_ms: &u64) {
    println!("Listing Kafka Topics");

    let consumer = utils::create_consumer();

    // Fetch metadata from the broker
    let metadata = consumer
        .fetch_metadata(None, Duration::from_millis(*timeout_ms))
        .expect("Failed to fetch metadata");

    // Print information about each topic
    let mut topic_names: Vec<String> = Vec::new();
    for topic in metadata.topics() {
        let topic_name = topic.name();
        if string_matches_regex(topic_name, regex) {
            println!("Topic: {}", topic_name);
            topic_names.push(topic_name.to_string());
        }
    }
    if topic_names.is_empty() {
        println!("No topics found matching regex: {}", regex);
        return;
    }

    let topic_names_slice: Vec<&str> = topic_names.iter().map(AsRef::as_ref).collect();
    println!("{:?}", topic_names);
    println!("Deleting Topics:\n{:?}\n", topic_names_slice);

    delete_topic_list(&topic_names_slice);
}

fn delete_topic_list(topic_names: &[&str]) {
    let admin_client = utils::create_admin_client();
    let opts = AdminOptions::new().operation_timeout(Some(Duration::from_secs(5)));

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let res = rt.block_on(async {
        let results = admin_client
            .delete_topics(&topic_names, &opts)
            .await
            .expect("topic deletion failed");

        for result in results {
            match result {
                Ok(topic) => println!("Topic {} deleted successfully", topic),
                Err((topic, err)) => println!("Failed to delete topic {}: {}", topic, err),
            }
        }
    });
    println!("{:?}", res);
}

pub fn string_matches_regex(s: &str, regex: &str) -> bool {
    let re = Regex::new(regex).unwrap();
    re.is_match(s)
}

pub fn exists(topic_names: &Vec<String>, timeout_ms: &u64) -> bool {
    println!("Checking if Kafka Topics exist {:?} {:?}", topic_names, timeout_ms);

    true
}
