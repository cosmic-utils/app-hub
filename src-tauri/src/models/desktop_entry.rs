use serde::Serialize;

#[derive(Serialize)]
pub struct DesktopEntry {
    #[serde(rename = "Type")]
    pub(crate) type_: String,
    #[serde(rename = "Version")]
    pub(crate) version: String,
    #[serde(rename = "Name")]
    pub(crate) name: String,
    #[serde(rename = "Comment")]
    pub(crate) comment: String,
    #[serde(rename = "Path")]
    pub(crate) path: String,
    #[serde(rename = "Exec")]
    pub(crate) exec: String,
    #[serde(rename = "Icon")]
    pub(crate) icon: String,
    #[serde(rename = "Terminal")]
    pub(crate) terminal: bool,
    #[serde(rename = "Categories")]
    pub(crate) categories: String,
}

impl DesktopEntry {
    pub fn new(
        type_: String,
        version: String,
        name: String,
        comment: String,
        path: String,
        exec: String,
        icon: String,
        terminal: bool,
        categories: Vec<String>,
    ) -> Self {

        let categories = categories.join(";");

        Self {
            type_,
            version,
            name,
            comment,
            path,
            exec,
            icon,
            terminal,
            categories,
        }
    }
}