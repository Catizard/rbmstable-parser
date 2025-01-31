// Represents one difficult table meta info
pub struct DifficultTable {
    // BMS difficult table name
    pub name: String,
    // BMS difficult table symbol
    pub symbol: String,
    // BMS difficult table tag
    pub tag: String,
    // BMS difficult table's related content url
    pub data_url: String,
    // Special BMS difficult table name mapping
    pub attr: String,
    // BMS difficult table mode
    pub mode: String,
    // BMS difficult table source url
    pub source_url: String,
    // BMS difficult table related contents
    pub contents: Vec<DifficultTableElement>
}

/*
* Represents one difficult table related content
* Warning: due to some historical issues, md5 and sha256 fields are not always both provided
*/
pub struct DifficultTableElement {
    // song title
    pub title: String,
    // song artist
    pub artist: String,
    // MD5 hash
    pub md5: String,
    // SHA256 hash
    pub sha256: String,
    // song mode
    pub mode: String,
    // song level mark
    pub level: String,
    // variant(差分) name
    pub diff_name: String,
    // song comment
    pub comment: String,
    // song info
    pub info: String,
    // bms id(?)
    pub bms_id: String
}
