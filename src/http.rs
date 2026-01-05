
use crate::cli::Config;
use reqwest::{Client};
use reqwest::header::CONTENT_TYPE;
use std::error::Error;


pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let mut request = client.request(config.method, &config.url);

    if let Some(body) = config.body {
        request = request.body(body);
    }


    let response = request.send().await?;

    println!("status: {}", response.status());
    println!("\nheaders:");
    for (k, v) in response.headers() {
        println!("{}: {:?}", k, v);
    }

    println!("\nbody:\n");


    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");


    if content_type.contains("application/json") {
        let json: serde_json::Value = response.json().await?;
        println!("{}", serde_json::to_string_pretty(&json)?);
    } else {
        let text = response.text().await?;
        println!("{}", text);
    }

    Ok(())
    
}