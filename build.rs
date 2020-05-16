use std::env;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;

fn main(){
    let target=env::var("TARGET").unwrap();

    if target.contains("pc-windows"){
        let manifest_dir=PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

        let mut lib_dir=manifest_dir.clone();
        let mut dll_dir=manifest_dir.clone();

        lib_dir.push("sdl2/msvc/lib");
        dll_dir.push("sdl2/msvc/dll");

        println!("cargo:rustc-link-search=all={}",lib_dir.display());
        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir"){
            let entry_path=entry.expect("Invalid fs entry").path();
            let file_name_result=entry_path.file_name();

            let mut new_file_path=manifest_dir.clone();

            if let Some(file_name)=file_name_result{
                let file_name=file_name.to_str().unwrap();
                if file_name.ends_with(".dll"){
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path,new_file_path.as_path()).expect("Can't copy from DLL dir");
                }
            }
        }
    }

    let mut game_settings=OpenOptions::new().truncate(true).write(true).open("settings/game_settings").unwrap();

    game_settings.write_all(&[0]).unwrap(); // Новая игра

    let buffer=[0u8;8];
    // Текущая страница игры
    game_settings.write_all(&buffer).unwrap();
    // Текущее положение в диалоге на странице
    game_settings.write_all(&buffer).unwrap();
    // Количество символов в секунду
    let buffer=0.25f32.to_be_bytes();
    game_settings.write_all(&buffer).unwrap();
    // Значение громкости
    game_settings.write_all(&[64u8]).unwrap();
    // Количество сделанных скриншотов (номер следующего)
    game_settings.write_all(&[0u8;4]).unwrap();
}