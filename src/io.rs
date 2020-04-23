use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use actix_multipart::{Field};
use actix_web::{Error, web};
use actix_web::http::header::{ContentDisposition};
use futures::StreamExt;

pub(crate) fn copy_file(file: &Path, destination: &Path) -> Result<(), Error> {
    println!("Move img to {}", destination.to_str().unwrap().to_owned());
    std::fs::copy(file, &destination)?;
    Ok(())
}

pub(crate) fn get_filename_extension(path: &Path) -> Option<String> {
        match path.extension()?.to_os_string().to_str() {
            Some(str) =>  Some(str.to_string()),
            None => None,
        }
}

pub(crate) fn generate_public_filename(content_type: ContentDisposition, iteration: usize) -> Option<String> {
    let filename = content_type.get_filename().unwrap();
    let extension = get_filename_extension(Path::new(&filename))?;

    let start = SystemTime::now();

    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");

    let epoch_in_seconds = since_the_epoch.as_secs();

    let filename_pub_epoch = format!("{}_{}.{}", epoch_in_seconds, iteration, extension);

    Some(format!("{}", filename_pub_epoch))
}

pub(crate) async fn save_file_to_temp_folder(field: &mut Field, temp_path: String) -> Result<(), Error> {
    let mut f = web::block(|| std::fs::File::create(temp_path)).await.unwrap();
    // Field in turn is stream of *Bytes* object
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        // filesystem operations are blocking, we have to use threadpool
        f = web::block(move || f.write_all(&data).map(|_| f)).await?;
    }
    Ok(())
}