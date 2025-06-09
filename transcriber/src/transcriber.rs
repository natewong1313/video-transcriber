use std::path::Path;

use crate::db::Db;
use crate::models::{Task, TaskStatus};
use crate::whisper::ModelType;
use crate::{converter, utils};
use anyhow::{Result, anyhow};
use hound::{WavReader, WavSpec};
use reqwest::Url;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::runtime::Handle;
use whisper_rs::{DtwMode, FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub const DOWNLOADS_FOLDER_PATH: &str = "./tmp/downloads";

pub fn start(
    model_type: ModelType,
    model_path: String,
    transcriber_task: Task,
    db_pool: PgPool,
) -> Result<String> {
    let rt = Handle::current();
    let db = Db::new(db_pool);
    rt.block_on(db.update_task_status(transcriber_task.clone(), TaskStatus::InProgress))?;
    // whisper_rs::install_logging_hooks();

    let download_url = Url::parse(&transcriber_task.url)?;
    let download_url_path = Path::new(download_url.path());

    let file_name = download_url_path
        .file_name()
        .and_then(|fname| fname.to_str())
        .ok_or_else(|| anyhow!("invalid url"))?;
    let Some(file_ext) = download_url_path.extension() else {
        return Err(anyhow!("invalid file extension"));
    };
    let decoded_file_name = urlencoding::decode(file_name)?;

    let mut file_path = format!(
        "{}/{}_{}",
        DOWNLOADS_FOLDER_PATH, transcriber_task.id, decoded_file_name
    );
    rt.block_on(utils::download_file_from_url(
        &transcriber_task.url,
        &file_path,
    ))?;
    if file_ext != "wav" {
        file_path = rt.block_on(converter::to_wav(&file_path, DOWNLOADS_FOLDER_PATH))?;
    }

    let mut ctx_params = WhisperContextParameters::default();
    ctx_params.dtw_parameters.mode = DtwMode::ModelPreset {
        model_preset: model_type.get_dtw_params(),
    };
    let ctx = WhisperContext::new_with_params(&model_path, WhisperContextParameters::default())?;

    let mut ctx_state = ctx.create_state()?;
    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });
    // params.set_print_special(false);
    // params.set_print_progress(false);
    // params.set_print_realtime(false);
    // params.set_print_timestamps(false);
    // params.set_token_timestamps(false);

    let reader = WavReader::open(file_path)?;
    let WavSpec {
        channels: _,
        sample_rate,
        bits_per_sample: _,
        sample_format: _,
    } = reader.spec();
    if sample_rate != 16000 {
        return Err(anyhow!("invalid sample rate: must be 165KHz"));
    }
    let og_samples_res: Result<Vec<i16>, _> = reader.into_samples::<i16>().collect();
    let og_samples = og_samples_res?;
    let mut samples = vec![0.0f32; og_samples.len()];
    whisper_rs::convert_integer_to_float_audio(&og_samples, &mut samples)?;

    // run that shit
    ctx_state.full(params, &samples)?;

    let num_segments = ctx_state.full_n_segments()?;
    let mut transcript = String::new();

    for i in 0..num_segments {
        let segment = ctx_state.full_get_segment_text(i)?;
        println!("{}", segment);
        transcript.push_str(&segment);
    }

    Ok(transcript)
}
