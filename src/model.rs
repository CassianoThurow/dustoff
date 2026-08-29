use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CleanupItem {
    pub id: &'static str,
    pub label: &'static str,
    pub description: String,
    pub estimated_bytes: Option<u64>,
    pub risk: Risk,
    pub action: CleanupAction,
    pub available: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Risk {
    Low,
    Moderate,
    High,
}

impl Risk {
    pub fn marker(self) -> &'static str {
        match self {
            Self::Low => "safe",
            Self::Moderate => "review",
            Self::High => "caution",
        }
    }
}

#[derive(Debug, Clone)]
pub enum CleanupAction {
    RemoveContents(PathBuf),
    Command {
        program: &'static str,
        args: &'static [&'static str],
    },
}
