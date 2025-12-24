# Polyglot Microservices (Spring Boot + Axum)- (Running)

This project is a **polyglot microservice example** built using:

- **Java – Spring Boot**
- **Rust – Axum**

The goal is to explore **cross-language microservice communication** while keeping the architecture simple at an early stage.

At this point, the project focuses only on **basic internal service-to-service communication**. No API gateway, service discovery, or advanced infrastructure is implemented yet.

---

## Project Status

**Early / Primary Level**

- Microservices communicate directly via HTTP
- No gateway or discovery mechanism
- No authentication or authorization
- No container orchestration
- No centralized configuration

The architecture is intentionally minimal to keep the focus on **interoperability between Java and Rust services**.

---

## Tech Stack

### Java Service
- Java
- Spring Boot
- Spring `RestClient` (for internal HTTP calls)

### Rust Service
- Rust
- Axum (web framework)
- Reqwest (HTTP client for internal communication)
- Tokio (async runtime)

---

## Communication

Internal communication between services is implemented using:

- **Spring Boot → Rust Axum**: Spring `RestClient`
- **Rust Axum → Spring Boot**: `reqwest`

All communication is currently **synchronous HTTP-based**, without retries, circuit breakers, or timeouts configured.

---

## Architecture (Current)

+------------------+ HTTP +------------------+
| Spring Boot | <--------------> | Rust Axum |
| Microservice |                 | Microservice |
+------------------+ +------------------+


- Services are started manually
- Service URLs are configured statically
- No load balancing or discovery


## Running the Services

### Spring Boot Service

```bash
cd java-service
./mvnw spring-boot:run
```
### Rust Axum Service
```bash
cd rust-service
cargo run
```

Make sure both services are running so they can communicate with each other.

## Configuration

Currently, service endpoints are configured using static URLs (e.g. localhost with fixed ports).

Example:

- Java service calls Rust service at http://localhost:8081
- Rust service calls Java service at http://localhost:8080

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

It is not production-ready.