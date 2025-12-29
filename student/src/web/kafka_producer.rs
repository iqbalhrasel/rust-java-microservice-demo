use std::{sync::Arc, time::Duration};

use rdkafka::{
    ClientConfig,
    message::{Header, OwnedHeaders},
    producer::{FutureProducer, FutureRecord},
};

#[derive(Clone)]
pub struct KafkaProducer {
    producer: FutureProducer,
}

#[derive(Clone)]
pub struct AppState {
    pub kafka: Arc<KafkaProducer>,
}

impl KafkaProducer {
    pub fn new(brokers: &str) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Failed to create Kafka producer");

        Self { producer }
    }

    pub async fn send(&self, topic: &str, key: &str, payload: &str) -> Result<(), String> {
        let headers = OwnedHeaders::new().insert(Header {
            key: "__TypeId__",
            value: Some(key),
        });

        self.producer
            .send(
                FutureRecord::to(topic)
                    .key(key)
                    .payload(payload)
                    .headers(headers),
                Duration::from_secs(0),
            )
            .await
            .map_err(|(e, _)| e.to_string())?;

        Ok(())
    }
}
