use super::StripeObject;

#[derive(Clone, Debug, Deserialize)]
pub struct ApiList<T: StripeObject> {
    pub data: Vec<T>,
    pub has_more: bool,
    pub total_count: i64,
    pub url: String
}
