use std::path::PathBuf;
use std::io::{self};
use std::sync::Arc;
use rfd;

pub async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| error.kind())
        .map_err(Error::IO)?;
    Ok((path, contents))
}

pub async fn pick_file() -> Result<(PathBuf, Arc<String>), Error> {
    let path = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(path.path().to_owned()).await
}

pub fn default_file() -> PathBuf {
    PathBuf::from(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR")))
}

pub async fn save_file(path: Option<PathBuf>, text: String) -> Result<PathBuf, Error>{
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .set_title("Choose a file name")
            .save_file()
            .await
            .ok_or(Error::DialogClosed)
            .map(| handle | handle.path().to_owned())?
    };
    tokio::fs::write(&path, text)
        .await
        .map_err(| error | Error::IO(error.kind()))?;
    Ok(path)
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IO(io::ErrorKind),
}
