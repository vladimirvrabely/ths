#[tokio::main]
async fn main() {
    ths_dashboard::service::run(
        &std::env::var("STATIC_DIR").expect("env var STATIC_DIR should be set"),
        &std::env::var("DB_PATH").expect("env var DB_PATH should be set"),
    )
    .await;
}
