# Polyglot Microservices (Spring Boot + Axum)- (Running)

This project is a **polyglot microservice example** built using:

- **Java – Spring Boot**
- **Rust – Axum**

The goal is to explore **cross-language microservice communication** while keeping the architecture simple at an early stage.

---

## Services

- school: Java (Spring boot) - `REST Api` + `Kafka Producer`
- student: Rust (Axum) - `REST Api` + `Kafka Producer`
- spring-kafka-con: Java (Spring boot) - `Kafka consumer`
- axum_kafka_con: Rust (Axum) - `Kafka consumer`

---

## Service Flow

- `school` service exposes REST Apis.
- `student` service exposes REST Apis.
- `school` service and `student` service talkes to each other via RestClient (Java) and Reqwest (Rust) library.
- `student` service is a `Kafka producer` which is consumed by `spring-kafka-con` service `Kafka consumer`

---

At this point, the project focuses only on **basic internal service-to-service communication**. No API gateway, service discovery, or advanced infrastructure is implemented yet.

---

## Project Status

**Stage 2 / Primary Level**

- Microservices communicate directly via HTTP
- Kafka support added
- No gateway or discovery mechanism
- No authentication or authorization
- No container orchestration
- No centralized configuration

The architecture is intentionally minimal to keep the focus on **interoperability between Java and Rust services**.

---

## Tech Stack

### `school` Service
- Java
- Spring Boot
- Spring `RestClient` (for internal HTTP calls)

### `student` Service
- Rust
- Axum (web framework)
- Reqwest (HTTP client for internal communication)
- Tokio (async runtime)

### Kafka + Zookeeper
- Kafka: confluentinc/cp-kafka:7.4.0
- Zookeeper: confluentinc/cp-zookeeper:7.4.0

### `spring-kafka-con` Service
- Java
- Spring Boot
- Spring Kafka

### `axum_kafka_con` Service
- Rust
- Tokio (async runtime)
- rdkafka
---

## Communication

Internal communication between services is implemented using:

- **Spring Boot → Rust Axum**: Spring `RestClient`
- **Rust Axum → Spring Boot**: `reqwest`

All communication is currently **synchronous HTTP-based**, without retries, circuit breakers, or timeouts configured.

---


## Running the Services

### Spring Boot Service

```bash
cd school
./mvnw spring-boot:run
```

```bash
cd spring-kafka-con
./mvnw spring-boot:run
```
### Rust Axum Service
```bash
cd student
cargo run
```

```bash
cd axum_kafka_con
cargo run
```

Make sure both services are running so they can communicate with each other.

## Configuration

Currently, service endpoints are configured using static URLs.

Example:

- `school` service calls `student` service at http://localhost:8090
- `student` service calls `school` service at http://localhost:8080

This will evolve in future iterations.

## Planned Improvements
The project is expected to evolve with:

- API Gateway (Spring Cloud Gateway / Envoy / Traefik)
- Service Discovery (Eureka / Consul)
- Centralized configuration
- Resilience patterns (timeouts, retries, circuit breakers)
- Observability (logging, metrics, tracing)
- Containerization (Docker)
- Orchestration (Kubernetes)

## Purpose

This repository is meant for:

- Learning polyglot microservice communication
- Comparing developer experience between Spring Boot and Axum
- Experimenting with Java ↔ Rust service interaction

It is not production-ready yet.