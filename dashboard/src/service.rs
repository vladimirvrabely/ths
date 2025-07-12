use axum::{Router, routing::get};
use tokio::signal::unix::{SignalKind, signal};
use tokio::sync::mpsc;
use tower_http::services::ServeDir;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::handler;
use crate::state::SharedState;

pub async fn run(static_dir: &str, db_path: &str) {
    init_env_logging();
    tracing::info!("Starting ths-dashboard service");

    let mut stop_rx = catch_terminate_signal();

    // build our application with a route
    let static_dir = ServeDir::new(static_dir);

    let shared_state = SharedState::new(db_path)
        .await
        .expect("should connect to db");

    let app = Router::new()
        .route("/", get(handler::index))
        .route("/history", get(handler::history))
        .nest_service("/static", static_dir)
        .with_state(shared_state);

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("TCP listener should be created");
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            let _ = stop_rx.recv().await;
        })
        .await
        .expect("server should serve until graceful shutdown");
}

fn init_env_logging() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}

fn catch_terminate_signal() -> mpsc::Receiver<Option<()>> {
    let (stop_tx, stop_rx) = mpsc::channel(1);

    tokio::spawn(async move {
        let stop = signal(SignalKind::terminate())
            .expect("should create SIGTERM listener")
            .recv()
            .await;

        tracing::info!("Received stop signal");

        stop_tx.send(stop).await.unwrap();
    });

    stop_rx
}
