use serde::de::Deserialize;
use serde::ser::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiList<T: Serialize + Deserialize> {
    data: Vec<T>,
    has_more: bool,
    total_count: i64,
    url: String
}
