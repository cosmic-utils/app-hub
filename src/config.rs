// SPDX-License-Identifier: {{LICENSE}}

use std::{any::TypeId, path::PathBuf};

use cosmic::Application;
use cosmic::{
    cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, CosmicConfigEntry},
    iced::Subscription,
};
use serde::{Deserialize, Serialize};

pub const CONFIG_VERSION: u64 = 1;

#[derive(Clone, CosmicConfigEntry, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Config {
    pub installation_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let installation_dir = PathBuf::from(format!(
            "{}/AppImages/",
            dirs::home_dir().unwrap().to_string_lossy()
        ));

        Self { installation_dir }
    }
}

impl Config {
    pub fn load() -> (Option<cosmic_config::Config>, Self) {
        match cosmic_config::Config::new(crate::app::AppModel::APP_ID, CONFIG_VERSION) {
            Ok(config_handler) => {
                let config = match Config::get_entry(&config_handler) {
                    Ok(ok) => ok,
                    Err((errs, config)) => {
                        log::info!("errors loading config: {:?}", errs);
                        config
                    }
                };
                (Some(config_handler), config)
            }
            Err(err) => {
                log::error!("failed to create config handler: {}", err);
                (None, Config::default())
            }
        }
    }

    pub fn subscription() -> Subscription<cosmic_config::Update<Self>> {
        struct ConfigSubscription;
        cosmic_config::config_subscription(
            TypeId::of::<ConfigSubscription>(),
            crate::app::AppModel::APP_ID.into(),
            CONFIG_VERSION,
        )
    }
}
