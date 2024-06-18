#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    taskem::app::setup().await.unwrap();
    Ok(())
}
