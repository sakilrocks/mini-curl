
mod cli;
mod http;

#[tokio::main]


async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = cli::parse_args()?;
    http::run(config).await?;
    Ok(())
    
}