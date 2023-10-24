mod code_page;
pub mod windows;

pub use code_page::CodePage;

pub type ANSI = windows::Windows1252;
