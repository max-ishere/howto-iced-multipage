use super::first::FirstPage;
use crate::{App, Message};
use iced::{widget::column, widget::*, Element};

#[derive(Debug, Clone)]
pub struct SecondPage(pub u8);

impl SecondPage {
    pub fn view(&self) -> Element<Message> {
        column!(
            text(format!("Second page, number is {}", self.0)),
            button("Go back").on_press(Message::SwitchPage(App::FirstPage(FirstPage(1))))
        )
        .into()
    }
}
