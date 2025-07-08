#[tokio::main]
async fn main() {
    ths_dashboard::service::run(
        &std::env::var("STATIC_DIR").expect("env var STATIC_DIR should be set"),
    )
    .await;
}
