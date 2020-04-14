use reqwest;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use std::error::Error;
use tokio::time::Instant;
use url::Url;

#[derive(Debug, Deserialize)]
struct ApiResult {
    #[serde(alias = "nextCursorMark")]
    next_cursor_mark: Option<String>,
    #[serde(flatten)]
    data: JsonValue,
}

pub async fn download_data<S: Into<String>>(url: S) -> Result<Vec<JsonValue>, Box<dyn Error>> {
    let instant = Instant::now();
    let mut url = url.into();
    let mut results: Vec<JsonValue> = vec![];
    let mut old_cursor = "".to_string();

    info!("downloading data from : {}", &url);

    loop {
        let result = reqwest::get(&url).await?.json::<ApiResult>().await?;
        results.push(result.data);

        match result.next_cursor_mark.clone() {
            Some(cursor) => {
                if old_cursor == cursor {
                    break;
                } else {
                    old_cursor = cursor.clone();
                    url = set_cursor_mark(url, cursor)?;
                    info!("fetching next cursor {:?}", url);
                }
            }
            None => break,
        }
    }

    info!("data downloaded in {:?}", instant.elapsed());
    Ok(results)
}

fn set_cursor_mark<S: Into<String>>(url: S, cursor: S) -> Result<String, Box<dyn Error>> {
    let mut parsed_url = Url::parse(&url.into())?;
    let cursor = cursor.into();

    use url::form_urlencoded;

    let mut new_query = form_urlencoded::Serializer::new(String::new());

    for (k, mut v) in parsed_url.query_pairs() {
        if k == "cursorMark" {
            v = Cow::from(&cursor);
        }

        new_query.append_pair(&k, &v);
    }

    parsed_url.set_query(Some(&new_query.finish()));

    Ok(parsed_url.as_str().into())
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

        let expected_result: Vec<JsonValue> = vec![serde_json::from_str(data)?];

        assert_eq!(
            result[0].get("args").unwrap(),
            expected_result[0].get("args").unwrap()
        );
        Ok(())
    }
}
