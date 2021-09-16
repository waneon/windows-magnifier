cmake_minimum_required(VERSION 3.14)

# set project name and version
project(windows-magnifier VERSION 0.1.1 LANGUAGES CXX)

# setup c++
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# make executable
add_executable("${CMAKE_PROJECT_NAME}" WIN32
	src/main.cpp
	src/Magnifier.h
	src/Magnifier.cpp
	res/app.rc
)

# link magnification windows api
target_link_libraries("${CMAKE_PROJECT_NAME}" Magnification)

if("${CMAKE_BUILD_TYPE}" STREQUAL "Release")
	set(TARGET_PATH "${CMAKE_BINARY_DIR}/Release/${CMAKE_PROJECT_NAME}.exe")
	set(CERT_PATH "Cert:\\CurrentUser\\My")

	# create certificate if not exists
	add_custom_command(
		TARGET "${CMAKE_PROJECT_NAME}"
		POST_BUILD
		COMMAND powershell "if ( !( Get-ChildItem '${CERT_PATH}' | where { $_.subject -eq 'CN=windows-magnifier' } ) ) { New-SelfSignedCertificate -Type CodeSigningCert -Subject 'CN=windows-magnifier' -CertStoreLocation '${CERT_PATH}' -NotAfter (Get-Date).AddYears(10) }"
	)

	# export certificate
	add_custom_command(
		TARGET "${CMAKE_PROJECT_NAME}"
		POST_BUILD
		COMMAND powershell "Export-Certificate -Cert ( Get-ChildItem '${CERT_PATH}' | where { $_.subject -eq 'CN=windows-magnifier' } ) -FilePath '${CMAKE_BINARY_DIR}/windows-magnifier.crt'"
	)

	# set manifest
	add_custom_command(
		TARGET "${CMAKE_PROJECT_NAME}"
		POST_BUILD
		COMMAND mt -manifest "${CMAKE_SOURCE_DIR}/manifest/magnifier.manifest" -outputresource:"${TARGET_PATH}"\;\#1
	)

	# sign
	add_custom_command(
		TARGET "${CMAKE_PROJECT_NAME}"
		POST_BUILD
		COMMAND SignTool sign /v /n "windows-magnifier" /t http://timestamp.digicert.com "${TARGET_PATH}"
	)
endif()