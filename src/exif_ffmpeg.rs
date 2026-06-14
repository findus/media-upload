use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{bail, Error};
use thiserror::Error as ThisError;

pub(crate) fn remove_video_metadata(filepath: &Path) -> Result<PathBuf, Error> {
    convert_file(filepath)
}

#[derive(Debug, ThisError)]
enum ConvertError {
    #[error("File Creation Failed Error")]
    FileCreationFailed,
    #[error("ToStr Failed")]
    ToStrFailed,
}

fn convert_file(file: &Path) -> Result<PathBuf, Error> {
    let filename = file
        .file_name()
        .ok_or(ConvertError::FileCreationFailed)?
        .to_str()
        .ok_or(ConvertError::ToStrFailed)?;

    let out = format!("/tmp/processed/{}", filename);
    std::fs::create_dir_all(Path::new("/tmp/processed/"))?;
    let output = Command::new("ffmpeg")
        .arg("-y")
        .args(["-i", file.to_str().ok_or(ConvertError::ToStrFailed)?])
        .args(["-map_metadata", "-1"])
        .args(["-c:v", "copy"])
        .args(["-c:a", "copy"])
        .arg(&out)
        .output()?;

    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    if output.status.code() != Some(0) {
        bail!("FFmpeg quit with a non zero exit code!");
    }

    let mut path = PathBuf::new();
    path.push(out);
    Ok(path)
}

#[cfg(test)]
mod test_ffmpeg {
    use std::path::Path;
    use crate::exif_ffmpeg::convert_file;

    #[test]
    #[ignore]
    fn it_works() {
        let _ = convert_file(Path::new("/Users/findus/Videos/Zeitraffer.mp4"));
    }
}
