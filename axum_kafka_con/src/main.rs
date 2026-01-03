use tokio::{signal, sync::watch::channel};

use crate::kafka_consumer::{ItemDemoDto, UnitDemoDto, consume_kafka};

mod kafka_consumer;

#[tokio::main]
async fn main() {
    let (sender, receiver) = channel(false);

    let kl1 = tokio::spawn(consume_kafka::<ItemDemoDto>(
        "axum1KafkaGroup",
        "axum1-topic",
        receiver.clone(),
    ));

    let kl2 = tokio::spawn(consume_kafka::<UnitDemoDto>(
        "axum2KafkaGroup",
        "axum2-topic",
        receiver.clone(),
    ));

    // loop {
    //     tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    // }

    shutdown_signal().await;
    println!("Shutdown signal received");

    let _ = sender.send(true);

    let _ = kl1.await;
    let _ = kl2.await;

    println!("App exited cleanly");
}

async fn shutdown_signal() {
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
        _ = ctrl_c => println!("Ctrl+C received"),
        _ = terminate => println!("SIGTERM received"),
    }
}
