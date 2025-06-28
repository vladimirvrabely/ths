#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ths::app::run().await
}
