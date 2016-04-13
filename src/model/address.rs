use std::collections::BTreeMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Address {
    pub line1: String,
    pub line2: Option<String>,
    pub city: String,
    pub country: String,
    pub postal_code: String,
    pub state: Option<String>
}

impl Into<BTreeMap<String, String>> for Address {
    fn into(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        match self { Address { line1, line2, city, country, postal_code, state } => {
            map.insert("line1".to_string(), line1);
            if let Some(line2) = line2 {
                map.insert("line2".to_string(), line2);
            }
            map.insert("city".to_string(), city);
            map.insert("country".to_string(), country);
            map.insert("postal_code".to_string(), postal_code);
            if let Some(state) = state {
                map.insert("state".to_string(), state);
            }
        }}
        map
    }
}

impl<'a> Into<BTreeMap<String, String>> for &'a Address {
    fn into(self) -> BTreeMap<String, String> {
        self.clone().into()
    }
}
