
pub struct DesktopFileBuilder {
    /// The `type_` field represents the type of the application. It's usually "Application" for desktop applications.
    type_: Option<String>,

    /// The `version` field represents the version of the desktop file format.
    version: Option<String>,

    /// The `name` field represents the name of the application.
    name: Option<String>,

    /// The `comment` field represents a short comment about the application.
    comment: Option<String>,

    /// The `path` field represents the path of the application on the file system.
    path: Option<String>,

    /// The `exec` field represents the command to execute to start the application.
    exec: Option<String>,

    /// The `icon` field represents the path of the application's icon.
    icon: Option<String>,

    /// The `terminal` field indicates whether the application should be run in a terminal.
    terminal: Option<bool>,

    /// The `categories` field represents the categories the application belongs to.
    categories: Option<String>,

    /// The `no_sandbox` field indicates whether the application should be run in a sandbox.
    no_sanbox: Option<bool>,
}

impl DesktopFileBuilder {

    pub fn new() -> Self {
        Self {
            type_: None,
            version: None,
            name: None,
            comment: None,
            path: None,
            exec: None,
            icon: None,
            terminal: None,
            categories: None,
            no_sanbox: None,
        }
    }

    // Mandatory fields
    pub fn set_type(&mut self, type_: String) -> &mut Self {
        self.type_ = Some(type_);
        self
    }

    // Mandatory fields
    pub fn set_version(&mut self, version: String) -> &mut Self {
        self.version = Some(version);
        self
    }

    // Mandatory fields
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn set_comment(&mut self, comment: String) -> &mut Self {
        self.comment = Some(comment);
        self
    }

    pub fn set_path(&mut self, path: String) -> &mut Self {
        self.path = Some(path);
        self
    }

    // Mandatory fields
    pub fn set_exec(&mut self, exec: String) -> &mut Self {
        self.exec = Some(exec);
        self
    }

    pub fn set_icon(&mut self, icon: String) -> &mut Self {
        self.icon = Some(icon);
        self
    }

    pub fn set_terminal(&mut self, terminal: bool) -> &mut Self {
        self.terminal = Some(terminal);
        self
    }

    pub fn set_categories(&mut self, categories: Vec<String>) -> &mut Self {
        let categories = categories.join(";");
        self.categories = Some(categories);
        self
    }

    pub fn set_no_sandbox(&mut self, no_sandbox: bool) -> &mut Self {
        self.no_sanbox = Some(no_sandbox);
        self
    }

    pub fn write_to_file(self, path: String) -> Result<String, String> {
        // Check mandatory fields
        if self.type_.is_none() {
            return Err("Type field is mandatory".to_string());
        }
        if self.version.is_none() {
            return Err("Version field is mandatory".to_string());
        }
        if self.name.is_none() {
            return Err("Name field is mandatory".to_string());
        }
        if self.exec.is_none() {
            return Err("Exec field is mandatory".to_string());
        }

        // Create the file content
        let mut file_content = String::from("[Desktop Entry]\n");

        if let Some(type_) = self.type_ {
            file_content.push_str(&format!("Type={}\n", type_));
        }

        if let Some(version) = self.version {
            file_content.push_str(&format!("Version={}\n", version));
        }

        if let Some(name) = self.name {
            file_content.push_str(&format!("Name={}\n", name));
        }

        if let Some(comment) = self.comment {
            file_content.push_str(&format!("Comment={}\n", comment));
        }

        if let Some(path) = self.path {
            file_content.push_str(&format!("Path={}\n", path));
        }

        if let Some(exec) = self.exec {
            if self.no_sanbox.is_some() && self.no_sanbox.unwrap() {
                file_content.push_str(&format!("Exec={} --no-sandbox\n", exec));
            }
            else {
                file_content.push_str(&format!("Exec={}\n", exec));
            }
        }

        if let Some(icon) = self.icon {
            file_content.push_str(&format!("Icon={}\n", icon));
        }

        if let Some(terminal) = self.terminal {
            file_content.push_str(&format!("Terminal={}\n", terminal));
        }

        if let Some(categories) = self.categories {
            file_content.push_str(&format!("Categories={}\n", categories));
        }

        // Write the file
        match std::fs::write(path, file_content) {
            Ok(_) => {
                Ok("Desktop file written successfully".to_string())
            },
            Err(e) => {
                eprintln!("Failed to write to file: {}", e);
                return Err("Failed to write .desktop file".to_string());
            }
        }

    }

}
