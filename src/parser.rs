use std::io::{self, Read};

use itertools::Itertools;
use thiserror::Error;

use crate::modal::DifficultTable;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("URL format must be ended with .json or .html")]
    UnSupportedURLFormat,
    #[error("Difficult table header data is corrupted: `{0}`")]
    CorruptedHeaderData(String),
    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    IOError(#[from] io::Error),
}

/// Parse one difficult table data from json data
///
/// * prefix_url: json corresponding url's prefix, could be empty. Only used when data_url is a relative path
///     e.g: Suppose our json is fetched from `https://stellabms.xyz/sl/header.json`, then prefix should be `https://stellabms.xyz/sl/`
///     This behavior would not be used in most cases, unit test could ignore this.
/// * data: difficult table header json data
pub fn parse_from_json(
    prefix_url: Option<String>,
    data: String,
) -> Result<DifficultTable, ParseError> {
    let mut header: DifficultTable = serde_json::from_slice(data.as_bytes())?;
    if header.name == "" {
        return Err(ParseError::CorruptedHeaderData(
            "Difficult table name cannot be empty".to_owned(),
        ));
    }
    if header.symbol == "" {
        return Err(ParseError::CorruptedHeaderData(
            "Difficult table symbol cannot be empty".to_owned(),
        ));
    }
    if header.data_url == "" {
        return Err(ParseError::CorruptedHeaderData(
            "Difficult table data_url cannot be empty".to_owned(),
        ));
    }
    if !header.data_url.starts_with("http") {
        let mut prefix_url = prefix_url.ok_or(ParseError::CorruptedHeaderData(
            "data_url is a relative path while no prefix url is provided".to_string(),
        ))?;
        if !prefix_url.ends_with("/") {
            prefix_url.push_str("/");
        }
        header.data_url = format!("{prefix_url}{}", header.data_url);
    }
    let mut resp = reqwest::blocking::get(header.data_url.clone())?;
    let mut body = String::new();
    resp.read_to_string(&mut body)?;
    header.contents = serde_json::from_slice(body.as_bytes())?;
    header.levels = header
        .contents
        .iter()
        .map(|content| content.level.clone())
        .unique()
        .sorted_by(|lhs, rhs| {
            let ilhs = lhs.parse::<i32>().ok();
            let irhs = rhs.parse::<i32>().ok();
            if ilhs.is_none() || irhs.is_none() {
                return Ord::cmp(lhs, rhs);
            }
            return Ord::cmp(&ilhs.unwrap(), &irhs.unwrap());
        })
        .collect();
    return Ok(header);
}

#[cfg(test)]
mod test {

    use super::parse_from_json;

    #[test]
    pub fn test_basic_header_deserialize() {
        let header_content = r#"
        {
            "data_url": "http://zris.work/bmstable/insane/insane_body.json",
            "last_update": "2017/02/05",
            "name": "発狂BMS難易度表",
            "original_url": "http://nekokan.dyndns.info/~lobsak/genocide/insane.html",
            "symbol": "★"
        }
        "#;

        let header = parse_from_json(None, header_content.to_string()).expect("parse failed");
        // (1) should be equal on basic fields
        assert_eq!(
            header.data_url,
            "http://zris.work/bmstable/insane/insane_body.json"
        );
        assert_eq!(header.last_update, "2017/02/05",);
        assert_eq!(header.name, "発狂BMS難易度表");
        assert_eq!(
            header.original_url,
            "http://nekokan.dyndns.info/~lobsak/genocide/insane.html"
        );
        assert_eq!(header.symbol, "★");
        // (2) contents should never be empty
        assert!(
            header.contents.len() > 0,
            "difficult table related contents should not be empty"
        );
        // (3) levels should never be empty
        assert!(
            header.levels.len() > 0,
            "difficult table related levels should not be empty"
        )
    }

    #[test]
    pub fn should_fail_on_garbage() {
        let garbage = "}not even a json{";
        assert!(parse_from_json(None, garbage.to_string()).is_err());
    }

    /**
     * Should at least contains name, symbol, data_url three fields
     */
    #[test]
    pub fn should_fail_on_missing_fields() {
        let test_cases = vec![
            r#"
                {
                    "data_url": "http://zris.work/bmstable/insane/insane_body.json",
                    "last_update": "2017/02/05",
                    "original_url": "http://nekokan.dyndns.info/~lobsak/genocide/insane.html",
                    "name": "",
                    "symbol": "★"
                }
            "#,
            r#"
                {
                    "data_url": "http://zris.work/bmstable/insane/insane_body.json",
                    "last_update": "2017/02/05",
                    "name": "発狂BMS難易度表",
                    "original_url": "http://nekokan.dyndns.info/~lobsak/genocide/insane.html",
                }
            "#,
            r#"
                {
                    "data_url": "",
                    "last_update": "2017/02/05",
                    "name": "発狂BMS難易度表",
                    "original_url": "http://nekokan.dyndns.info/~lobsak/genocide/insane.html",
                    "symbol": "★"
                }
            "#,
            r#"
                {
                    "data_url": "NOT A VALID HTTP URL",
                    "last_update": "2017/02/05",
                    "name": "発狂BMS難易度表",
                    "original_url": "http://nekokan.dyndns.info/~lobsak/genocide/insane.html",
                    "symbol": "★"
                }
            "#,
        ];
        assert!(test_cases
            .iter()
            .all(|data| parse_from_json(None, data.to_string()).is_err()))
    }
}
