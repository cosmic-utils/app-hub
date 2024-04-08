use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestInstallation {
    pub file_path: String,
    pub no_sandbox: Option<bool>,
}
