use ez_ffmpeg::{FfmpegContext, Output};
use std::{error::Error, path::PathBuf};
use tokio::fs;

const TEMP_FOLDER_PATH: &str = "./tmp";

// takes in a file, remuxes it to a wav
pub async fn to_wav(input_path: String) -> Result<String, Box<dyn Error>> {
    fs::create_dir_all(TEMP_FOLDER_PATH).await?;

    let output_path = format!("{}/out.wav", TEMP_FOLDER_PATH);

    let build_ctx = FfmpegContext::builder()
        .input(input_path)
        .output(Output::from(output_path.clone()).set_audio_sample_rate(16000))
        .build()?;
    build_ctx.start()?.await?;

    Ok(output_path)
}
