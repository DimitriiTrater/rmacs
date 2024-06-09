use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::buttons::labeled_button;
use crate::filesystem::{default_file, load_file, pick_file, save_file, Error};

use iced::{
    executor,
    highlighter::{self, Highlighter},
    widget::{column, container, horizontal_space, row, text, text_editor},
    Application, Command, Length, Theme,
};

use iced_aw::menu::{Item, Menu};

use iced_aw::{menu_bar, menu_items};

pub struct Editor {
    path: Option<PathBuf>,
    content: text_editor::Content,
    error: Option<Error>,
}

#[derive(Debug, Clone)]
pub enum Message {
    FileMenuBar(String),

    Edit(text_editor::Action),
    New,
    Open,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    Save,
    FileSaved(Result<PathBuf, Error>),
}

impl Application for Editor {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                path: None,
                content: text_editor::Content::new(),
                error: None,
            },
            Command::perform(load_file(default_file()), Message::FileOpened),
        )
    }

    fn title(&self) -> String {
        String::from("RMACS")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::FileMenuBar(_) => Command::none(),
            Message::Edit(action) => {
                self.content.perform(action);

                self.error = None;

                Command::none()
            }
            Message::New => {
                self.path = None;
                self.content = text_editor::Content::new();

                Command::none()
            }
            Message::Open => Command::perform(pick_file(), Message::FileOpened),
            Message::FileOpened(Ok((path, content))) => {
                self.path = Some(path);
                self.content = text_editor::Content::with_text(&content);

                Command::none()
            }
            Message::FileOpened(Err(error)) => {
                self.error = Some(error);

                Command::none()
            }
            Message::Save => {
                let text = self.content.text();

                Command::perform(save_file(self.path.clone(), text), Message::FileSaved)
            }
            Message::FileSaved(Ok(path)) => {
                self.path = Some(path);
                Command::none()
            }
            Message::FileSaved(Err(error)) => {
                self.error = Some(error);
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let menu_tpl = |items| Menu::new(items).max_width(180.0).offset(15.0).spacing(5.0);

        let mb = menu_bar!((
            labeled_button("File", Message::FileMenuBar("File".into())).width(Length::Shrink),
            {
                let sub = menu_tpl(menu_items!((labeled_button("New", Message::New)
                    .width(Length::Fill))(
                    labeled_button("Open", Message::Open).width(Length::Fill)
                )(
                    labeled_button("Save", Message::Save).width(Length::Fill)
                )));
                sub
            }
        ));

        let controls = row![mb];

        let input = text_editor(&self.content)
            .height(400)
            .on_action(Message::Edit)
            .highlight::<Highlighter>(
                highlighter::Settings {
                    theme: highlighter::Theme::SolarizedDark,
                    extension: self
                        .path
                        .as_ref()
                        .and_then(|path| path.extension()?.to_str())
                        .unwrap_or("rs")
                        .to_string(),
                },
                |highlight, _theme| highlight.to_format(),
            );

        let status_bar = {
            let status = if let Some(Error::IO(error)) = self.error {
                text(error.to_string())
            } else {
                match self.path.as_deref().and_then(Path::to_str) {
                    Some(path) => text(path).size(14),
                    None => text("New file"),
                }
            };

            let position = {
                let (line, column) = self.content.cursor_position();

                text(format!("{}:{}", line + 1, column + 1))
            };

            row![status, horizontal_space(), position]
        };

        container(column![controls, input, status_bar].spacing(10))
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
