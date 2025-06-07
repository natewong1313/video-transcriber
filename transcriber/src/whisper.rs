use std::{error::Error, path::PathBuf, ptr::null_mut};

use futures_util::StreamExt;
use hound::{WavReader, WavSpec};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use whisper_rs::{
    DtwMode, DtwModelPreset, FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters,
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
            ModelType::Large => "ggml-large-v3.bin",
        }
    }
    fn get_dtw_params(&self) -> DtwModelPreset {
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
pub async fn download_model(model_type: ModelType) -> Result<String, Box<dyn Error>> {
    fs::create_dir_all(MODELS_PATH).await?;

    let filename = model_type.filename();
    let output_path_str = format!("{}/{}", MODELS_PATH, filename);
    let output_path = PathBuf::from(&output_path_str);
    if output_path.exists() {
        return Ok(output_path_str);
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
    Ok(output_path_str)
}

pub async fn do_transcription(
    model_type: ModelType,
    model_path: String,
    file_path: String,
) -> Result<String, Box<dyn Error>> {
    whisper_rs::install_logging_hooks();

    let mut ctx_params = WhisperContextParameters::default();
    ctx_params.dtw_parameters.mode = DtwMode::ModelPreset {
        model_preset: model_type.get_dtw_params(),
    };
    let ctx = WhisperContext::new_with_params(&model_path, WhisperContextParameters::default())?;

    let mut ctx_state = ctx.create_state()?;
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    params.set_token_timestamps(false);

    let reader = WavReader::open(file_path)?;
    let WavSpec {
        channels,
        sample_rate,
        bits_per_sample: _,
        sample_format: _,
    } = reader.spec();
    if channels < 2 {
        return Err(Box::from("invalid number of channels"));
    }
    if sample_rate != 16000 {
        println!("{}", sample_rate);
        return Err(Box::from("invalid sample rate: must be 165KHz"));
    }

    let samples_res: Result<Vec<i16>, _> = reader.into_samples::<i16>().collect();
    let samples = samples_res?;

    let mut audio = vec![0.0f32; samples.len()];
    whisper_rs::convert_integer_to_float_audio(&samples, &mut audio)?;
    audio = whisper_rs::convert_stereo_to_mono_audio(&audio)?;

    // run that shit
    ctx_state.full(params, &audio[..])?;

    let num_segments = ctx_state.full_n_segments()?;
    let mut transcript = String::new();

    for i in 0..num_segments {
        let segment = ctx_state.full_get_segment_text(i)?;
        println!("{}", segment);
        transcript.push_str(&segment);
    }

    Ok(transcript)
}
