#[derive(Clone, Debug, Deserialize)]
pub struct Delete {
    pub deleted: bool,
    pub id: String
}
