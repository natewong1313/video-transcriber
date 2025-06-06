use std::{error::Error, path::PathBuf};

use futures_util::StreamExt;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

const MODELS_PATH: &str = "./models";

pub enum ModelType {
    Base,
    Tiny,
    Small,
    // probably won't use these larger models but keeping them in case
    Medium,
    Large,
}
impl ModelType {
    fn filename(&self) -> &'static str {
        match self {
            ModelType::Base => "ggml-base.bin",
            ModelType::Tiny => "ggml-tiny.bin",
            ModelType::Small => "ggml-small.bin",
            ModelType::Medium => "ggml-medium.bin",
            ModelType::Large => "ggml-large.bin",
        }
    }
}

// given a model, download it to the file system and return the file path
pub async fn download_model(model_type: ModelType) -> Result<PathBuf, Box<dyn Error>> {
    fs::create_dir_all(MODELS_PATH).await?;

    let filename = model_type.filename();
    let output_path_str = format!("{}/{}", MODELS_PATH, filename);
    let output_path = PathBuf::from(&output_path_str);
    if output_path.exists() {
        return Ok(output_path);
    }

    let mut file = File::create_new(&output_path).await?;
    let download_url = format!(
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/{}",
        filename
    );
    let mut response_stream = reqwest::get(download_url).await?.bytes_stream();
    while let Some(item) = response_stream.next().await {
        let chunk = item?;
        file.write_all(&chunk).await?;
    }
    Ok(output_path)
}
