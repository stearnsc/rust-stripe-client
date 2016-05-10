use url_encodable::UrlEncodable;

#[derive(Clone, Debug, Deserialize)]
pub struct DateOfBirth {
    pub day: i64,
    pub month: i64,
    pub year: i64
}

impl UrlEncodable for DateOfBirth {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        vec![
            ("day".to_string(), format!("{:02}", self.day)),
            ("month".to_string(), format!("{:02}", self.month)),
            ("year".to_string(), format!("{:04}", self.year))
        ]
    }
}
