#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub use macos::{get_selection, is_cursor_editable, select_backward_chars};
#[cfg(target_os = "windows")]
pub use windows::{get_selection, is_cursor_editable, select_backward_chars};
