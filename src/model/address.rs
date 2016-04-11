#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Address {
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub country: String,
    pub postal_code: String,
    pub state: Option<String>
}
