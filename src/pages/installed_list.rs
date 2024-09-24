use std::io::{BufRead, BufReader};

use common_utils::{app_images_helpers::read_all_app, models::installed_app::InstalledApp};
use cosmic::{
    iced::{self, Length, Padding},
    widget::{self, column},
    Command, Element,
};

use crate::{app, fl};

#[derive(Debug, Clone)]
pub enum InstalledListMessage {
    UninstallApp(InstalledApp),
    UninstallationComplete,
    LoadApps,
}

pub struct InstalledList {
    installed_apps: Vec<InstalledApp>,
}

impl Default for InstalledList {
    // Initialize default
    fn default() -> Self {
        let apps = read_all_app();
        Self {
            installed_apps: if apps.is_ok() {
                apps.unwrap()
            } else {
                Vec::new()
            },
        }
    }
}

impl InstalledList {
    pub fn load_apps(&mut self) {
        let apps = read_all_app();
        self.installed_apps = if apps.is_ok() {
            apps.unwrap()
        } else {
            Vec::new()
        }
    }

    pub fn view<'a>(&'a self) -> Element<'a, InstalledListMessage> {
        let mut col =
            column::<InstalledListMessage>().push(widget::text::title1(fl!("installed-list")));

        col = col.push(widget::vertical_space(Length::from(20)));

        for app in &self.installed_apps {
            col = col.push(
                widget::container(
                    widget::row()
                        .push(
                            widget::column()
                                .push(
                                    widget::row()
                                        .push(
                                            widget::image::Image::new(
                                                app.icon_path.clone().unwrap(),
                                            )
                                            .width(Length::from(50)),
                                        )
                                        .push(
                                            widget::text::title3(&app.name)
                                                .vertical_alignment(
                                                    iced::alignment::Vertical::Center,
                                                )
                                                .height(Length::Fill),
                                        ),
                                )
                                .width(Length::Fill),
                        )
                        .push(
                            widget::column()
                                .push(
                                    cosmic::widget::button::text(fl!("uninstall"))
                                        .on_press(InstalledListMessage::UninstallApp(app.clone()))
                                        .style(widget::button::Style::Destructive),
                                )
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .align_items(iced::Alignment::End),
                        )
                        .height(Length::from(50)),
                )
                .padding(Padding::from(10))
                .style(cosmic::theme::Container::Card)
                .width(iced::Length::Fill)
                .align_x(iced::alignment::Horizontal::Left),
            );
            col = col.push(widget::vertical_space(Length::from(10)));
        }

        widget::container(col)
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .center_y()
            .into()
    }

    pub fn update(&mut self, message: InstalledListMessage) -> Command<crate::app::Message> {
        let mut commands = Vec::new();
        match message {
            InstalledListMessage::UninstallApp(app) => {
                log::info!("uninstall app: {:?}", app);

                let current_exe = std::env::current_exe().map_err(|_| "unable to get current exe");
                match current_exe {
                    Ok(current_exe) => {
                        let cmd = std::process::Command::new("pkexec")
                            .arg(current_exe.parent().unwrap().join("app_hub_backend"))
                            .arg("--action")
                            .arg("uninstall")
                            .arg("--uninstall-app-name")
                            .arg(app.name)
                            .spawn();
                        match cmd {
                            Ok(mut res) => {
                                // Leggi lo stdout
                                if let Some(stdout) = res.stdout.take() {
                                    let reader = BufReader::new(stdout);
                                    for line in reader.lines() {
                                        if let Ok(line) = line {
                                            log::debug!("app_hub_backend output: {}", line);
                                        }
                                    }
                                }

                                // Leggi lo stderr
                                if let Some(stderr) = res.stderr.take() {
                                    let reader = BufReader::new(stderr);
                                    for line in reader.lines() {
                                        if let Ok(line) = line {
                                            log::error!("app_hub_backend error: {}", line);
                                        }
                                    }
                                }

                                let output =
                                    res.wait_with_output().expect("Failed to wait on child");
                                if output.status.success() {
                                    log::info!("Uninstallation successful");

                                    commands.push(Command::perform(async {}, |_| {
                                        app::Message::UninstallationComplete
                                    }));
                                } else {
                                    log::error!("Uninstallation failed");
                                    //return Err("Uninstallation failed".to_string());
                                }
                            }
                            Err(error) => {
                                log::error!("error: {:?}", error);
                            }
                        }
                    }
                    Err(_) => todo!(),
                }
            }
            InstalledListMessage::UninstallationComplete | InstalledListMessage::LoadApps => {
                self.load_apps();
            }
        }
        Command::batch(commands)
    }
}
