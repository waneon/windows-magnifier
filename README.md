# Magni
Magni is a simple, fast and smooth magnifier for Windows 10/11.

## Use
* `Alt+1`: toggle magnifier
* `Alt+2`: exit the program

## Install

### via Release package
#### 1. [Download](https://github.com/waneon/windows-magnifier/releases) Magni.exe and Magni_cert.cer
#### 2. Install Magni_cert.cer "Trusted Root Certification Authorities" of the local machine.
#### 3. Place Magni.exe to the secure locations
* \Program Files\ subdirectoreis
* \Program Files (x86)\ subdirecotries
* \Windows\system32\ subdirectories

### Build
#### 1. Build the package
#### 2. Create a certification using `makecert`
#### 3. Sign the executable using `SignTool`
#### 4. Goto `via Release package #2`