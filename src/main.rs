pub mod application;
pub mod buttons;
pub mod filesystem;

use crate::application::Editor;
use iced::Application;

fn main() -> iced::Result {
    Editor::run(iced::Settings::default())
}
