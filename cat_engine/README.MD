# ![logo](https://github.com/Clomance/Visual-Novel/raw/master/cat_engine/logo_400x400.png)

# Cat Engine

A 2D graphics engine with audio support.

Uses OpenGL 2.0 and above.

#### Dependencies
 - Graphics - [glium](https://github.com/glium/glium)
 - Images - [image](https://github.com/image-rs/image)
 - Fonts - [rusttype](https://gitlab.redox-os.org/redox-os/rusttype)
 - Music - [cpal](https://github.com/RustAudio/cpal) and [minimp3](https://github.com/germangb/minimp3-rs)

If you use audio feature, alsa delevopment libraries are required for Linux: `libasound2-dev` for Debian and Ubuntu, `alsa-lib-devel` for Fedora (see [cpal](https://github.com/RustAudio/cpal)).

## Features

Default features = ["texture_graphics", "simple_graphics", "text_graphics"]

Other features = ["audio", "mouse_cursor_icon", "auto_hide"]

### audio

Enables to use audio output.

### mouse_cursor_icon

Replaces the default mouse cursor with user's image.

The cursor points to the center of the image.

The default path to the image is `./mouse_cursor_icon.png`.

The ImageBase for the image binds to the texture vertex buffer.
The default range is 4..8.

### auto_hide

The window gets minimized when loses focus and
it stops sending outer events, except gained focus event.
The window gets back when it gains focus.

### fps_counter

A simple fps counter. The value updates every second.