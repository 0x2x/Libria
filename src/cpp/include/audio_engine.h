#pragma once

extern "C" {
    void audio_init();
    void audio_load(const char* path);
    void audio_play();
    void audio_pause();
    void audio_stop();
    void audio_seek(double seconds);
    void audio_metadata(const char* path);
}