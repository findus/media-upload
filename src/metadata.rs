use std::path::PathBuf;
use crate::exif_image;
use crate::exif_ffmpeg;
use anyhow::Error;

pub trait MetaData {
    fn remove_metadata(&self) -> Result<PathBuf, Error>;
}

pub struct VideoFile<'a> {
    pub path: &'a str,
}

impl MetaData for VideoFile<'_> {
    fn remove_metadata(&self) -> Result<PathBuf, Error> {
        exif_ffmpeg::remove_video_metadata(self.path.as_ref())
    }
}

pub struct Image<'a> {
    pub path: &'a str,
}

impl MetaData for Image<'_> {
    fn remove_metadata(&self) -> Result<PathBuf, Error> {
        exif_image::remove_img_metadata(self.path.as_ref())
    }
}

pub struct Noop<'a> {
    pub path: &'a str,
}

impl MetaData for Noop<'_> {
    fn remove_metadata(&self) -> Result<PathBuf, Error> {
        Ok(PathBuf::new())
    }
}
