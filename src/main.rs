mod app;
mod components;
mod pages;
mod styles;
mod types;

use app::Probe;

fn main() -> iced::Result {
    iced::run(Probe::update, Probe::view)
}
