use crate::{app::Message, components::styles, models::Page};
use iced::{
    Element, Length,
    widget::{button, column, container},
};

pub fn view<'a>(page: Page) -> Element<'a, Message> {
    container(
        column![
            button("Overview")
                .on_press(Message::Navigate(Page::Overview))
                .padding(8)
                .width(Length::Fill)
                .style(move |theme, status| {
                    styles::sidebar_button(theme, status, page == Page::Overview)
                }),
            button("Memory")
                .on_press(Message::Navigate(Page::Memory))
                .padding(8)
                .width(Length::Fill)
                .style(move |theme, status| {
                    styles::sidebar_button(theme, status, page == Page::Memory)
                }),
            button("CPU")
                .on_press(Message::Navigate(Page::Cpu))
                .padding(8)
                .width(Length::Fill)
                .style(move |theme, status| {
                    styles::sidebar_button(theme, status, page == Page::Cpu)
                }),
            button("Battery")
                .on_press(Message::Navigate(Page::Battery))
                .padding(8)
                .width(Length::Fill)
                .style(move |theme, status| {
                    styles::sidebar_button(theme, status, page == Page::Battery)
                }),
            button("Storage")
                .on_press(Message::Navigate(Page::Storage))
                .padding(8)
                .width(Length::Fill)
                .style(move |theme, status| {
                    styles::sidebar_button(theme, status, page == Page::Storage)
                }),
        ]
        .spacing(10),
    )
    .width(120)
    .height(Length::Fill)
    .padding(10)
    .style(styles::sidebar)
    .into()
}
