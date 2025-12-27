use axum::{Json, Router, extract::State, routing::post};
use serde::{Deserialize, Serialize};

use crate::web::kafka_producer::AppState;

pub fn get_kafka_route(state: AppState) -> Router {
    return Router::new()
        .route("/produce", post(produce_message))
        .with_state(state);
}

#[derive(Deserialize, Serialize, Debug)]
struct DataDemoDto {
    name: String,
    point: i32,
}

async fn produce_message(
    State(state): State<AppState>,
    Json(dto): Json<DataDemoDto>,
) -> Result<&'static str, &'static str> {
    println!("-->> dataDemoDto: {:?}", &dto);

    let payload = serde_json::to_string(&dto).unwrap();
    state
        .kafka
        .send("kafka-topic-spring", "demoDataDto", &payload)
        .await
        .map_err(|_| "Kafka send failed")?;

    Ok("Message sent to Kafka")
}
