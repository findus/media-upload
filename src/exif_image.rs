use std::path::{Path, PathBuf};
use anyhow::{bail, Error};

pub(crate) fn remove_img_metadata(filepath: &Path) -> Result<PathBuf, Error> {
    let metadata = rexiv2::Metadata::new_from_path(filepath)?;

    let supports_any = metadata.supports_exif() || metadata.supports_xmp() || metadata.supports_iptc();

    if !supports_any {
        println!("Passed file does not support Exif, XMP or IPTC");
        bail!("Exif/XMP/IPTC is unsupported on this filetype");
    }

    // clear() removes Exif, XMP and IPTC alike, so GPS/author/caption data
    // stashed in XMP or IPTC (common on phone/editor exports) is wiped too,
    // not just Exif.
    metadata.clear();
    metadata.save_to_file(filepath)?;
    println!("Stripped Metadata");
    Ok(PathBuf::from(filepath))
}
