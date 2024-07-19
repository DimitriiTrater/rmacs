use iced::{widget::text, Font};

use crate::application::Message;

pub fn new_icon<'a>() -> iced::Element<'a, Message> {
    icon('\u{E800}')
}

pub fn open_icon<'a>() -> iced::Element<'a, Message> {
    icon('\u{F115}')
}

pub fn save_icon<'a>() -> iced::Element<'a, Message> {
    icon('\u{E801}')
}

fn icon<'a, Message>(codepoint: char) -> iced::Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("symbols");
    println!("{:?}", ICON_FONT);

    text(codepoint).font(ICON_FONT).into()
}
