use axum::{Json, Router, extract::State, routing::post};
use serde::{Deserialize, Serialize};

use crate::web::kafka_producer::AppState;

pub fn get_kafka_route(state: AppState) -> Router {
    return Router::new()
        .route("/produce1", post(produce_message1))
        .route("/produce2", post(produce_message2))
        .with_state(state);
}

#[derive(Deserialize, Serialize, Debug)]
struct DataDemoDto {
    name: String,
    point: i32,
}

#[derive(Deserialize, Serialize, Debug)]
struct ElementDemoDto {
    element: String,
    age: i32,
}

async fn produce_message1(
    State(state): State<AppState>,
    Json(dto): Json<DataDemoDto>,
) -> Result<&'static str, &'static str> {
    println!("-->> dataDemoDto: {:?}", &dto);

    let payload = serde_json::to_string(&dto).unwrap();
    state
        .kafka
        .send("kafka1-topic-spring", "dataDemoDto", &payload)
        .await
        .map_err(|_| "Kafka send failed")?;

    Ok("Message sent to Kafka")
}

async fn produce_message2(
    State(state): State<AppState>,
    Json(dto): Json<ElementDemoDto>,
) -> Result<&'static str, &'static str> {
    println!("-->> elementDemoDto: {:?}", &dto);

    let payload = serde_json::to_string(&dto).unwrap();
    state
        .kafka
        .send("kafka2-topic-spring", "elementDemoDto", &payload)
        .await
        .map_err(|_| "Kafka send failed")?;

    Ok("Message sent to Kafka")
}
