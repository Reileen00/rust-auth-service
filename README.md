
# `rust-auth-service`

> **High-Performance Authentication Microservice in Rust**

A production-grade authentication service built with **Rust**, designed for **security, performance, and scalability**.
This project demonstrates real-world backend engineering skills: database design, secure authentication flows, middleware, and infrastructure integration.

## 🧰 Tech Stack

| Category         | Technology       |
| ---------------- | ---------------- |
| API Framework    | Axum             |
| Database         | PostgreSQL       |
| Async Runtime    | Tokio            |
| ORM              | SQLx             |
| Caching          | Redis            |
| Auth             | JWT              |
| Password Hashing | Argon2           |
| Rate Limiting    | Tower Governor   |
| API Docs         | OpenAPI (utoipa) |
| Containerization | Docker           |

---

##  Architecture Overview

```
Client
  ↓
Axum HTTP API
  ↓
JWT Auth Middleware
  ↓
PostgreSQL (Users)
  ↓
Redis (Refresh Tokens / Sessions)
```

---

##  Project Structure

```
rust-auth-service/
├── src/
│   ├── main.rs            # App entrypoint
│   ├── config.rs          # Environment configuration
│   ├── db.rs              # PostgreSQL connection pool
│   ├── crypto.rs          # Password hashing (Argon2)
│   ├── jwt.rs             # JWT creation & validation
│   ├── redis.rs           # Redis session helpers
│   ├── models.rs          # Request & DB models
│   ├── openapi.rs         # OpenAPI schema
│   ├── routes/
│   │   ├── auth.rs        # Register / Login / Refresh
│   │   ├── user.rs        # Authenticated user routes
│   ├── middleware/
│   │   ├── auth.rs        # JWT auth middleware
│   │   ├── rate_limit.rs  # Request throttling
│
├── migrations/            # SQLx migrations
├── benches/               # Criterion benchmarks
├── docker-compose.yml
├── .env
```

---

##  API Endpoints

| Method | Endpoint    | Description                    |
| ------ | ----------- | ------------------------------ |
| POST   | `/register` | Create a new user account      |
| POST   | `/login`    | Authenticate and receive JWT   |
| GET    | `/me`       | Get current authenticated user |
| POST   | `/refresh`  | Refresh access token           |
| POST   | `/logout`   | Revoke refresh token           |

---

##  Authentication Flow

1. **Register**

   * Password hashed with Argon2
   * Stored securely in PostgreSQL

2. **Login**

   * Password verified
   * Access token (JWT) issued
   * Refresh token stored in Redis

3. **Authenticated Requests**

   * JWT validated via middleware
   * User context injected into request

4. **Logout**

   * Refresh token revoked from Redis

---

##  Security Considerations

* **Argon2** for password hashing (memory-hard)
* **JWT expiration** enforced
* **Rate limiting** on auth endpoints
* **No plaintext passwords** stored or logged
* **Environment-based secrets**
* **Stateless access tokens** + server-side refresh revocation

---

##  Getting Started

### Prerequisites

* Rust (stable)
* Docker + Docker Compose

---

###  Clone the repo

```bash
git clone https://github.com/yourusername/rust-auth-service.git
cd rust-auth-service
```

---

###  Start dependencies

```bash
docker-compose up -d
```

---

###  Configure environment

```env
DATABASE_URL=postgres://postgres:postgres@localhost:5432/auth
REDIS_URL=redis://127.0.0.1/
JWT_SECRET=supersecretkey
```

---

###  Run migrations

```bash
sqlx migrate run
```

---

###  Run the server

```bash
cargo run
```

Server runs at:

```
http://localhost:3000
```

---

```bash
cargo bench
```

---



## Future Improvements

* Role-based access control (RBAC)
* OAuth2 / OpenID Connect
* Distributed rate limiting
* Observability (metrics + tracing)
* Kubernetes deployment manifests

---
