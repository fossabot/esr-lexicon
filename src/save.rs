use std::collections::HashSet;
use std::error::Error;
use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::io::BufReader;
use tokio::prelude::*;
use tokio::stream::StreamExt;

#[derive(Debug)]
pub enum WriteMode {
    Truncate,
    Append,
}

pub async fn write_data<S: Into<String>>(
    data: S,
    file_path: S,
    write_mode: WriteMode,
) -> Result<(), Box<dyn Error>> {
    let file_path = file_path.into();

    info!("writing data to : {}", file_path);

    let mut file = match write_mode {
        WriteMode::Append => {
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(&file_path)
                .await?
        }
        WriteMode::Truncate => {
            OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&file_path)
                .await?
        }
    };

    file.write_all(data.into().as_bytes()).await?;

    Ok(())
}

pub async fn dedup_file<S: Into<String>>(file_path: S) -> Result<(), Box<dyn Error>> {
    let file_path = file_path.into();
    let file = File::open(&file_path).await?;

    info!("Deduping content of {}", file_path);

    let reader = BufReader::new(file);
    let mut lines = reader
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .await
        .into_iter()
        .collect::<HashSet<String>>()
        .into_iter()
        .collect::<Vec<String>>();

    lines.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    let mut result: String = lines.join("\n");
    result = format!("{}\n", result);

    write_data(result, file_path, WriteMode::Truncate).await?;

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

        write_data(input, &file_path, WriteMode::Append).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_dedup() -> Result<(), Box<dyn std::error::Error>> {
        let file = NamedTempFile::new()?;
        let input = "lorem\nipsum\nlorem\n";
        let file_path = format!("{}", &file.path().display());

        write_data(input, &file_path, WriteMode::Truncate).await?;
        dedup_file(file_path).await?;

        Ok(())
    }
}
