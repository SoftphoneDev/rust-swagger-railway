use axum::{routing::get, Router, Json};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Serialize, utoipa::ToSchema)]
struct HealthResponse {
    status: String,
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check endpoint", body = HealthResponse)
    ),
    tag = "Health"
)]
async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "OK".into(),
    })
}

#[derive(OpenApi)]
#[openapi(
    paths(health_handler),
    components(schemas(HealthResponse)),
    tags(
        (name = "Health", description = "Health check endpoints")
    ),
    info(
        title = "SEO Engine API",
        version = "1.0.0",
        description = "Rust + Qdrant SEO engine"
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_handler))
        .merge(SwaggerUi::new("/docs")
            .url("/api-docs/openapi.json", ApiDoc::openapi()));

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid number");

    let addr = format!("0.0.0.0:{}", port);

    println!("🚀 Server running at http://{}", addr);
    println!("📖 API endpoints:");
    println!("   - Health: http://{}/health", addr);
    println!("   - API docs: http://{}/docs", addr);

    axum::serve(TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}
