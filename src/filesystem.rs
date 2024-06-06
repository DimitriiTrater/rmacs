use std::path::Path;
use std::io::{self};
use std::sync::Arc;
use rfd;

pub async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, Error> {
    tokio::fs::read_to_string(path)
        .await
        .map(Arc::new)
        .map_err(|error| error.kind())
        .map_err(Error::IO)
}

pub async fn pick_file() -> Result<Arc<String>, Error> {
    let path = rfd::AsyncFileDialog::new()
        .set_title("Choose a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(path.path()).await

}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IO(io::ErrorKind),
}
