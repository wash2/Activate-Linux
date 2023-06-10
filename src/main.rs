mod localize;

use std::collections::HashMap;

use cosmic::{
    executor,
    iced::{
        theme,
        widget::{column, text},
        window, Application, Color, Command,
    },
    iced_sctk::{
        commands::layer_surface::{self, Anchor, Layer},
        settings::InitialSurface,
    },
    iced_style::{application, Theme},
    settings,
};
use localize::localize;
use log::info;

fn main() {
    // Initialize logger
    pretty_env_logger::init();
    info!("Starting ActivateLinux");
    localize();

    let mut settings = settings();
    settings.id = Some("ActivateLinux".to_string());
    settings.initial_surface = InitialSurface::LayerSurface(
        cosmic::iced::wayland::actions::layer_surface::SctkLayerSurfaceSettings {
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
    );

    Activate::run(settings).expect("Failed to run ActivateLinux");
}

struct Activate {
    distro: String,
}

impl Application for Activate {
    type Executor = executor::Default;

    type Message = ();

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let distro = if let Ok(distro) = os_release::OsRelease::new() {
            distro.name
        } else {
            "Linux".to_string()
        };

        (Self { distro }, cosmic::iced::Command::none())
    }

    fn title(&self) -> String {
        let distro_var = HashMap::from([("distro", self.distro.clone())]);
        fl!("activate", distro_var)
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        cosmic::iced::Command::none()
    }

    fn view(
        &self,
        _id: window::Id,
    ) -> cosmic::iced::Element<'_, Self::Message, cosmic::iced::Renderer<Self::Theme>> {
        let distro_var = HashMap::from([("distro", self.distro.clone())]);
        column![
            text(fl!("activate", distro_var.clone()))
                .size(24)
                .style(theme::Text::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.5))),
            text(fl!("go-to-settings", distro_var))
                .size(16)
                .style(theme::Text::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.5))),
        ]
        .spacing(4)
        .into()
    }

    fn close_requested(&self, _id: window::Id) -> Self::Message {
        info!("Exiting ActivateLinux");
        std::process::exit(0);
    }

    fn style(&self) -> <Self::Theme as application::StyleSheet>::Style {
        <Self::Theme as application::StyleSheet>::Style::Custom(Box::new(CustomStyleSheet))
    }
}

pub struct CustomStyleSheet;
impl application::StyleSheet for CustomStyleSheet {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        application::Appearance {
            background_color: Color::from_rgba(0.0, 0.0, 0.0, 0.0),
            text_color: Color::from_rgba(1.0, 1.0, 1.0, 0.5),
        }
    }
}
