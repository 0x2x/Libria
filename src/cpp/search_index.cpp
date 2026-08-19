//
// Created by Quinn on 8/18/2026.
//

#include "include/search_index.h"
#include <iostream>
#include <string>
#include <vector>
#include <filesystem>

std::string toLowerCase(std::string str) {
    std::transform(str.begin(), str.end(), str.begin(), ::toLowerCase);
    return str;
}

std::vector<std::string> searchMusicIndex(const std::string& rootFolder, const std::string& query) {
    std::vector<std::string> matches;
    std::string lowerQuery = toLowerCase(query);

    std::vector<std::string> extensions = {".mp3", ".flac", ".wav", ".aac", ".ogg"};
    try {
        for (const auto& entry : std::filesystem::recursive_directory_iterator(rootFolder)) {
            if (entry.is_regular_file()) {
                std::string ext = entry.path().extension().string();

                auto it = std::find(extensions.begin(), extensions.end(), toLowerCase(ext));
                if (it != extensions.end()) {
                    std::string filename = entry.path().filename().string();
                    if (toLowerCase(filename).find(toLowerCase(lowerQuery)) != std::string::npos) {
                        matches.push_back(entry.path().string());
                    }
                }
            }
        }

    }catch (const fs::filesystem_error& e) {
        std::cerr << "Filesystem Error: " << e.what() << std::endl;
    }
}
void search_algorithm() {
}