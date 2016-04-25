#[derive(Clone, Debug, Deserialize)]
pub struct Verification {
    fields_needed: Vec<String>,
    due_by: i64,
    contacted: bool,
    disabled_reason: String
}
