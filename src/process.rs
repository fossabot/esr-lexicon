use std::error::Error;
use crate::config;
use crate::download;
use crate::save;
use jq_rs;
use serde_json::Value as JsonValue;

pub async fn run(source: config::Source) -> Result<(), Box<dyn Error>> {
    let response = download::download_data(&source.url).await?;

    info!("processing data with jq expression : '{}'", source.jq);

    let result = parse(response.to_string(), source.jq)?;
   
    if result.is_empty() {
        warn!("empty data")
    } else {
        save::write_data(result, source.output).await?;
    }

    Ok(())
}

fn parse<S: Into<String>>(data: S, jq_expression: S) -> Result<String, Box<dyn Error>> {
    let jq_result: String = jq_rs::run(&jq_expression.into(), &data.into())?;
    let parsed_jq_result: JsonValue = serde_json::from_str(&jq_result)?;

    let result = match parsed_jq_result {
        JsonValue::Array(array) => array
            .iter()
            .filter_map(|v| v.as_str())
            .collect::<Vec<&str>>()
            .join("\n"),
        JsonValue::String(content) => content.clone(),
        _ => "".to_string(),
    };

    Ok(result)
}

#[cfg(test)]
mod process_tests {
    use super::*;
    use crate::config::{parse_config, Source};
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_run() -> Result<(), Box<dyn std::error::Error>> {
        let file = NamedTempFile::new()?;
        let file_path = format!("{}", &file.path().display());

        let config = Source {
            url : "https://postman-echo.com/get?foo1=bar1&foo2=bar2".into(),
            jq : ".[\"args\"][\"foo\"]".into(),
            output: file_path,
        };

        run(config).await?;

        Ok(())
    }


   #[tokio::test]
    async fn test_parse() -> Result<(), Box<dyn std::error::Error>> {
        let config_string = r#"
        [
            {
                "url": "https://data.enseignementsup-recherche.gouv.fr/api/records/1.0/search/?dataset=fr-esr-repertoire-national-structures-recherche&rows=0&facet=libelle",
                "jq": ".[\"facet_groups\"][0][\"facets\"]|map(.[\"name\"])",
                "output": "scanr.struct.name"
            }
        ]"#;

        let config = parse_config(config_string)?;
        
        let json_data = r#"
        {
            "nhits": 10896,
            "parameters": {
                "dataset": "fr-esr-repertoire-national-structures-recherche",
                "timezone": "UTC",
                "rows": 0,
                "format": "json",
                "facet": [
                    "libelle"
                ]
            },
            "records": [],
            "facet_groups": [
                {
                    "facets": [
                        {
                            "count": 4,
                            "path": "LABORATOIRE DE PSYCHOLOGIE",
                            "state": "displayed",
                            "name": "LABORATOIRE DE PSYCHOLOGIE"
                        },
                        {
                            "count": 3,
                            "path": "CENTRE D'ETUDE DES CORRESPONDANCES ET JOURNAUX INTIMES DES 19E ET 20E SIECLES",
                            "state": "displayed",
                            "name": "CENTRE D'ETUDE DES CORRESPONDANCES ET JOURNAUX INTIMES DES 19E ET 20E SIECLES"
                        }
                    ],
                    "name": "libelle"
                }
            ]
        }"#;

        let result = parse(json_data, &config[0].jq)?;
        let expected_result = "LABORATOIRE DE PSYCHOLOGIE\nCENTRE D'ETUDE DES CORRESPONDANCES ET JOURNAUX INTIMES DES 19E ET 20E SIECLES";

        assert_eq!(result, expected_result);

        Ok(())
    }
}