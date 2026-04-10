use crate::{app::Message, styles, types::Page};
use iced::{
    Element, Length,
    widget::{button, column, container},
};

pub fn view<'a>() -> Element<'a, Message> {
    container(
        column![
            button("Overview")
                .on_press(Message::Navigate(Page::Overview))
                .padding(8)
                .style(styles::sidebar_button),
            button("Memory")
                .on_press(Message::Navigate(Page::Memory))
                .padding(8)
                .style(styles::sidebar_button),
        ]
        .spacing(10),
    )
    .width(Length::Shrink)
    .height(Length::Fill)
    .padding(10)
    .style(styles::sidebar)
    .into()
}
