use std::error::Error;
use tokio::fs::OpenOptions;
use tokio::prelude::*;

pub async fn write_data<S: Into<String>>(data: S, file_path: S) -> Result<(), Box<dyn Error>> {
    let file_path = file_path.into();

    info!("writing data to : {}", file_path);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&file_path)
        .await?;

    file.write_all(data.into().as_bytes()).await?;

    Ok(())
}

#[cfg(test)]
mod save_test {
    use super::*;
    use tempfile::NamedTempFile;

   #[tokio::test]
    async fn test_write_data() -> Result<(), Box<dyn std::error::Error>> {
        let file = NamedTempFile::new()?;
        let input = "lorem\nipsum";
        let file_path = format!("{}", &file.path().display());

        write_data(input, &file_path).await?;

        Ok(())
    }
}