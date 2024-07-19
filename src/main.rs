pub mod application;
pub mod buttons;
pub mod filesystem;
pub mod icons;

use crate::application::Editor;
use iced::Application;
use iced::Font;
use iced::Settings;

fn main() -> iced::Result {
    Editor::run(Settings {
        fonts: vec![include_bytes!("../fonts/symbols.ttf").as_slice().into()],
        ..Default::default()
    })
}
