use anyhow::{Result, anyhow};
use ez_ffmpeg::{FfmpegContext, Output};
use std::path::Path;

// takes in a file, remuxes it to a wav
pub async fn to_wav(input_path: &str, output_folder: &str) -> Result<String> {
    let file_stem = Path::new(input_path)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| anyhow!("invalid path"))?;
    let output_path = format!("{}/{}.wav", output_folder, file_stem);

    let build_ctx = FfmpegContext::builder()
        .input(input_path)
        .output(Output::from(output_path.clone()).set_audio_sample_rate(16000))
        .build()?;
    build_ctx.start()?.await?;

    Ok(output_path)
}
