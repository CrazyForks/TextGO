#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
pub use macos::{get_selection, get_selection_location, is_cursor_editable, select_backward_chars};
#[cfg(target_os = "windows")]
pub use windows::{
    get_selection, get_selection_location, is_cursor_editable, select_backward_chars,
};
