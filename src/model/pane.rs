use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TmuxPane {
    pub id: String,
    pub index: u32,
    pub active: bool,
    pub command: String,
    pub width: u32,
    pub height: u32,
    pub top: u32,
    pub left: u32,
    pub cwd: String,
}
