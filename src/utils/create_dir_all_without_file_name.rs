use std::{path::{PathBuf}, borrow::Borrow};

use anyhow::Result;
use tokio::fs::create_dir_all;

pub async fn create_dir_all_without_file_name(file_path: impl Borrow<PathBuf>) -> Result<()> {
    let mut dir = PathBuf::from(file_path.borrow());
    dir.pop();
    create_dir_all(dir).await?;

    Ok(())
}