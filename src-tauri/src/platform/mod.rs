#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub use macos::get_selected_text;
#[cfg(target_os = "windows")]
pub use windows::get_selected_text;
