use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents one difficult table meta info
///
/// NOTE: name, symbol, data_url must presents and must be non-empty value, and data_url should be a valid url
#[derive(Deserialize, Serialize, Debug)]
pub struct DifficultTable {
    /// BMS difficult table name
    pub name: String,
    /// BMS difficult table symbol
    pub symbol: String,
    /// BMS difficult table last update time (not used)
    #[serde(default)]
    pub last_update: String,
    /// BMS difficult table tag (unkown field)
    #[serde(default)]
    pub tag: String,
    /// BMS difficult table's related content url
    ///
    /// Warning: This field maybe a relative path
    ///
    /// TODO: The old jbmstable-parser shows that this field might be a list of urls.
    /// For now, we don't care
    #[serde(default)]
    pub data_url: String,
    /// Unkown field
    #[serde(default)]
    pub data_rule: Vec<String>,
    /// Special BMS difficult table name mapping (unkown field)
    #[serde(default)]
    pub attr: String,
    /// BMS difficult table mode (e.g beat-5k, beat-7k, popn-5k...)
    ///
    /// Warning: This field is not used
    #[serde(default)]
    pub mode: String,
    /// BMS difficult table source url
    #[serde(default)]
    pub original_url: String,
    /// BMS difficult table related contents
    #[serde(skip_deserializing)]
    pub contents: Vec<DifficultTableElement>,
    /// BMS difficult table related levels
    ///
    /// This field is ensured to be sorted, which comparison rule between lhs and rhs is definied as:
    /// * if lhs and rhs are both numbers, then compare them as number
    /// * if any of them are not number, then compare them as string
    /// The level field is forced to be existed and cannot be empty, so there is no other cases
    ///
    /// # Example:
    /// ```text
    /// [0, 1, 2, 3, ..., 24, 25, ???]
    /// ```
    #[serde(skip_deserializing)]
    pub levels: Vec<String>,
    /// BMS difficult table related courses
    ///
    /// # Format Explanation
    /// Courses are defined as a two-dimensional array, for example:
    /// ```text
    /// "course": [
    ///    [
    ///      {
    ///        "name": "Satellite sl0",
    ///        --- Snipped ---
    ///      }
    ///    ]
    /// ]
    /// ```
    /// Which is not very handy, therefore this field is defined as Vec<DifficultTableCourse> and
    /// with custom serializer/deserializer.
    /// * lift_serialize: serialize courses to a two-dimensional array
    /// * unlift_deserialize: deserialize a two-dimensional array to courses
    ///
    #[serde(
        serialize_with = "lift_serialize",
        deserialize_with = "unlift_deserialize",
        rename = "course",
        default
    )]
    pub courses: Vec<DifficultTableCourse>,
}

///
/// Represents one difficult table related content
///
/// Warning: due to some historical issues, sha256 is not always present
#[derive(Deserialize, Serialize, Debug)]
pub struct DifficultTableElement {
    /// song title
    pub title: String,
    /// song artist
    pub artist: String,
    /// MD5 hash
    pub md5: String,
    /// SHA256 hash
    #[serde(default)]
    pub sha256: String,
    /// song mode(unkown field)
    #[serde(default)]
    pub mode: String,
    /// song level mark
    pub level: String,
    /// variant(差分) name(unkown field)
    #[serde(default)]
    pub diff_name: String,
    /// song comment(discarded, not used field)
    #[serde(skip)]
    pub comment: String,
    /// song info(unkown field)
    #[serde(default)]
    pub info: String,
    /// bms id(unkown field)
    #[serde(default)]
    pub bms_id: String,
}

/// Represents one difficult table related course
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DifficultTableCourse {
    /// course name
    pub name: String,
    /// course constraints
    #[serde(rename = "constraint")]
    pub constraints: Vec<String>,
    /// course trophy
    pub trophy: Vec<DifficultTableCourseTrophy>,
    /// chart md5s
    pub md5: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DifficultTableCourseTrophy {
    // trophy name
    pub name: String,
    // trophy miss rate
    #[serde(rename = "missrate")]
    pub miss_rate: f32,
    // trophy score rate
    #[serde(rename = "scorerate")]
    pub score_rate: f32,
}

fn lift_serialize<S>(x: &Vec<DifficultTableCourse>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let lift_vec = vec![x.clone()];
    lift_vec.serialize(s)
}

fn unlift_deserialize<'de, D>(d: D) -> Result<Vec<DifficultTableCourse>, D::Error>
where
    D: Deserializer<'de>,
{
    let lifted_courses: Vec<Vec<DifficultTableCourse>> = Deserialize::deserialize(d)?;
    Ok(lifted_courses.into_iter().flatten().collect())
}
