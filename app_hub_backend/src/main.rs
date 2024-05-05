use std::fs::File;
use std::io::Write;
use clap::Parser;
use log::{error, info};
use serde::Serialize;

#[derive(
    clap::ValueEnum, Clone, Default, Debug, Serialize,
)]
#[serde(rename_all = "kebab-case")]
enum Action {
    #[default]
    Install,
    Uninstall
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// AppImage file path to install
    #[arg(short, long)]
    file_path: Option<String>,

    /// Action to perform
    #[arg(short, long, default_value_t, value_enum, requires_if("install", "file_path"), requires_if("uninstall", "uninstall_app_name"))]
    action: Action,

    /// App name to uninstall
    #[arg(short, long)]
    uninstall_app_name: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.action {
        Action::Install => {
            let file_path = args.file_path.as_ref().ok_or("file_path is required")?;
            info!("Installing AppImage: {}", file_path);
        }
        Action::Uninstall => {
            let app_name = args.uninstall_app_name.as_ref().ok_or("uninstall_app_name is required")?;
            info!("Uninstalling AppImage with app name: {}", app_name);
        }
    }

    // write a file to /usr/share/applications/test.txt using rust
    let mut file = File::create("/usr/share/applications/test.txt")?;
    file.write_all(b"Hello, world!")?;

    Ok(())
}
