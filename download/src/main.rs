use anyhow::Result;
use reqwest::Client;
use tokio;

#[tokio::main]
async fn main() {
    if let Err(err) = download_file().await {
        eprintln!("Error downloading file: {}", err);
    } else {
        println!("File downloaded successfully!");
    }
}

async fn download_file() -> Result<()> {
    // URL for Research Data - Home Value Index (ZHVI)
    let url = "https://raw.githubusercontent.com/selva86/datasets/master/BostonHousing.csv";

    println!("Downloading file from Research Data...");
    println!("URL: {}", url);

    // Create an async client
    let client = Client::new();

    // Send GET request asynchronously
    let response = client.get(url).send().await?;

    // Check if the request was successful
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to download file: HTTP status {}",
            response.status()
        ));
    }

    // Get the content asynchronously
    let content = response.bytes().await?;

    // Save the content to a file
    let filename = "home_value_index.csv";
    println!("Downloaded {} bytes to {}", content.len(), filename);
    tokio::fs::write(filename, &content).await?;

    Ok(())
}
