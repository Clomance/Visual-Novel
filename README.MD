# Visual novel written in Rust.

[Russian version of README](https://github.com/Clomance/Visual-Novel/blob/master/README-RUS.MD)
[Estonian version of README](https://github.com/Clomance/Visual-Novel/blob/master/README-ET.MD)

### Version 1.5.0

Latest stable Rust, minimal version of OpenGL - `2.0`.

#### Dependencies
 - Graphics - [glium](https://github.com/glium/glium)
 - Images - [image](https://github.com/image-rs/image)
 - Fonts - [rusttype](https://gitlab.redox-os.org/redox-os/rusttype)
 - Music - [rodio](https://github.com/RustAudio/rodio)

Runs stable on x64 versions of Windows 10. Haven't tested other systems yet.

Alsa delevopment libraries are required for Linux: `libasound2-dev` for Debian and Ubuntu, `alsa-lib-devel` for Fedora (check [cpal](https://github.com/RustAudio/cpal)).

For building and starting the game run `cargo run --release`.

### Screenshots
![screenshot1](https://raw.githubusercontent.com/Clomance/Visual-Novel/master/screenshots/screenshot0.png)

### Other

Controls:
 - F5 - screenshot
 - Space - "next" for the dialogues

#### Debugging

Run `cargo run` to build and run the game in debug mode.

New debug features will be added soon.

`F10` key toggles the ability to minimize the game. Enabled by default.

### Current problems
 - Game window stays unfocused after switching to it using `win + tab`.
 - Buttons move abruptly if cursor is moved slowly in the main menu. 