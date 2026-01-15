use std::sync::Arc;

use consulrs::api::check::common::AgentServiceCheckBuilder;
use consulrs::api::service::requests::RegisterServiceRequest;
use consulrs::client::{ConsulClient, ConsulClientSettingsBuilder};
use consulrs::service::{self, deregister};
use local_ip_address::local_ip;
use reqwest::Client;
use serde::Deserialize;

use crate::errors::Error;

const SERVICE_NAME: &str = "student";
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
        SERVICE_NAME,
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

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct ConsulService {
    ServiceAddress: String,
    ServicePort: u16,
}

pub async fn get_school_service_address() -> Result<String, Error> {
    let client = Client::new();
    let res = client
        .get("http://127.0.0.1:8500/v1/catalog/service/school")
        .send()
        .await
        .map_err(|_| Error::InternalServerError)?;

    let services: Vec<ConsulService> = res.json().await.map_err(|_| Error::InternalServerError)?;

    if services.is_empty() {
        return Err(Error::SchoolNotFoundError);
    }

    let s = &services[0];
    Ok(format!("http://{}:{}", s.ServiceAddress, s.ServicePort))
}
