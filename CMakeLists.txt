cmake_minimum_required(VERSION 3.10)

# set project name and version
set(VERSION 0.1.0)
project(Magni VERSION "${VERSION}")

# make executable
set(EXE_NAME Magni)
add_executable("${EXE_NAME}"
	"${CMAKE_SOURCE_DIR}/src/main.cpp"
	"${CMAKE_SOURCE_DIR}/src/maginfo.h"
	"${CMAKE_SOURCE_DIR}/src/maginfo.cpp"
)

# link magnification library
target_link_libraries(Magni Magnification)

# make version information header file
configure_file("${CMAKE_SOURCE_DIR}/config/version.h.in" version.h)
target_include_directories(Magni PUBLIC "${CMAKE_BINARY_DIR}")

# add manifest
add_custom_command(
	TARGET Magni
	POST_BUILD
	COMMAND "mt.exe" -manifest "${CMAKE_SOURCE_DIR}/src/magni.exe.manifest" -outputresource:"${CMAKE_BINARY_DIR}/Release/${EXE_NAME}.exe"\;\#1
)