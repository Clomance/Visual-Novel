#[cfg(target_os="windows")]
mod windows;
#[cfg(target_os="windows")]
pub use windows::Music;

#[cfg(target_os="linux")]
mod linux;
#[cfg(target_os="linux")]
pub use linux::Music;

#[cfg(target_os="macos")]
mod linux;
#[cfg(target_os="macos")]
pub use linux::Music;