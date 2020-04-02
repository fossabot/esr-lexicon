use std::error::Error;
use std::fs;
use serde::Deserialize;


#[derive(Debug, Deserialize, PartialEq)]
pub struct Source {
    pub url: String,
    pub jq: String,
    pub output: String,
}

pub fn open_file<S: Into<String>>(path: S) -> String {
    let path = path.into();

    info!("opening config : {}", &path);

    let contents = fs::read_to_string(&path).expect("Failed to read config file");
    contents
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

   #[tokio::test]
    async fn test_open_file() -> Result<(), Box<dyn std::error::Error>> {
        let config = open_file("config.json");
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
            url: "https://data.enseignementsup-recherche.gouv.fr/api/records/1.0/search/?dataset=fr-esr-repertoire-national-structures-recherche&rows=0&facet=libelle".into(),
            jq: ".[\"facet_groups\"][0][\"facets\"]|map(.[\"name\"])".into(),
            output: "scanr.struct.name".into(),
        };

        assert_eq!(config, vec!(expected_config));
        Ok(())
    }
}
