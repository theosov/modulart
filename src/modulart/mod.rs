mod grid;

use grid::{
    Grid,
};
use iced::widget::container;
use iced::widget::column;
use iced::{executor, Theme};
use iced::{
    Application,
    Command,
    Element,
    Length,
};

#[derive(Default)]
pub struct Modulart {
    grid: Grid,
    version: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    Grid(grid::Message, usize),
}

impl Application for Modulart {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                ..Self::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Modulart")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let version = self.version;
        let content = column![
            self.grid.view().map(move |message| Message::Grid(message, version)),
        ];

        container(content).width(Length::Fill).height(Length::Fill).into()
    }
}
