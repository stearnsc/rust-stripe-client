use std::collections::BTreeMap;
use super::source_type::SourceType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    pub currency: String,
    pub amount: i64,
    pub source_types: Option<BTreeMap<SourceType, i64>>
}
