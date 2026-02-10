use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TmuxWindow {
    pub name: String,
    pub index: u32,
    pub id: String,
    pub active: bool,
    pub panes: u32,
}
