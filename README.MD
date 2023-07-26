# A visual novel written in Rust.

 - [Russian version of README](https://github.com/Clomance/Visual-Novel/blob/master/README-RUS.MD)
 - [Estonian version of README](https://github.com/Clomance/Visual-Novel/blob/master/README-ET.MD)

### Version 2.0.0

Latest stable Rust, minimal version of OpenGL - `2.0`.

#### Dependencies
 - Graphics - [glium](https://github.com/glium/glium)
 - Images - [image](https://github.com/image-rs/image)
 - Fonts - [rusttype](https://gitlab.redox-os.org/redox-os/rusttype)
 - Sound - [cpal](https://github.com/RustAudio/cpal) and [minimp3](https://github.com/germangb/minimp3-rs)

Runs stable on x64 versions of Windows 10. Haven't tested other systems yet.

Alsa delevopment libraries are required for Linux: `libasound2-dev` for Debian and Ubuntu, `alsa-lib-devel` for Fedora (see [cpal](https://github.com/RustAudio/cpal)).

For building and starting the game run `cargo run --release`.

### Screenshots
![screenshot1](./screenshots/screenshot0.png)

### Other

Controls:
 - F5 - screenshot
 - Space - "next" for the dialogues

### Current problems
 - Game window stays unfocused after switching to it using `win + tab`.
 - Buttons move abruptly if cursor is moved slowly in the main menu. 

### Cross-compile errors
 - 32 bit: error on X86_32
 - IOS: cannot find value `kAudioDevicePropertyBufferFrameSize` in crate `sys`
 - Linux arch, aarch64: Unable to locate package alsa-lib-devel.aarch64
 - Windows aarch64: minimp3/minimp3.h(187): fatal error C1189: #error:  MINIMP3_ONLY_SIMD used, but SSE/NEON not enabled
