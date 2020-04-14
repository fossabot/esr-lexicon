use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum SourceType {
    #[serde(rename = "url")]
    Url(String),
    #[serde(rename = "file")]
    FilePath(String),
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Source {
    #[serde(flatten)]
    pub source: SourceType,
    pub jq: String,
    pub output: String,
}

pub fn parse_config<S: Into<String>>(data: S) -> Result<Vec<Source>, Box<dyn Error>> {
    info!("parsing config");
    let config: Vec<Source> = serde_json::from_str(&data.into())?;
    info!("config parsed");
    Ok(config)
}

#[cfg(test)]
mod config_tests {
    use super::*;
    use crate::utils;

    #[tokio::test]
    async fn test_open_file() -> Result<(), Box<dyn std::error::Error>> {
        let config = utils::open_file("config.json").await?;
        assert_eq!(config.is_empty(), false);
        Ok(())
    }

    #[tokio::test]
    async fn test_parse_config() -> Result<(), Box<dyn std::error::Error>> {
        let config_string = r#"
        [
            {
                "url": "https://data.enseignementsup-recherche.gouv.fr/api/records/1.0/search/?dataset=fr-esr-repertoire-national-structures-recherche&rows=0&facet=libelle",
                "jq": ".[\"facet_groups\"][0][\"facets\"]|map(.[\"name\"])",
                "output": "scanr.struct.name"
            }
        ]"#;

        let config = parse_config(config_string)?;

        let expected_config = Source {
            source: SourceType::Url("https://data.enseignementsup-recherche.gouv.fr/api/records/1.0/search/?dataset=fr-esr-repertoire-national-structures-recherche&rows=0&facet=libelle".into()),
            jq: ".[\"facet_groups\"][0][\"facets\"]|map(.[\"name\"])".into(),
            output: "scanr.struct.name".into(),
        };

        assert_eq!(config, vec!(expected_config));

        let config_string = r#"
        [
            {
                "path": "/home/foo/bar.json",
                "jq": ".[\"facet_groups\"][0][\"facets\"]|map(.[\"name\"])",
                "output": "scanr.struct.name"
            }
        ]"#;

        let config = parse_config(config_string)?;

        let expected_config = Source {
            source: SourceType::FilePath("/home/foo/bar.json".into()),
            jq: ".[\"facet_groups\"][0][\"facets\"]|map(.[\"name\"])".into(),
            output: "scanr.struct.name".into(),
        };

        assert_eq!(config, vec!(expected_config));

        Ok(())
    }
}
