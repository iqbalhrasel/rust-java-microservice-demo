use std::fmt::Debug;

use rdkafka::{
    ClientConfig, Message,
    consumer::{Consumer, StreamConsumer},
};
use serde::{Deserialize, de::DeserializeOwned};

#[derive(Deserialize, Debug)]
pub struct ItemDemoDto {
    pub title: String,
    pub score: i32,
}

#[derive(Deserialize, Debug)]
pub struct UnitDemoDto {
    pub heading: String,
    pub count: i32,
}

pub async fn consume_kafka<T>(group_id: impl Into<String>, topic: impl Into<String>)
where
    T: Debug + DeserializeOwned,
{
    let group_id = group_id.into();
    let topic = topic.into();
    println!("group id : {}", &group_id);

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("group.id", &group_id)
        .set("auto.offset.reset", "earliest")
        .set("socket.timeout.ms", "4000")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&[&topic]).expect("Can't subscribe");

    loop {
        match consumer.recv().await {
            Ok(message) => {
                if let Some(payload) = message.payload() {
                    match serde_json::from_slice::<T>(payload) {
                        Ok(dto) => {
                            println!("Received message: {:?}", dto);
                        }
                        Err(err) => {
                            eprintln!("JSON deserialization error: {:?}", err);
                        }
                    }
                } else {
                    println!("Received message with empty payload");
                }
            }
            Err(e) => eprintln!("Kafka error: {:?}", e),
        }
    }
}
