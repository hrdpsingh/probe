use crate::{
    components::sidebar,
    pages::{memory, overview},
    types::Page,
};

use iced::{
    Element, Length,
    widget::{container, row},
};

#[derive(Default)]
pub struct Probe {
    page: Page,
}

#[derive(Clone)]
pub enum Message {
    Navigate(Page),
}

impl Probe {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Navigate(page) => {
                self.page = page;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = match self.page {
            Page::Overview => overview::view(),
            Page::Memory => memory::view(),
        };

        row![
            sidebar::view(),
            container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_theme: &Theme| {
                    container::Style::default()
                        .background(Color::from_rgb8(230, 240, 250))
                })
        ]
        .into()
    }
}
