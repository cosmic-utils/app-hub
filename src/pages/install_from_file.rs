use cosmic::dialog::ashpd::url::Url;
use cosmic::dialog::file_chooser::{self, FileFilter};
use cosmic::iced::Length;
use cosmic::widget::vertical_space;
use cosmic::{
    iced::{self},
    widget::{self, column},
    Command, Element,
};
use std::io::{BufRead, BufReader};
use std::process::Stdio;
use std::sync::Arc;

use crate::config::Config;
use crate::{app, fl};

#[derive(Debug, Clone)]
pub enum InstallFromFileMessage {
    ChooseFile,
    FileSelected(Url),
    InstallationSuccessful,
    Cancelled,
    OpenError(Arc<file_chooser::Error>),
    NoSandboxSelected(bool),
}

pub struct InstallFromFile {
    loading: bool,
    successful: bool,
    no_sandbox: bool,
}

impl Default for InstallFromFile {
    // Initialize default
    fn default() -> Self {
        log::info!("creating a default page");
        Self {
            loading: false,
            successful: false,
            no_sandbox: false,
        }
    }
}

impl InstallFromFile {
    pub fn view<'a>(&self) -> Element<'a, InstallFromFileMessage> {
        let mut col = column::<InstallFromFileMessage>().push(
            widget::container(widget::text::title1(fl!("install-from-file")))
                .width(iced::Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Center),
        );

        if self.loading {
            col = col.push(
                widget::container(cosmic::widget::text::text(fl!("installing")))
                    .width(iced::Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center),
            );
        } else {
            col = col.push(widget::vertical_space(Length::from(30)));
            col = col.push(
                widget::container(
                    cosmic::widget::button::text(fl!("choose-file"))
                        .style(widget::button::Style::Suggested)
                        .on_press(InstallFromFileMessage::ChooseFile),
                )
                .width(iced::Length::Fill)
                .align_x(iced::alignment::Horizontal::Center),
            );
            col = col.push(widget::vertical_space(Length::from(30)));
            col = col.push(
                widget::container(cosmic::widget::checkbox(
                    "No sandbox",
                    self.no_sandbox,
                    InstallFromFileMessage::NoSandboxSelected,
                ))
                .width(iced::Length::Fill)
                .align_x(iced::alignment::Horizontal::Center),
            );
        }

        if self.successful {
            col = col.push(vertical_space(Length::from(30))).push(
                widget::container(cosmic::widget::text::text(fl!("installation-completed")))
                    .width(iced::Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center),
            );
        }

        widget::container(col)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_y()
            .center_x()
            .into()
    }

    pub fn update(&mut self, message: InstallFromFileMessage) -> Command<crate::app::Message> {
        let mut commands = vec![];
        match message {
            InstallFromFileMessage::ChooseFile => {
                log::info!("Choosing .appImage file to install");

                self.loading = true;
                log::info!("loading after: {}", self.loading);

                commands.push(cosmic::command::future(async move {
                    log::info!("opening new dialog");

                    #[cfg(feature = "rfd")]
                    let filter = FileFilter::new("AppImage files").extension("AppImage");

                    #[cfg(feature = "xdg-portal")]
                    let filter = FileFilter::new("AppImage files").glob("*.AppImage");

                    let dialog = file_chooser::open::Dialog::new()
                        .title("Choose a file")
                        .filter(filter);

                    match dialog.open_file().await {
                        Ok(response) => {
                            //InstallFromFileMessage::FileSelected(response.url().to_owned())

                            app::Message::ChooseFile(response.url().clone())
                        }

                        Err(file_chooser::Error::Cancelled) => app::Message::Cancelled,

                        Err(why) => app::Message::OpenError(Arc::new(why)),
                    }
                }));
            }
            InstallFromFileMessage::FileSelected(url) => {
                log::info!("requested to install {}", url);

                match url.to_file_path() {
                    Ok(path) => {
                        let config = Config::load();
                        let installation_dir = config.1.installation_dir;
                        log::info!("Using installation dir : {:?}", installation_dir);
                        //let result = install_app_image(path, installation_dir, false);
                        //log::info!("result from install: {:?}", result);

                        let current_exe =
                            std::env::current_exe().map_err(|_| "unable to get current exe");
                        match current_exe {
                            Ok(current_exe) => {
                                let cmd = std::process::Command::new("pkexec")
                                    .arg(current_exe.parent().unwrap().join("app_hub_backend"))
                                    .arg("--action")
                                    .arg("install")
                                    .arg("--file-path")
                                    .arg(path)
                                    .arg("--install-dir")
                                    .arg(installation_dir)
                                    .arg("--no-sandbox")
                                    .arg(self.no_sandbox.to_string())
                                    .stdout(Stdio::piped())
                                    .stderr(Stdio::piped())
                                    .spawn();

                                match cmd {
                                    Ok(mut child) => {
                                        // Read stdout
                                        if let Some(stdout) = child.stdout.take() {
                                            let reader = BufReader::new(stdout);
                                            for line in reader.lines() {
                                                if let Ok(line) = line {
                                                    log::debug!("app_hub_backend output: {}", line);
                                                } else {
                                                    log::error!("Failed to read line from stdout");
                                                }
                                            }
                                        }

                                        // Read stderr
                                        if let Some(stderr) = child.stderr.take() {
                                            let reader = BufReader::new(stderr);
                                            for line in reader.lines() {
                                                if let Ok(line) = line {
                                                    log::error!("app_hub_backend error: {}", line);
                                                } else {
                                                    log::error!("Failed to read line from stderr");
                                                }
                                            }
                                        }

                                        let output = child
                                            .wait_with_output()
                                            .expect("Failed to wait on child");
                                        log::debug!("output: {:?}", output);
                                        if output.status.success() {
                                            log::info!("Installation successful");
                                            self.successful = true;
                                            commands.push(Command::perform(async {}, |_| {
                                                app::Message::InstallationSuccessful
                                            }));
                                            commands.push(Command::perform(async {}, |_| {
                                                app::Message::LoadApps
                                            }));
                                        } else {
                                            log::error!("Installation failed");
                                            //return Err("Installation failed".to_string());
                                        }
                                    }
                                    Err(error) => {
                                        log::error!("error: {:?}", error);
                                        //return Err("Failed to install AppImage".to_string());
                                    }
                                }
                            }
                            Err(e) => {
                                log::error!("current_exe error: {:?}", e);
                            }
                        };
                    }
                    Err(e) => {
                        log::error!("error converting url to file path: {:?}", e);
                        //TODO show error message
                    }
                };
            }
            InstallFromFileMessage::Cancelled => {
                self.loading = false;
            }
            InstallFromFileMessage::OpenError(arc) => {
                //TODO sho error message
                self.loading = false;
            }
            InstallFromFileMessage::InstallationSuccessful => {
                self.loading = false;
            }
            InstallFromFileMessage::NoSandboxSelected(selected) => {
                self.no_sandbox = selected;
                log::info!("no sandbox: {}", self.no_sandbox);
            }
        }
        Command::batch(commands)
    }
}
