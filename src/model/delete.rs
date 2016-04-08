#[derive(Debug, Clone, Deserialize)]
pub struct Delete {
    pub deleted: bool,
    pub id: String
}
