#include "include/audio_engine.h"
#include "include/debug.h"
#define MINIAUDIO_IMPLEMENTATION
#include "include/miniaudio.h"

static ma_engine g_engine;
static ma_sound g_sound;
static bool g_engine_initialized = false;
static bool g_sound_loaded = false;
static float g_sound_volume;
// Current Track Data
static float current_count;
extern "C" {
    void audio_init() {
        if (g_sound_loaded) return;

        if (ma_engine_init(NULL, &g_engine) == MA_SUCCESS) {
            g_engine_initialized = true;
            information("Audio engine started successfully");
        }
        else {
            warning("Failed to initalize audio engine");
        }
    }

    void audio_load(const char* path) {
        if (!g_engine_initialized) return;

        if (g_sound_loaded) {
            ma_sound_uninit(&g_sound);
            g_sound_loaded = false;
        }

        // Load Files (WAV, MP3, FLAC)
        if (ma_sound_init_from_file(&g_engine, path, 0, NULL, NULL, &g_sound) == MA_SUCCESS) {
            g_sound_loaded = true;
            information("Audio Loaded Successfully");
        }
        else {
            warning("Failed to load audio file");
        }
    }

    void audio_play() {
        information("Audio Playing");
        if (g_sound_loaded) {
            ma_result result = ma_sound_start(&g_sound);
            if (result != MA_SUCCESS) {
                warning("Failed to start audio");
            }
        }
    }

    void audio_pause() {
        information("Audio Pause");
        if (!g_engine_initialized) return;
        if (g_sound_loaded) {
            ma_sound_uninit(&g_sound);
            g_sound_loaded = false;
        }
    }

    void audio_stop() {
        information("Audio Stpo");
    }

    void audio_seek(double seconds) {
        information("Seeking");
        if (!g_engine_initialized) return;
        if (g_sound_loaded) {
            ma_sound_uninit(&g_sound);
            g_sound_loaded = false;

        }
    }

    void audio_metadata(const char* path) {
        if (!g_engine_initialized) return;
        if (g_sound_loaded) {
            ma_sound_uninit(&g_sound);
            g_sound_loaded = false;

        }
    }
}
