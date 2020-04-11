#![allow(non_snake_case,non_upper_case_globals,non_camel_case_types,unused_must_use,dead_code)]
mod sync_raw_ptr;
pub use sync_raw_ptr::SyncRawPtr;

mod drawable;
pub use drawable::Drawable;

mod colors;
pub use colors::*;

pub fn get_monitor_size()->[f64;2]{
    let size=glutin::event_loop::EventLoop::new().primary_monitor().size();
    [size.width as f64,size.height as f64]
}