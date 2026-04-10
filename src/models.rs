#[derive(Clone, Copy, Default, PartialEq)]
pub enum Page {
    #[default]
    Overview,
    Memory,
    Cpu,
    Storage,
    Battery,
}
