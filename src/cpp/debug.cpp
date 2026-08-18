#include "include/debug.h"
#include "include/colors.h"
#include <iostream>

extern "C" void information(const char* msg) {
    std::cout << Color::BLUE << "[Information]" << Color::RESET << " " << msg << std::endl;
}

extern "C" void warning(const char* msg) {
    std::cout << Color::RED << "[Error]" << Color::RESET << " " << msg << std::endl;
}

