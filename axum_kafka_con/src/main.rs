use crate::kafka_consumer::{ItemDemoDto, UnitDemoDto, consume_kafka};

mod kafka_consumer;

#[tokio::main]
async fn main() {
    tokio::spawn(consume_kafka::<ItemDemoDto>(
        "axum1KafkaGroup",
        "axum1-topic",
    ));

    tokio::spawn(consume_kafka::<UnitDemoDto>(
        "axum2KafkaGroup",
        "axum2-topic",
    ));

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
