mod modulart;

use modulart::Modulart;
use iced::{self, window, Application};
use iced::Settings;

fn main() -> iced::Result {
    env_logger::builder().format_timestamp(None).init();

    Modulart::run(Settings {
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
