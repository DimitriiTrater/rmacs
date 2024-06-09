use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::filesystem::{default_file, load_file, pick_file, Error};
use iced::{
    executor,
    widget::{button, column, container, horizontal_space, row, text, text_editor},
    Application, Command, Theme,
};

pub struct Editor {
    path: Option<PathBuf>,
    content: text_editor::Content,
    error: Option<Error>,
}

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    Open,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
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
            Message::Edit(action) => {
                self.content.perform(action);

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
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let controls = row![button("Open").on_press(Message::Open)];

        let file_path = match self.path.as_deref().and_then(Path::to_str) {
            Some(path) => text(path).size(14),
            None => text(""),
        };

        let input = text_editor(&self.content)
            .height(400)
            .on_action(Message::Edit);

        let position = {
            let (line, column) = self.content.cursor_position();

            text(format!("{}:{}", line + 1, column + 1))
        };

        let status_bar = row![file_path, horizontal_space(), position];

        container(column![controls, input, status_bar].spacing(10))
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
