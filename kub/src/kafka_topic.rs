
use std::collections::HashMap;

pub fn create(topic_names: &Vec<String>, partitions: u8, replicas: u8, config: &HashMap<String, String>, timeout_ms: &u64) -> bool {

    println!("Checking if Kafka Topics exist {:?} {:?} {:?} {:?} {:?}", topic_names, partitions, replicas, config, timeout_ms);

    true
}

pub fn exists(topic_names: &Vec<String>, timeout_ms: &u64) -> bool {

    println!("Checking if Kafka Topics exist {:?} {:?}", topic_names, timeout_ms);

    true
}
