use std::sync::Arc;

use axum::{
    Router,
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::{MethodRouter, get, get_service},
};
use consulrs::client::ConsulClient;
use sqlx::mysql::MySqlPoolOptions;
use tokio::{net::TcpListener, signal};
use tower_http::services::ServeDir;

use crate::{
    model::StudentService,
    web::kafka_producer::{AppState, KafkaProducer},
};

mod consul;
mod errors;
mod model;
mod web;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8090")
        .await
        .expect("cant start tcp listener");

    let consul_config = Arc::new(consul::config());
    let registration_config = consul_config.clone();

    consul::register_service(registration_config.clone()).await;

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect("mysql://root:admin@localhost:3306/std_db")
        .await
        .expect("error connecting db");

    sqlx::migrate!("./db_migrations")
        .run(&pool)
        .await
        .expect("db migration failed");

    let student_service = StudentService::new(pool);

    let producer = KafkaProducer::new("localhost:9092");
    let state = AppState {
        kafka: Arc::new(producer),
    };

    let all_routes = Router::new()
        .route("/health", get(health))
        .merge(web::student_routes::student_routes(student_service))
        .merge(web::kafka_message::get_kafka_route(state))
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(fallback_route());

    println!("server is listening on {:?}", listener.local_addr());

    axum::serve(listener, all_routes)
        .with_graceful_shutdown(shutdown_signal(consul_config.clone()))
        .await
        .expect("error serving the student service!!");
}

async fn health() -> impl IntoResponse {
    return (StatusCode::OK, "OK").into_response();
}

fn fallback_route() -> MethodRouter {
    async fn handle_404_not_found() -> (StatusCode, &'static str) {
        return (StatusCode::NOT_FOUND, "resource not found");
    }

    return get_service(ServeDir::new("/").not_found_service(handle_404_not_found.into_service()));
}

async fn main_response_mapper(response: Response) -> Response {
    println!("Response status {}", response.status());
    return response;
}

async fn shutdown_signal(consul_client: Arc<ConsulClient>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            println!(" ctrl+c called. Shutting down");
            consul::deregister_service(consul_client).await;
        },
        _ = terminate => {
            println!("terminated");
            consul::deregister_service(consul_client).await;
        },
    }
}
