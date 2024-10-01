pub mod date_time_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer};
    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S";
    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        match NaiveDateTime::parse_from_str(&s, FORMAT) {
            Ok(t) => Ok(t.and_utc()),
            Err(_) => match serde_json::from_str::<DateTime<Utc>>(&format!("\"{}\"", s)) {
                Ok(t) => Ok(t),
                Err(e) => Err(serde::de::Error::custom(format!(
                    "deserializing {} as DateTime<Utc> failed: {}",
                    s, e
                ))),
            },
        }
    }
}

pub mod nullable_date_time_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer};
    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S";
    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        if let Some(s) = s {
            // This is standard.
            match NaiveDateTime::parse_from_str(&s, FORMAT) {
                Ok(t) => Ok(Some(t.and_utc())),
                Err(_) => match serde_json::from_str::<DateTime<Utc>>(&format!("\"{}\"", s)) {
                    Ok(t) => Ok(Some(t)),
                    Err(e) => Err(serde::de::Error::custom(format!(
                        "deserializing {} as DateTime<Utc> failed: {}",
                        s, e
                    ))),
                },
            }
        } else {
            Ok(None)
        }
    }
}
