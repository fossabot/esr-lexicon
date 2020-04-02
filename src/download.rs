use std::error::Error;
use reqwest;
use tokio::time::{Instant};
use serde_json::Value as JsonValue;

pub async fn download_data<S: Into<String>>(url: S) -> Result<JsonValue, Box<dyn Error>> {
    let instant = Instant::now();
    let url = url.into();
    
    info!("downloading data from : {}", &url);
    
    let response = reqwest::get(&url).await?.json::<JsonValue>().await?;
    
    info!("data downloaded in {:?}", instant.elapsed());
    
    Ok(response)
}

#[cfg(test)]
mod download_tests {
    use super::*;
    use serde_json;

   #[tokio::test]
    async fn test_download() -> Result<(), Box<dyn std::error::Error>> {
        let result = download_data("https://postman-echo.com/get?foo1=bar1&foo2=bar2").await?;
        let data = r#"
        {
            "args": {
                "foo1": "bar1",
                "foo2": "bar2"
            }
        }
        "#;

        let expected_result : JsonValue =  serde_json::from_str(data)?;
        
        assert_eq!(result.get("args").unwrap(), expected_result.get("args").unwrap());
        Ok(())
    }
}

