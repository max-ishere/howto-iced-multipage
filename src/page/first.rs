use super::second::SecondPage;
use crate::{App, Message};
use iced::{widget::column, widget::*, Element};

#[derive(Debug, Clone)]
pub struct FirstPage(pub u8);

#[derive(Debug, Clone)]
pub enum FirstPageMessage {
    IncrementButton,
}

impl FirstPage {
    pub fn view(&self) -> Element<Message> {
        column! {
            text(format!("First page (number is {})", self.0)),
            button("Increment").on_press(FirstPageMessage::IncrementButton.into()),
            button("Go forward").on_press(Message::SwitchPage(App::SecondPage(SecondPage(self.0))))
        }
        .into()
    }

    pub(crate) fn update(&mut self, message: FirstPageMessage) {
        match message {
            FirstPageMessage::IncrementButton => self.0 += 1,
        }
    }
}

impl From<FirstPageMessage> for Message {
    fn from(message: FirstPageMessage) -> Self {
        Self::FirstPage(message)
    }
}
