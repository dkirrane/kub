use std::time::Duration;
use rdkafka::consumer::Consumer;
use crate::utils;

pub fn is_ready(min_broker_count: u8, timeout_ms: &u64) -> bool {

    println!("Checking if Kafka is ready {:?} {:?}", min_broker_count, timeout_ms);

    let consumer = utils::create_consumer();

    // Fetch metadata from the broker
    let metadata = consumer
        .fetch_metadata(None, Duration::from_millis(1000))
        .expect("Failed to fetch metadata");

    let broker_count = metadata.brokers().len();
    let topic_count = metadata.topics().len();
    println!("\nCluster information:");
    println!("  Broker count: {}", broker_count);
    println!("  Topics count: {}", topic_count);
    println!("  Metadata broker name: {}", metadata.orig_broker_name());
    println!("  Metadata broker id: {}\n", metadata.orig_broker_id());

    println!("Brokers:");
    for broker in metadata.brokers() {
        println!(
            "  Id: {}  Host: {}:{}  ",
            broker.id(),
            broker.host(),
            broker.port()
        );
    }

    println!("\nBrokers: min_broker_count={}, broker_count={}", min_broker_count, broker_count);
    if broker_count >= min_broker_count as usize {
        println!("\nKafka is ready");
        true
    } else {
        println!("\nKafka is not ready");
        false
    }
}
