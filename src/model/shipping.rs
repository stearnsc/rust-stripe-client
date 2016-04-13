use std::collections::BTreeMap;
use super::address::Address;
use util::structured;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Shipping {
    pub address: Address,
    pub carrier: Option<String>,
    pub name: String,
    pub phone: String,
    pub tracking_number: Option<String>
}

impl Into<BTreeMap<String, String>> for Shipping {
    fn into(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        match self { Shipping { address, carrier, name, phone, tracking_number } => {
            map.extend(structured("address", address.into()));
            if let Some(carrier) = carrier {
                map.insert("carrier".to_string(), carrier);
            }
            map.insert("name".to_string(), name);
            map.insert("phone".to_string(), phone);
            if let Some(tracking_number) = tracking_number {
                map.insert("tracking_number".to_string(), tracking_number);
            }
        }}
        map
    }
}

impl<'a> Into<BTreeMap<String, String>> for &'a Shipping {
    fn into(self) -> BTreeMap<String, String> {
        self.clone().into()
    }
}
