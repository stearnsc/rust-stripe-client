use url_encodable::UrlEncodable;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Address {
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub country: String,
    pub postal_code: String,
    pub state: Option<String>
}

impl UrlEncodable for Address {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        let mut vec = Vec::new();
        match *self { Address {
            ref line1,
            ref line2,
            ref city,
            ref country,
            ref postal_code,
            ref state
        } => {
            vec.push(("line1".to_string(), line1.to_string()));
            if let &Some(ref line2) = line2 {
                vec.push(("line2".to_string(), line2.to_string()));
            }
            vec.push(("city".to_string(), city.to_string()));
            vec.push(("country".to_string(), country.to_string()));
            vec.push(("postal_code".to_string(), postal_code.to_string()));
            if let &Some(ref state) = state {
                vec.push(("state".to_string(), state.to_string()));
            }
        }}
        vec
    }
}
