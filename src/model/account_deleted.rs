#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDeleted {
    pub deleted: bool,
    pub id: String
}
