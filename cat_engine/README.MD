![logo](./logo_400x400.png)
# Cat Engine

A 2D graphics engine with audio support.

Uses OpenGL 2.0 and above.

#### Dependencies
 - Graphics - [glium](https://github.com/glium/glium)
 - Images - [image](https://github.com/image-rs/image)
 - Fonts - [rusttype](https://gitlab.redox-os.org/redox-os/rusttype)
 - Music - [cpal](https://github.com/RustAudio/cpal) and [minimp3](https://github.com/germangb/minimp3-rs)

If you use audio feature alsa delevopment libraries are required for Linux: `libasound2-dev` for Debian and Ubuntu, `alsa-lib-devel` for Fedora (see [cpal](https://github.com/RustAudio/cpal)).

## Features

Default features = ["texture_graphics", "simple_graphics", "text_graphics"]

Other features = ["audio", "mouse_cursor_icon", "auto_hide"]

### audio

Enables to use audio output.

### mouse_cursor_icon

Replaces default mouse cursor with users one.

It uses image to draw the cursor.

The cursor points to the center of cursor image.

The default path to the image is `./mouse_cursor_icon.png`.

### auto_hide

The window gets minimized when loses focus.