use url_encodable::UrlEncodable;

#[derive(Clone, Debug, Deserialize)]
pub struct Dimensions {
    pub height: f64,
    pub length: f64,
    pub weight: f64,
    pub width: f64
}

impl UrlEncodable for Dimensions {
    fn key_value_pairs(&self) -> Vec<(String, String)> {
        vec![
            ("height".to_string(), self.height.to_string()),
            ("length".to_string(), self.length.to_string()),
            ("weight".to_string(), self.weight.to_string()),
            ("width".to_string(),  self.width.to_string()),
        ]
    }
}
