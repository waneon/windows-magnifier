cmake_minimum_required(VERSION 3.14)

# set project name and version
project(windows-magnifier VERSION 0.1.1 LANGUAGES CXX)

# set c++
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# make executable
add_executable("${CMAKE_PROJECT_NAME}" WIN32
	src/main.cpp
	src/maginfo.h
	src/maginfo.cpp
	res/app.rc
)

# link magnification windows api
target_link_libraries("${CMAKE_PROJECT_NAME}" Magnification)

# add manifest
add_custom_command(
	TARGET "${CMAKE_PROJECT_NAME}"
	POST_BUILD
	COMMAND mt.exe -manifest "${CMAKE_SOURCE_DIR}/manifest/magnifier.manifest" -outputresource:"${CMAKE_BINARY_DIR}/Release/${CMAKE_PROJECT_NAME}.exe"\;\#1
)