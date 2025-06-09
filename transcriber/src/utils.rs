use anyhow::Result;
use futures_util::StreamExt;
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn download_file_from_url(url: &str, output_path: &str) -> Result<()> {
    let mut file = File::create_new(output_path).await?;
    let mut response_stream = reqwest::get(url).await?.bytes_stream();
    while let Some(item) = response_stream.next().await {
        let chunk = item?;
        file.write_all(&chunk).await?;
    }
    Ok(())
}
