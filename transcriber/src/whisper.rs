use anyhow::Result;
use std::io::{Error, ErrorKind};
use tokio::fs::{self};
use whisper_rs::DtwModelPreset;

use crate::utils;

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
            ModelType::Large => "ggml-large-v3.bin",
        }
    }
    pub fn get_dtw_params(&self) -> DtwModelPreset {
        match self {
            ModelType::Base => DtwModelPreset::BaseEn,
            ModelType::Tiny => DtwModelPreset::TinyEn,
            ModelType::Small => DtwModelPreset::Small,
            ModelType::Medium => DtwModelPreset::Medium,
            ModelType::Large => DtwModelPreset::LargeV3,
        }
    }
}

// given a model, download it to the file system and return the file path
pub async fn download_model(model_type: ModelType) -> Result<String> {
    fs::create_dir_all(MODELS_PATH).await?;

    let filename = model_type.filename();
    let output_path = format!("{}/{}", MODELS_PATH, filename);
    let download_url = format!(
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/{}",
        filename
    );
    if let Err(e) = utils::download_file_from_url(&download_url, &output_path).await {
        match e.downcast_ref::<Error>() {
            // if the file already exists we can ignore this error
            Some(err) if err.kind() == ErrorKind::AlreadyExists => {}
            _ => return Err(e),
        }
    };

    Ok(output_path)
}
