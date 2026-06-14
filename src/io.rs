use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use actix_multipart::Field;
use actix_web::error::ErrorInternalServerError;
use actix_web::http::header::ContentDisposition;
use actix_web::web;
use anyhow::Error;
use futures::StreamExt;
use thiserror::Error as ThisError;

pub(crate) fn copy_file(file: &PathBuf, destination: &Path) -> Result<(), Error> {
    println!("Move img to {}", destination.to_str().unwrap());
    std::fs::copy(file, destination)?;
    Ok(())
}

pub(crate) fn get_filename_extension(path: &Path) -> Option<String> {
    match path.extension()?.to_os_string().to_str() {
        Some(str) => Some(str.to_string()),
        None => None,
    }
}

#[derive(Debug, ThisError)]
enum IOError {
    #[error("Could not extract Filename")]
    CouldNotExtractFilename,
    #[error("Could not get filename extension")]
    CouldNotGetFileNameExtension,
}

pub(crate) fn generate_public_filename(
    content_type: &ContentDisposition,
    iteration: usize,
) -> Result<String, Error> {
    let filename = content_type
        .get_filename()
        .ok_or(IOError::CouldNotExtractFilename)?;

    let extension = get_filename_extension(Path::new(filename))
        .ok_or(IOError::CouldNotGetFileNameExtension)?;

    let since_the_epoch = SystemTime::now().duration_since(UNIX_EPOCH)?;

    Ok(format!(
        "{}_{}.{}",
        since_the_epoch.as_nanos(),
        iteration,
        extension
    ))
}

pub(crate) async fn save_file_to_temp_folder(
    field: &mut Field,
    temp_path: String,
) -> Result<(), actix_web::Error> {
    let mut bytes = Vec::new();
    while let Some(chunk) = field.next().await {
        let data = chunk?;
        bytes.extend_from_slice(&data);
    }
    web::block(move || std::fs::write(&temp_path, &bytes))
        .await
        .map_err(ErrorInternalServerError)?
        .map_err(ErrorInternalServerError)
}
