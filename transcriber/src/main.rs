use std::{error::Error, path::Path};

use whisper::{ModelType, do_transcription};

mod converter;
mod whisper;

const MODEL_TYPE: ModelType = ModelType::Base;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut file_path = std::env::args()
        .nth(1)
        .expect("please provide a file path as an argument");

    let model_path = whisper::download_model(MODEL_TYPE).await?;

    let Some(file_ext) = Path::new(&file_path).extension() else {
        return Err(Box::from("could not parse input file"));
    };

    if file_ext != "wav" {
        file_path = converter::to_wav(file_path).await?;
    }

    do_transcription(MODEL_TYPE, model_path, file_path).await?;

    Ok(())
}
