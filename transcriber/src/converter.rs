use std::{
    error::Error,
    path::{Path, PathBuf},
};

use tokio::fs;

const TEMP_FOLDER_PATH: &str = "./tmp";

pub async fn to_wav(input_path: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    fs::create_dir_all(TEMP_FOLDER_PATH).await?;

    Ok(PathBuf::new())
}
