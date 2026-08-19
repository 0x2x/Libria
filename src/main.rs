pub mod services;
pub mod models;
use std::env;

use std::ffi::CString;
unsafe extern "C" {
    fn audio_init();
    fn audio_load(path: *const std::ffi::c_char);
    fn audio_play();
    fn audio_pause();
    fn audio_stop();
    fn audio_seek(seconds: f64);

    fn information(msg: *const std::ffi::c_char);
    fn error(msg: *const std::ffi::c_char);
}


fn test() {

}
fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = &args[1];
    match cmd.as_str() {
        "playlist" | "playlists" =>{
            println!("Ran Playlist Command")
        },
        "list" | "view" => println!("g"), // List Current tracks
        "index" => println!("Index"),
        "set" =>{
            let arg_one = &args[2];
            match arg_one.as_str() {
                "default_folder" => {
                    let folder_path = &args[3];
                    println!("That may be a foder?")
                },
                _ => println!("That is not an argument")
            };
        },
        "play" => {
            let arg_one = &args[2];
            if(arg_one == "sade") {
                unsafe {
                    audio_init();
                    let path = CString::new(r"rpath").unwrap();
                    audio_load(path.as_ptr());
                    audio_play();
                    std::thread::sleep(std::time::Duration::from_secs(30));
                    audio_pause();
                    audio_seek(30.0);
                    audio_play();
                    audio_stop();
                }
            }
        }
        "np" => {
            unsafe {
                audio_init();
                let path = CString::new(r"path").unwrap();
                audio_load(path.as_ptr());
                audio_play();
                std::thread::sleep(std::time::Duration::from_secs(30));
                audio_pause();
                audio_seek(30.0);
                audio_play();
                audio_stop();
            }
        },
        _ => println!("Unknown Command")
    };
}

