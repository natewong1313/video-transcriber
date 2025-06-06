use std::{error::Error, path::PathBuf};

mod converter;
mod whisper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let file_arg = std::env::args()
        .nth(1)
        .expect("please provide a file path as an argument");
    let mut file_path = PathBuf::from(&file_arg);

    let model_path_str = whisper::download_model(whisper::ModelType::Tiny).await?;
    let model_path = PathBuf::from(&model_path_str);

    let Some(file_ext) = file_path.extension() else {
        return Err(Box::from("could not parse input file"));
    };

    if file_ext != "wav" {
        file_path = converter::to_wav(file_path).await?;
        println!("new file path: {:?}", file_path);
    }

    Ok(())
}
