use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TmuxSession {
    pub name: String,
    pub id: String,
    pub windows: u32,
    pub attached: bool,
    pub created: u64,
}
