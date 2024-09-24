use std::path::PathBuf;

use cosmic::{
    iced::{self, Length},
    widget::{self, column},
    Command, Element,
};

use crate::{config::Config, fl};

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    InstallationDirChanged(String),
    Submitted,
}

pub struct Settings {
    installation_dir_value: String,
}

impl Default for Settings {
    // Initialize default
    fn default() -> Self {
        let config = Config::load();
        Self {
            installation_dir_value: config.1.installation_dir.to_string_lossy().to_string(),
        }
    }
}

impl Settings {
    pub fn view<'a>(&'a self) -> Element<'a, SettingsMessage> {
        let mut col = column::<SettingsMessage>().push(widget::text::title1(fl!("settings")));

        col = col.push(widget::vertical_space(Length::from(20)));
        
        col = col.push(widget::text::text(fl!("installation-dir")));

        col = col.push(
            widget::container(
                cosmic::widget::text_input("placeholder", &self.installation_dir_value)
                    .on_input(SettingsMessage::InstallationDirChanged),
            )
            .width(iced::Length::Fill)
            .align_x(iced::alignment::Horizontal::Right),
        );

        col = col.push(widget::vertical_space(Length::from(20)));

        col = col.push(
            widget::container(
                cosmic::widget::button::text(fl!("save-settings"))
                    .style(widget::button::Style::Standard)
                    .on_press(SettingsMessage::Submitted),
            )
            .width(iced::Length::Fill)
            .align_x(iced::alignment::Horizontal::Left),
        );

        widget::container(col)
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .center_y()
            .into()
    }

    pub fn update(&mut self, message: SettingsMessage) -> Command<crate::app::Message> {
        match message {
            SettingsMessage::InstallationDirChanged(installation_dir) => {
                self.installation_dir_value = installation_dir;
                log::info!("new installation dir: {}", self.installation_dir_value);
            }
            SettingsMessage::Submitted => {
                log::info!("setting change submited");
                let mut config = Config::load();
                config.1.set_installation_dir(
                    &config.0.unwrap(),
                    PathBuf::from(&self.installation_dir_value),
                );
            }
        }
        Command::none()
    }
}
