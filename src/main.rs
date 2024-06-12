use std::{net::SocketAddr, sync::Arc};

use axum::{
    body::Body,
    http::header,
    response::{IntoResponse, Response},
    routing, serve, Extension, Router,
};
use prometheus::{register_gauge_vec, Encoder, TextEncoder};
use tokio::sync::Mutex;

mod settings;

#[derive(Clone)]
struct State {
    device: Arc<Mutex<tapo::PlugEnergyMonitoringHandler>>,
    current_power_gauge: prometheus::GaugeVec,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read settings from the environment
    let settings = settings::Settings::new()?;

    // Create a new Tapo API client and connect to the device
    let client = tapo::ApiClient::new(settings.tapo.username, settings.tapo.password);
    let device = client.p110(settings.tapo.ip).await?;

    // Get the device information initially to verify the connection
    let device_info = device.get_device_info().await?;
    println!("Successfully connected to {}!", device_info.model);

    let shared_device = Arc::new(Mutex::new(device));

    // Register prometheus metrics
    let current_power_gauge = register_gauge_vec!(
        "current_power",
        "Current power usage in watts",
        &["device_id", "type", "model", "mac", "nickname"]
    )?;

    // Create new server and route for the metrics endpoint
    let app = Router::new()
        .route("/metrics", routing::get(metrics_handler))
        .layer(Extension(State {
            device: shared_device,
            current_power_gauge,
        }));

    // Start listening for incoming connections
    let address = SocketAddr::from(([0, 0, 0, 0], 3456));
    let listener = tokio::net::TcpListener::bind(address).await?;
    println!("Server listening on http://{}", listener.local_addr()?);
    serve(listener, app).await?;

    return Ok(());
}

async fn metrics_handler(Extension(state): Extension<State>) -> impl IntoResponse {
    let device = state.device.lock().await;

    let device_info = device.get_device_info().await.unwrap();
    let current_power = device.get_current_power().await.unwrap();

    // Update the current power gauge
    state
        .current_power_gauge
        .with_label_values(&[
            &device_info.device_id,
            &device_info.r#type,
            &device_info.model,
            &device_info.mac,
            &device_info.nickname,
        ])
        .set(current_power.current_power as f64);

    // Gather the metrics and encode them in the Prometheus exposition format
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    // Return the metrics with the correct content type
    Response::builder()
        .header(header::CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .unwrap()
}
