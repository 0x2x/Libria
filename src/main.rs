pub mod services;
pub mod models;

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

fn main() {
    unsafe {
        audio_init();
        let path = CString::new(r"TEST").unwrap();
        audio_load(path.as_ptr());
        audio_play();
        std::thread::sleep(std::time::Duration::from_secs(30));
        audio_pause();
        audio_seek(30.0);
        audio_play();
        audio_stop();
    }
}

