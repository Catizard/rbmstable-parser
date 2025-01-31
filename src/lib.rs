use std::io::Read;

pub use modal::DifficultTable;
pub use modal::DifficultTableElement;
pub use parser::ParseError;

mod modal;
mod parser;

/// Parse difficult table data from an url
///
/// * `url` - difficult table url, must be valid HTTP url and be suffixed with .htm[l] or .json
///
/// # Example:
/// ```text
/// // Parse Satellite table, which url is `https://stellabms.xyz/sl/table.html`
/// let satellite_header_url = "https://stellabms.xyz/sl/table.html";
/// let dth: DifficultTable = parse(satellite_header_url.to_string())?;
/// ```
pub fn parse(url: String) -> Result<DifficultTable, ParseError> {
    if !url.starts_with("http") {
        return Err(ParseError::UnSupportedURLFormat);
    }
    if !url.ends_with(".json") && !url.ends_with(".htm") && !url.ends_with(".html") {
        return Err(ParseError::UnSupportedURLFormat);
    }
    let mut resp = reqwest::blocking::get(url.clone())?;
    let mut body = String::new();
    resp.read_to_string(&mut body)?;
    if body.is_empty() {
        return Err(ParseError::CorruptedHeaderData(format!(
            "Get nothing from {}",
            url
        )));
    }
    // If url is ends with .json, then we don't do anything
    if url.ends_with(".json") {
        let prefix_url = url[0..=url.rfind('/').unwrap()].to_owned();
        return parser::parse_from_json(Some(prefix_url), body);
    }
    // Otherwise, we need an extra step to get the header json content
    // <meta name="bmstable" content="header.json">
    //                                -----------> what we want
    let meta_line = body
        .lines()
        .find(|line| line.contains("<meta name=\"bmstable\""))
        .ok_or(ParseError::CorruptedHeaderData(
            "Cannot fetch meta line".to_string(),
        ))?;
    let pos = meta_line
        .find("content=")
        .ok_or(ParseError::CorruptedHeaderData(
            "Cannot parse meta line".to_string(),
        ))?;
    let l = pos + "content=".len() + 1;
    let r = meta_line.len() - 4;
    let mut header_url = url[0..=url.rfind('/').unwrap()].to_owned();
    let prefix_url = header_url.clone();
    header_url.push_str(&meta_line[l..r]);
    let mut resp = reqwest::blocking::get(header_url)?;
    // NOTE: don't reuse the body
    let mut body = String::new();
    resp.read_to_string(&mut body)?;
    return parser::parse_from_json(Some(prefix_url), body);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn should_fail_on_unsupported_format() {
        let test_cases = vec![
            "NOT A VALID HTTP URL",
            "ftp://satellite.json",
            "http://zris.work/bmstable/satellite/header",
        ];
        assert!(test_cases.iter().all(|url| parse(url.to_string()).is_err()));
    }

    /// basic parse api test
    ///
    /// Parse difficult table data from below urls:
    /// * http://zris.work/bmstable/satellite/header.json (.json, has courses)
    /// * https://stellabms.xyz/sl/table.html (.html, has courses)
    /// * http://zris.work/bmstable/insane2/insane_header.json (.json, has courses)
    /// * http://zris.work/bmstable/insane/insane_header.json (.json, has no courses)
    #[test]
    pub fn basic_test() {
        // (url, has_courses)
        let test_cases: Vec<(&str, bool)> = vec![
            ("http://zris.work/bmstable/satellite/header.json", true),
            ("https://stellabms.xyz/sl/table.html", true),
            ("http://zris.work/bmstable/insane2/insane_header.json", true),
            ("http://zris.work/bmstable/insane/insane_header.json", false),
        ];
        for case in test_cases {
            let (header_url, has_courses) = case;
            println!("[basic_test]: current test case is ({header_url}, {has_courses})");
            let dth: DifficultTable = parse(header_url.to_string()).expect("parse json url failed");
            assert!(
                !dth.name.is_empty(),
                "difficult table name should not be empty"
            );
            assert!(
                !dth.symbol.is_empty(),
                "difficult table symbol should not be empty"
            );
            assert!(
                !dth.data_url.is_empty(),
                "difficult table data_url should not be empty"
            );
            assert!(
                dth.contents.len() > 0,
                "difficult table contents should not be empty"
            );
            assert!(
                dth.levels.len() > 0,
                "difficult table levels should not be empty"
            );
            if has_courses {
                assert!(
                    dth.courses.len() > 0,
                    "difficult table courses should not be empty"
                );
                let value = serde_json::to_value(&dth).unwrap();
                let serialized_courses = serde_json::to_string(&value["course"]).unwrap();
                assert!(
                    serialized_courses.starts_with("[[") && serialized_courses.ends_with("]]"),
                    "difficult table courses should be serialized to two dimensional array, got {serialized_courses}"
                )
            }
        }
    }
}
