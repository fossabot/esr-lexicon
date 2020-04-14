use std::error::Error;
use tokio::fs::File;
use tokio::prelude::*;

pub async fn open_file<S: Into<String>>(path: S) -> Result<String, Box<dyn Error>> {
    let path = path.into();

    info!("opening file : {}", &path);

    let mut file = File::open(path).await?;

    let mut contents = vec![];

    file.read_to_end(&mut contents).await?;

    Ok(String::from_utf8(contents)?)
}
