use iced::{Color, Theme, widget::container};

pub fn content(_theme: &Theme) -> container::Style {
    container::Style::default().background(Color::from_rgb8(230, 240, 250))
}
