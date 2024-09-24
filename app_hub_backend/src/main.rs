use backend::app_image_installer::install_app_image;
use backend::app_image_uninstaller::uninstall_app_image;
use backend::app_image_updater::app_image_update;
use clap::Parser;
use log::{error, info};
use serde::Serialize;
use std::path::PathBuf;

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Action {
    #[default]
    Install,
    Uninstall,
    Update,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// AppImage file path to install
    #[arg(short, long)]
    file_path: Option<String>,

    /// Installation directory
    /// The directory where the AppImage will be installed
    #[arg(short, long)]
    install_dir: Option<String>,

    /// Action to perform
    #[arg(
        short,
        long,
        default_value_t,
        value_enum,
        requires_if("install", "file_path"),
        requires_if("install", "install_dir"),
        requires_if("uninstall", "uninstall_app_name"),
        requires_if("update", "new_install_dir"),
        requires_if("update", "old_install_dir")
    )]
    action: Action,

    /// App name to uninstall
    #[arg(short, long)]
    uninstall_app_name: Option<String>,

    /// No sandbox flag
    #[arg(short, long)]
    no_sandbox: Option<bool>,

    /// New app images directory path
    /// The new path to the directory where the AppImages are stored
    #[arg(long)]
    new_install_dir: Option<String>,

    /// Old app images directory path
    /// The old path to the directory where the AppImages are stored
    #[arg(long)]
    old_install_dir: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    info!("Starting AppHub backend");
    let args = Args::parse();

    match args.action {
        Action::Install => {
            info!("Received install action");
            // read required arguments
            let file_path = args.file_path.as_ref().ok_or("file_path is required")?;
            let installation_dir = args.install_dir.as_ref().ok_or("install_dir is required")?;
            let no_sandbox = args.no_sandbox.unwrap_or(false);

            // install the AppImage
            if let Err(e) = install_app_image(
                PathBuf::from(file_path.clone()),
                PathBuf::from(installation_dir.clone()),
                no_sandbox,
            ) {
                error!("Failed to install AppImage: {}", e);
                return Err("Failed to install AppImage".into());
            }
            info!("Installing AppImage: {}", file_path);
        }
        Action::Uninstall => {
            // read required arguments
            let app_name = args
                .uninstall_app_name
                .as_ref()
                .ok_or("uninstall_app_name is required")?;

            // uninstall the AppImage
            if let Err(e) = uninstall_app_image(app_name.clone()) {
                error!("Failed to uninstall AppImage: {}", e);
                return Err("Failed to uninstall AppImage".into());
            }
            info!("Uninstalling AppImage with app name: {}", app_name);
        }
        Action::Update => {
            // read required arguments
            let new_install_dir = args
                .new_install_dir
                .as_ref()
                .ok_or("new_install_dir is required")?;
            let old_install_dir = args
                .old_install_dir
                .as_ref()
                .ok_or("old_install_dir is required")?;

            info!(
                "Updating desktop files with new install dir: {}",
                new_install_dir
            );

            app_image_update(old_install_dir.clone(), new_install_dir.clone())?;
        }
    }

    info!("AppHub backend finished successfully");

    Ok(())
}
