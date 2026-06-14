use std::path::{Path, PathBuf};
use anyhow::{bail, Error};

pub(crate) fn remove_img_metadata(filepath: &Path) -> Result<PathBuf, Error> {
    let metadata = rexiv2::Metadata::new_from_path(filepath)?;
    println!("Supports exif: {} -> {}", filepath.display(), metadata.supports_exif());

    match metadata.supports_exif() {
        true => {
            metadata.clear_exif();
            metadata.save_to_file(filepath)?;
            println!("Stripped Metadata");
            Ok(PathBuf::from(filepath))
        }
        _ => {
            println!("Passed file does not support exif");
            bail!("Exif is unsupported on this filetype");
        }
    }
}
