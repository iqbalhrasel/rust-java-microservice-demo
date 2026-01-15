use std::sync::Arc;

use consulrs::api::check::common::AgentServiceCheckBuilder;
use consulrs::api::service::requests::RegisterServiceRequest;
use consulrs::client::{ConsulClient, ConsulClientSettingsBuilder};
use consulrs::service::{self, deregister};
use local_ip_address::local_ip;

const SERVICE_NAME: &str = "student-service";
const SERVICE_PORT: u64 = 8090;

pub fn config() -> ConsulClient {
    let client = ConsulClient::new(
        ConsulClientSettingsBuilder::default()
            .address("http://127.0.0.1:8500")
            .build()
            .unwrap(),
    )
    .unwrap();

    return client;
}

pub async fn register_service(consul_client: Arc<ConsulClient>) {
    let host_ip = local_ip().expect("unable to detect host ip").to_string();
    let health_url = format!("http://{}:{}/health", host_ip, SERVICE_PORT);

    service::register(
        consul_client.as_ref(),
        "student",
        Some(
            RegisterServiceRequest::builder()
                .id(format!("{SERVICE_NAME}-{SERVICE_PORT}"))
                .name(SERVICE_NAME)
                .address(host_ip)
                .port(8090 as u32)
                .check(
                    AgentServiceCheckBuilder::default()
                        .name("health_check")
                        .interval("5s")
                        .http(health_url)
                        .status("passing")
                        .build()
                        .unwrap(),
                ),
        ),
    )
    .await
    .unwrap();

    println!("Registered '{}' with Consul Agent", SERVICE_NAME);
}

pub async fn deregister_service(consul_client: Arc<ConsulClient>) {
    deregister(
        consul_client.as_ref(),
        &format!("{SERVICE_NAME}-{SERVICE_PORT}"),
        None,
    )
    .await
    .expect("Failed to deregister service");

    println!("Deregistered '{}'", SERVICE_NAME);
}
