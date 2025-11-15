# 🦀 Rust API Template

**A production-ready template for building REST APIs with Rust + Swagger UI + Railway deployment**

FastAPI-like Rust web service with automatic OpenAPI documentation, interactive Swagger UI, and one-click Railway deployment.

## 🚀 Features

- **🦀 Rust Performance** - Built with Axum and Tokio for blazing fast APIs
- **📖 Automatic API Documentation** - Generate OpenAPI 3.0.3 specs from code annotations
- **🎯 Interactive Swagger UI** - Explore and test APIs at `/docs`
- **🚂 Railway Ready** - One-click deployment with Dockerfile and railway.toml
- **⚡ FastAPI-like Experience** - Simple decorators for endpoint documentation
- **🔒 Type Safety** - Full compile-time type checking with automatic schema generation
- **📦 Template Structure** - Copy and customize for your own projects

## 📦 Dependencies

```toml
[dependencies]
anyhow = "1.0"
axum = "0.7"
qdrant-client = "1.15.0"
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
tower = "0.4"
utoipa = { version = "4", features = ["axum_extras"] }
utoipa-swagger-ui = { version = "7", features = ["axum"] }
```

## 🏃‍♂️ Quick Start

### Local Development
```bash
# Clone this template
git clone <your-repo>
cd seo-engine

# Run the server
cargo run

# Open your browser
# API Documentation: http://127.0.0.1:3000/docs
# Health Check: http://127.0.0.1:3000/health
```

### 🚂 Deploy to Railway

1. **Fork this template repository**
2. **Connect to Railway:**
   - Go to [railway.app](https://railway.app)
   - Click "Deploy from GitHub repo"
   - Select your forked repository
3. **Automatic deployment** - Railway will detect the `railway.toml` and `Dockerfile`
4. **Access your API** at the provided Railway URL

That's it! Your Rust API with Swagger UI is now live on Railway.

## 📝 Adding New Endpoints

### 1. Define Response Schema

```rust
#[derive(Serialize, utoipa::ToSchema)]
struct UserResponse {
    id: u32,
    name: String,
    email: String,
}
```

### 2. Create Documented Endpoint

```rust
#[utoipa::path(
    get,
    path = "/users/{id}",
    params(
        ("id" = u32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 404, description = "User not found")
    ),
    tag = "Users"
)]
async fn get_user(Path(id): Path<u32>) -> Result<Json<UserResponse>, StatusCode> {
    // Implementation here
}
```

### 3. Register in OpenAPI

```rust
#[derive(OpenApi)]
#[openapi(
    paths(
        health_handler,
        get_user,  // Add your new endpoint
    ),
    components(schemas(
        HealthResponse,
        UserResponse,  // Add your new schema
    )),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Users", description = "User management")  // Add new tag
    ),
    info(
        title = "SEO Engine API",
        version = "1.0.0",
        description = "Rust + Qdrant SEO engine"
    )
)]
struct ApiDoc;
```

### 4. Add Route

```rust
let app = Router::new()
    .route("/health", get(health_handler))
    .route("/users/:id", get(get_user))  // Add your route
    .merge(SwaggerUi::new("/docs")
        .url("/api-docs/openapi.json", ApiDoc::openapi()));
```

## 📖 Documentation Features

### Request/Response Examples

```rust
#[utoipa::path(
    post,
    path = "/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = UserResponse,
            example = json!({"id": 1, "name": "John", "email": "john@example.com"})
        )
    )
)]
```

### Path Parameters

```rust
#[utoipa::path(
    get,
    path = "/users/{id}/posts/{post_id}",
    params(
        ("id" = u32, Path, description = "User ID"),
        ("post_id" = u32, Path, description = "Post ID")
    )
)]
```

### Query Parameters

```rust
#[derive(Deserialize, utoipa::IntoParams)]
struct SearchQuery {
    #[param(example = "rust")]
    q: Option<String>,
    #[param(minimum = 1, maximum = 100, default = 10)]
    limit: Option<u32>,
}

#[utoipa::path(
    get,
    path = "/search",
    params(SearchQuery),
    responses(...)
)]
```

## 🔧 Common Patterns

### Error Handling

```rust
#[derive(Serialize, utoipa::ToSchema)]
struct ErrorResponse {
    error: String,
    details: Option<String>,
}

#[utoipa::path(
    responses(
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 500, description = "Internal Server Error", body = ErrorResponse)
    )
)]
```

### Authentication Headers

```rust
#[utoipa::path(
    security(
        ("api_key" = [])
    ),
    responses(...)
)]
```

## 🌐 API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/docs` | GET | Interactive API documentation |

## 🛠️ Development

```bash
# Build
cargo build

# Run with auto-reload (install cargo-watch)
cargo install cargo-watch
cargo watch -x run

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## 📚 Resources

- [utoipa Documentation](https://docs.rs/utoipa/)
- [Axum Documentation](https://docs.rs/axum/)
- [OpenAPI 3.0.3 Specification](https://swagger.io/specification/)
- [Swagger UI](https://swagger.io/tools/swagger-ui/)

## 🎯 Why This Template?

| Feature | FastAPI (Python) | This Template (Rust) |
|---------|------------------|----------------------|
| Auto docs | ✅ `/docs` | ✅ `/docs` |
| Type hints | ✅ Python types | ✅ Rust types |
| Performance | ~1000 req/s | ~10,000+ req/s |
| Memory usage | ~50MB | ~5MB |
| Compile-time safety | ❌ | ✅ |
| Railway deployment | Manual setup | ✅ One-click |
| Production ready | Requires config | ✅ Out of the box |

## 🚀 Getting Started with This Template

1. **Fork this repository** or use as a template on GitHub
2. **Customize** the API name and endpoints in `src/main.rs`
3. **Add your business logic** following the documented patterns
4. **Deploy to Railway** with zero configuration
5. **Share your API** with the world!

---

**🚂 Built for Railway • 🦀 Powered by Rust • 📖 Documented with Swagger**