use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
    
};
use axum::response::Html;
use serde_json::Value;
use std::{sync::{Arc, Mutex}};
use tokio::net::TcpListener;
use tower_http::services::fs::ServeDir;
use tower_http::trace::{TraceLayer, DefaultMakeSpan, DefaultOnResponse};

//use tracing::{info, Level};
use tracing_subscriber;
use askama::Template;
use tracing::{Level};


#[derive(Template)]
#[template(path = "dashboard.html")]
struct DashboardTemplate<'a> {
    context: &'a String,
}


#[derive(Default)]
struct AppState {
    latest_metrics: Mutex<Vec<Value>>,
}

impl AppState {
    fn add_metric(&self, metric: Value) {
        let mut metrics = self.latest_metrics.lock().unwrap();
        if metrics.len() >= 20 {
            metrics.remove(0); // Remove the oldest metric
        }
        metrics.push(metric);
    }
}


async fn get_metrics_json(State(state): State<Arc<AppState>>) -> Json<Value> {
    let metrics_guard = state.latest_metrics.lock().unwrap().last().cloned();
    let data = metrics_guard.clone().unwrap_or_else(|| serde_json::json!({ "message": "No metrics yet." }));
    Json(data)
}

async fn get_state_json(State(state): State<Arc<AppState>>) -> Json<Value> {
    let metrics_guard = state.latest_metrics.lock().unwrap();
    let data = serde_json::json!({
        "latest_metrics": metrics_guard.clone(),
    });
    Json(data)
}


#[tokio::main]
async fn main() {
    println!("Starting Axum Webhook Server...");
    // Setup tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    let state = Arc::new(AppState::default());

    let static_service = Router::new()
        .nest_service("/static", ServeDir::new("./static"));
        //.nest_service("/static", ServeDir::new("/workspaces/system_monitor/static"));
    
    let dynamic_routes = Router::new()
        .route("/dashboard", get(render_dashboard))
        .route("/api/state", get(get_state_json))
        .route("/api/state/lastest", get(get_metrics_json))
        .route("/webhook", post(receive_webhook))
        .route("/", get(render_dashboard))
        .with_state(state);   // ✅ Apply state only here
    
    let app = static_service
        .merge(dynamic_routes)        
        .layer(
            TraceLayer::new_for_http()
                // Customize how spans are created for requests
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                // Customize what happens when a response is produced
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                // You can also add .on_request(...) for when a request starts
                // and .on_failure(...) for when a request fails
        );        
        //.layer(TraceLayer::new_for_http());
    
    let listener = TcpListener::bind("0.0.0.0:5000").await.unwrap();
    
    axum::serve(listener, app)
        .await
        .unwrap();
}


async fn receive_webhook(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) {
    state.add_metric(Some(payload).unwrap());
}

async fn render_dashboard() -> Html<String> {
    let version = env!("CARGO_PKG_VERSION").to_string();

    let context_json = serde_json::to_string(&serde_json::json!({
        "version": version,
    })).unwrap();

    let rendered = DashboardTemplate {
        context: &context_json,
    }
    .render()
    .unwrap();

    Html(rendered)
}

