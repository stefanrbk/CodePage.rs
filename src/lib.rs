mod code_page;
pub mod dos;
pub mod windows;

pub use code_page::CodePage;

pub type ANSI = windows::Windows1252;
pub type DOS = dos::Dos437;

pub use dos::{CP437Graphical, CP437};
pub use windows::CP1252;
