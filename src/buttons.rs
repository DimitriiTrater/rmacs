use iced::{
    alignment,
    widget::{button, button::Button, text},
    Element, Renderer, Theme,
};

use crate::application::Message;

fn base_button<'a>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    msg: Message,
) -> Button<'a, Message, Theme, Renderer> {
    button(content)
        .padding([4, 8])
        .style(iced::theme::Button::Positive)
        .on_press(msg)
}

pub fn labeled_button<'a>(
    lable: &str,
    msg: Message,
) -> button::Button<'a, Message, Theme, Renderer> {
    base_button(
        text(lable).vertical_alignment(alignment::Vertical::Center),
        msg,
    )
}
