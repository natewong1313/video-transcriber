use crate::db::Db;
use crate::models::{Task, TaskStatus};
use crate::whisper::ModelType;
use crate::{converter, utils};
use anyhow::{Result, anyhow};
use hound::{WavReader, WavSpec};
use log;
use reqwest::Url;
use sqlx::PgPool;
use std::path::Path;
use tokio::runtime::Handle;
use whisper_rs::{DtwMode, FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub const DOWNLOADS_FOLDER_PATH: &str = "./tmp/downloads";

pub fn start(
    model_type: ModelType,
    model_path: String,
    transcriber_task: Task,
    db_pool: PgPool,
) -> Result<String> {
    let task_id = transcriber_task.id;
    let task_name = format!("task{task_id}");
    log::info!(target: &task_name, "in progress");

    // need this to run async function in this sync block
    let rt = Handle::current();
    let db = Db::new(db_pool);
    rt.block_on(db.update_task_status(transcriber_task.clone(), TaskStatus::InProgress))?;
    whisper_rs::install_logging_hooks();

    log::info!(target: &task_name, "checking url");
    // first we download the users file from s3
    let download_url = Url::parse(&transcriber_task.url)?;
    let download_url_path = Path::new(download_url.path());

    let file_name = download_url_path
        .file_name()
        .and_then(|fname| fname.to_str())
        .ok_or_else(|| anyhow!("invalid url"))?;
    let Some(file_ext) = download_url_path.extension() else {
        log::error!(target: &task_name, "invalid file ext");
        return Err(anyhow!("invalid file extension"));
    };

    // edge case where files that are uploaded to s3 with spaces need to be decoded
    let decoded_file_name = urlencoding::decode(file_name)?;
    let mut file_path = format!(
        "{}/{}_{}",
        DOWNLOADS_FOLDER_PATH, transcriber_task.id, decoded_file_name
    );

    log::info!(target: &task_name, "downloading from s3");
    rt.block_on(utils::download_file_from_url(
        &transcriber_task.url,
        &file_path,
    ))?;
    if file_ext != "wav" {
        log::info!(target: &task_name, "converting file");
        // whisper only works with wav files to we remux
        file_path = rt.block_on(converter::to_wav(&file_path, DOWNLOADS_FOLDER_PATH))?;
    }

    log::info!(target: &task_name, "init whisper");
    // now we can run the model
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

    log::info!(target: &task_name, "reading input file");
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

    log::info!(target: &task_name, "start whisper");
    ctx_state.full(params, &samples)?;

    log::info!(target: &task_name, "process segments");
    let num_segments = ctx_state.full_n_segments()?;
    let mut transcript = String::new();

    for i in 0..num_segments {
        let segment = ctx_state.full_get_segment_text(i)?;
        transcript.push_str(&segment);
    }

    log::info!(target: &task_name, "uploaded transcript");
    rt.block_on(db.update_task_transcript(transcriber_task.clone(), &transcript))?;
    rt.block_on(db.update_task_status(transcriber_task.clone(), TaskStatus::Finished))?;

    Ok(transcript)
}
