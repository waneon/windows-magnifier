cmake_minimum_required(VERSION 3.10)

# set project name and version
set(VERSION 0.1.0)
project(Magni VERSION "${VERSION}")

# make executable
add_executable(Magni src/main.cpp src/maginfo.h src/maginfo.cpp)

# link magnification library
target_link_libraries(Magni Magnification)

# make version information header file
configure_file("${CMAKE_SOURCE_DIR}/config/version.h.in" version.h)
target_include_directories(Magni PUBLIC "${CMAKE_BINARY_DIR}")