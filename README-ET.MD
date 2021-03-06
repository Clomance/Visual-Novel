# Rustil kirjutatud visuaalne novell.

### Versioon 2.0.0

Viimane stabiilne Rust, minimaalne OpenGL-i versioon - `2.0`.

#### Sõltuvused
 - Graafika - [glium](https://github.com/glium/glium)
 - Pildid - [image](https://github.com/image-rs/image)
 - Fondid - [rusttype](https://gitlab.redox-os.org/redox-os/rusttype)
 - Hääl - [cpal](https://github.com/RustAudio/cpal), [minimp3](https://github.com/germangb/minimp3-rs)

Töötab stabiilselt 64-bitisel Windows 10 versioonil. Pole veel testinud teistel süsteemidel.

Alsa development on vajalik Linuxil töötamiseks:  `libasound2-dev` Debian ja Ubuntu puhul, `alsa-lib-devel` Fedora puhul (vaata [cpal](https://github.com/RustAudio/cpal)).

Mängi kompileerimiseks ja käivitamiseks kasuta käsku `cargo run --release`.

### Kuvatõmmised
![screenshot1](./screenshots/screenshot0.png)

### Muu

Juhtnupud:
 - F5 - kuvatõmmis
 - Space - järgmine dialoog

### Probleemid
 - Mängu aken jääb mittefokuseerituks peale `win + tab`.
 - Menüünupud liiguvad äkitselt kui kursorit liigutada aeglaselt.
