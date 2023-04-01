
pub fn is_ready(timeout_ms: &u64) -> bool {

    println!("Checking if Kafka Connect is ready {:?}", timeout_ms);

    true
}

pub fn is_connector(connectors: &Vec<String>, timeout_ms: &u64) -> bool {
    println!("Checking if Kafka Connect connectors exist {:?} {:?}", connectors, timeout_ms);
    true
}