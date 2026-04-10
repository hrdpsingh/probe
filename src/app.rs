use crate::{
    components::sidebar,
    models::Page,
    pages::{battery, cpu, memory, overview, storage},
    styles,
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
            Page::Storage => storage::view(),
            Page::Cpu => cpu::view(),
            Page::Battery => battery::view(),
        };

        row![
            sidebar::view(self.page),
            container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .style(styles::content)
        ]
        .into()
    }
}
