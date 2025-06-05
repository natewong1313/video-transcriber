use std::{error::Error, io::ErrorKind};

use futures_util::StreamExt;
use tokio::{
    fs::{File, metadata},
    io::AsyncWriteExt,
};

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
pub async fn download_model(model_type: ModelType) -> Result<String, Box<dyn Error>> {
    let filename = model_type.filename();
    let output_path = format!("./models/{}", filename);
    // check if model has already been downloaded
    // TODO: add logic for if a new model version is released
    let mut file = match File::create_new(&output_path).await {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists => return Ok(output_path),
            _ => return Err(Box::new(e)),
        },
    };

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
