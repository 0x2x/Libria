fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/cpp/audio_engine.cpp")
        .file("src/cpp/debug.cpp") // <-- Add your debug implementation here!
        .include("src/cpp")
        .compile("audio_engine");

    println!("cargo:rerun-if-changed=src/cpp/audio_engine.cpp");
    println!("cargo:rerun-if-changed=src/cpp/debug.cpp");
    println!("cargo:rerun-if-changed=src/cpp/include/audio.h");
    println!("cargo:rerun-if-changed=src/cpp/include/debug.h");
}