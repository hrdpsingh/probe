use iced::{
    Border, Color, Theme,
    border::Radius,
    widget::{button, container},
};

pub fn sidebar(_theme: &Theme) -> container::Style {
    container::Style::default()
        .background(Color::from_rgb8(240, 245, 250))
        .color(Color::from_rgb(0.0, 0.0, 0.0))
}

pub fn sidebar_button(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Active => button::Style {
            background: Some(Color::from_rgb8(240, 245, 250).into()),
            text_color: Color::from_rgb8(0, 0, 0),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(10.0),
            },
            ..button::Style::default()
        },
        button::Status::Hovered => button::Style {
            background: Some(Color::from_rgb8(230, 240, 250).into()),
            text_color: Color::from_rgb8(0, 0, 0),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(10.0),
            },
            ..button::Style::default()
        },
        button::Status::Pressed => button::Style {
            background: Some(Color::from_rgb8(230, 240, 250).into()),
            text_color: Color::from_rgb8(0, 0, 0),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(10.0),
            },
            ..button::Style::default()
        },
        button::Status::Disabled => todo!(),
    }
}
