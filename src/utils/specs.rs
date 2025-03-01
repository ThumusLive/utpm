use serde::{Deserialize, Serialize};
use typst_project::manifest::tool::Tool;

/// A modify version of the `typst.toml` adding options to utpm
#[derive(Serialize, Deserialize, Clone)]
pub struct Extra {
    /// The name of where you store your packages (default: local)
    pub namespace: Option<String>,

    /// List of url's for your dependencies (will be resolved with install command)
    pub dependencies: Option<Vec<String>>,

    /// Exclude files when using `publish` command.
    pub exclude: Option<Vec<String>>,
}

impl Default for Extra {
    fn default() -> Self {
        Self {
            namespace: Some("local".to_string()),
            dependencies: None,
            exclude: None,
        }
    }
}

impl Extra {
    pub fn new(
        namespace: Option<String>,
        dependencies: Option<Vec<String>>,
        exclude: Option<Vec<String>>,
    ) -> Self {
        Self {
            namespace,
            dependencies,
            exclude,
        }
    }
}

impl From<Option<Tool>> for Extra {
    fn from(op_tool: Option<Tool>) -> Self {
        match op_tool {
            Some(tool) => tool
                .get_section("utpm")
                .unwrap_or(Some(Extra::default()))
                .unwrap_or(Extra::default()),
            None => Extra::default(),
        }
    }
}
