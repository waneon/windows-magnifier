cmake_minimum_required(VERSION 3.18)

# set build type
set(CMAKE_BUILD_TYPE Release)

# set project name and version
project(Magni VERSION 0.1.0)

# make executable
set(EXE_NAME Magni)
add_executable("${EXE_NAME}" WIN32
	"${CMAKE_SOURCE_DIR}/src/main.cpp"
	"${CMAKE_SOURCE_DIR}/src/maginfo.h"
	"${CMAKE_SOURCE_DIR}/src/maginfo.cpp"
)

# link magnification windows api
target_link_libraries(Magni Magnification)

# add manifest
if($<STREQUAL:"${CMAKE_BUILD_TYPE}","Release">)
	add_custom_command(
		TARGET Magni
		POST_BUILD
		COMMAND "mt.exe" -manifest "${CMAKE_SOURCE_DIR}/src/magni.exe.manifest" -outputresource:"${CMAKE_BINARY_DIR}/Release/${EXE_NAME}.exe"\;\#1
	)
endif()