use reqwest;
use anyhow::Result;
use tokio;

async fn get_rss_feed(url: &str) -> Result<String> {
    let content = reqwest::get(url).await?.text().await?;
    println!("{:?}", content);
    Ok(content)
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html";
    let res = get_rss_feed(&url).await?;
    println!("{:?}", res);
    Ok(())
}
