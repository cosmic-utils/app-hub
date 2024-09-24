use cosmic::Element;

use crate::{app, fl, pages};

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub enum NavPage {
    #[default]
    InstallFromFile,
    InstalledList,
    Settings,
}

impl Default for &NavPage {
    fn default() -> Self {
        &NavPage::InstallFromFile
    }
}

impl NavPage {
    pub fn title(&self) -> String {
        match self {
            Self::InstallFromFile => fl!("install-from-file"),
            Self::InstalledList => fl!("installed-list"),
            Self::Settings => fl!("settings"),
        }
    }

    pub fn view<'a>(&self, app: &'a app::AppModel) -> Element<'a, app::Message> {
        match self {
            NavPage::InstallFromFile => app
                .install_from_file
                .view()
                .map(app::Message::InstallFromFile),
            NavPage::InstalledList => app.installed_list.view().map(app::Message::InstalledList),
            NavPage::Settings => app.settings.view().map(app::Message::Settings),
        }
    }
    
    pub fn all() -> &'static [Self] {
        &[Self::InstallFromFile, Self::InstalledList, Self::Settings]
    }
}
