mod settings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = settings::Settings::new()?;

    let client = tapo::ApiClient::new(settings.tapo.username, settings.tapo.password);
    let device = client.p110(settings.tapo.ip).await?;

    let device_info = device.get_device_info().await?;

    println!("Successfully connected to {}!", device_info.model);

    return Ok(());
}
