use iced::{widget::Theme, Element, Sandbox, Settings};
use page::{
    first::{FirstPage, FirstPageMessage},
    second::SecondPage,
};

mod page {
    pub mod first;
    pub mod second;
}

fn main() -> iced::Result {
    App::run(Settings::default())
}

#[derive(Debug, Clone)]
pub enum App {
    FirstPage(FirstPage),
    SecondPage(SecondPage),
}

#[derive(Debug, Clone)]
pub enum Message {
    FirstPage(FirstPageMessage),
    SwitchPage(App),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self::FirstPage(FirstPage(1))
    }

    fn title(&self) -> String {
        "Two page app".to_string()
    }

    fn view(&self) -> Element<Self::Message> {
        match self {
            Self::FirstPage(page) => page.view(),
            Self::SecondPage(page) => page.view(),
        }
    }

    fn update(mut self: &mut Self, message: Self::Message) {
        match (&mut self, message) {
            (_, Message::SwitchPage(new)) => *self = new,

            (App::FirstPage(page), Message::FirstPage(message)) => page.update(message),

            (page, message) => panic!(
                "Incorrect message routing:\n\
                page {page:?}\n\
                recieved message {message:?}"
            ),
        }
    }

    fn theme(&self) -> Theme {
        Theme::CatppuccinMocha
    }
}
