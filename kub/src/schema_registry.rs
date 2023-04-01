
pub fn is_ready(timeout_ms: &u64) -> bool {

    println!("Checking if Kafka Schema Registry is ready {:?}", timeout_ms);

    true
}

pub fn add_schemas(schemas: &Vec<String>, timeout_ms: &u64) -> bool {
    println!("Checking if Kafka Schema Registry add schema {:?} {:?}", schemas, timeout_ms);
    true
}