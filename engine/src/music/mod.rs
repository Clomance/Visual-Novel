mod windows;
#[cfg(target_os="windows")]
pub use windows::Music;

mod linux;
#[cfg(target_os="linux")]
pub use linux::Music;