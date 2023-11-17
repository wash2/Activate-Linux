mod localize;

use std::{collections::HashMap, path::Path};

use cosmic::{
    app::{Command, Core, Settings},
    iced::{
        widget::{column, text},
        Color,
    },
    iced_sctk::commands::layer_surface::{self, Anchor, Layer},
    iced_style::application::{self, Appearance},
    Theme,
};
use localize::localize;
use log::info;

fn main() {
    // Initialize logger
    pretty_env_logger::init();
    info!("Starting ActivateLinux");
    localize();

    cosmic::app::run::<Activate>(
        Settings::default()
            .antialiasing(true)
            .client_decorations(false)
            .debug(false)
            .default_text_size(16.0)
            .scale_factor(1.0)
            .no_main_window(true)
            .exit_on_close(true),
        (),
    )
    .expect("Failed to run Activate Linux");
}

struct Activate {
    core: Core,
    distro: String,
}

impl cosmic::Application for Activate {
    type Message = ();
    type Executor = cosmic::executor::single::Executor;
    type Flags = ();
    const APP_ID: &'static str = "com.system76.CosmicLauncher";

    fn init(core: Core, _flags: Self::Flags) -> (Self, Command<()>) {
        let distro = if let Some(Ok(distro)) = Path::new("/.flatpak-info")
            .exists()
            .then(|| os_release::OsRelease::new_from(Path::new("/var/run/host/etc/os-release")))
        {
            distro.name
        } else if let Ok(distro) = os_release::OsRelease::new() {
            distro.name
        } else {
            "Linux".to_string()
        };

        (
            Self { distro, core },
            cosmic::iced_sctk::commands::layer_surface::get_layer_surface(
                cosmic::iced::wayland::actions::layer_surface::SctkLayerSurfaceSettings {
                    id: cosmic::iced::window::Id(1),
                    layer: Layer::Overlay,
                    anchor: Anchor::BOTTOM.union(Anchor::RIGHT),
                    output: cosmic::iced::wayland::actions::layer_surface::IcedOutput::All,
                    margin: cosmic::iced::wayland::actions::layer_surface::IcedMargin {
                        top: 0,
                        right: 36,
                        bottom: 24,
                        left: 0,
                    },
                    keyboard_interactivity: layer_surface::KeyboardInteractivity::None,
                    exclusive_zone: -1,
                    pointer_interactivity: false,
                    ..Default::default()
                },
            ),
        )
    }

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn style(&self) -> Option<<Theme as application::StyleSheet>::Style> {
        Some(<Theme as application::StyleSheet>::Style::Custom(Box::new(
            |theme| {
                let mut text = theme.cosmic().on_bg_color();
                text.alpha = 0.5;
                let text = text.into();

                Appearance {
                    background_color: Color::from_rgba(0.0, 0.0, 0.0, 0.0),
                    text_color: text,
                    icon_color: text,
                }
            },
        )))
    }

    fn view(&self) -> cosmic::Element<Self::Message> {
        unimplemented!()
    }

    fn view_window(&self, _id: cosmic::iced::window::Id) -> cosmic::Element<Self::Message> {
        let distro_var = HashMap::from([("distro", self.distro.clone())]);
        column![
            text(fl!("activate", distro_var.clone())).size(24),
            text(fl!("go-to-settings", distro_var)).size(16)
        ]
        .spacing(4)
        .into()
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        cosmic::iced::Command::none()
    }
}
