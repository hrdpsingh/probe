use iced::{
    Application, Command, Element, Font, Settings, Theme, executor,
    font::Weight,
    widget::{column, row, text},
};
use nix::sys::utsname::uname;
use std::{env, fs, path::Path};
use sysinfo::{Disks, System};

fn main() -> iced::Result {
    ByteScope::run(Settings::default())
}

struct ByteScope {
    hostname: String,
    kernel: String,
    os: String,
    display_server: String,
    ram: u64,
    storage: u64,
    cpu: String,
    product_name: String,
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
            Ok(val) => val.to_string_lossy().to_string(),
            Err(_) => "Unavailable".to_string(),
        };

        let kernel = match uname() {
            Ok(val) => val.release().to_string_lossy().to_string(),
            Err(_) => "Unavailable".to_string(),
        };

        let os = os_info::get().to_string();

        let display_server = match env::var("XDG_SESSION_TYPE") {
            Ok(val) => val,
            Err(_) => "Unavailable".to_string(),
        };

        let mut sys = System::new();
        sys.refresh_memory();
        let ram = sys.total_memory();

        let storage = Disks::new_with_refreshed_list()
            .list()
            .iter()
            .find(|disk| disk.mount_point() == Path::new("/"))
            .map(|disk| disk.total_space())
            .expect("Unavailable");

        let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap();
        let mut cpu = "Unavailable".to_string();
        for line in cpuinfo.lines() {
            if line.starts_with("model name")
                && let Some((_, name)) = line.split_once(':')
            {
                cpu = name.trim().to_string();
            }
        }

        let product_name = fs::read_to_string("/sys/devices/virtual/dmi/id/product_name")
            .unwrap_or_else(|_| "Unknown".into());

        (
            Self {
                hostname,
                kernel,
                os,
                display_server,
                ram,
                storage,
                cpu,
                product_name,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("ByteScope")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let software = column![
            text("Software").font(Font {
                weight: Weight::Bold,
                ..Font::default()
            }),
            text(format!("Hostname: {}", self.hostname)),
            text(format!("Kernel: {}", self.kernel)),
            text(format!("OS: {}", self.os)),
            text(format!("Display Server: {}", self.display_server)),
        ]
        .padding(20)
        .spacing(10);

        let hardware = column![
            text("Hardware").font(Font {
                weight: Weight::Bold,
                ..Font::default()
            }),
            text(format!(
                "Memory: {:.2} GB",
                self.ram as f64 / (1024.0 * 1024.0 * 1024.0)
            )),
            text(format!(
                "Storage: {:.2} GB",
                self.storage as f64 / (1024.0 * 1024.0 * 1024.0)
            )),
            text(format!("CPU: {}", self.cpu)),
            text(format!("Product Name: {}", self.product_name)),
        ]
        .padding(20)
        .spacing(10);

        row![software, hardware,].spacing(20).padding(20).into()
    }
}
