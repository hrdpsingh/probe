use iced::{
    Application, Command, Element, Settings, Theme, executor,
    widget::{column, text},
};
use nix::sys::utsname::uname;

fn main() -> iced::Result {
    ByteScope::run(Settings::default())
}

struct ByteScope {
    hostname: String,
    kernel: String,
}

#[derive(Debug, Clone)]
enum Message {}

impl Application for ByteScope {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let hostname = match hostname::get() {
            Ok(name) => name.to_string_lossy().to_string(),
            Err(_) => "Unavailable".to_string(),
        };

        let kernel = match uname() {
            Ok(info) => info.release().to_string_lossy().to_string(),
            Err(_) => "Unavailable".to_string(),
        };

        (Self { hostname, kernel }, Command::none())
    }

    fn title(&self) -> String {
        String::from("ByteScope")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            text(format!("Hostname: {}", self.hostname)),
            text(format!("Kernel: {}", self.kernel)),
        ]
        .padding(20)
        .spacing(10)
        .into()
    }
}
