use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
struct Money {
    currency: String,
    amount: i64,
    source_types: Option<BTreeMap<String, i64>>
}
