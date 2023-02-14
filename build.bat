@echo off

set CERT_PATH="Cert:/CurrentUser/My"
set CERT_OUT_FILE="%~dp0/target/release/windows-magnifier.cer"
set CERTIFICATE="windows-magnifier"
set BINARY_FILE="%~dp0/target/release/magnifier.exe"

echo:
echo ===== building with release mode
cargo build --release

echo:
echo ===== creating certificate
powershell "if ( !(Get-ChildItem '%CERT_PATH%' | where { $_.subject -eq 'CN=%CERTIFICATE%' }) ) { New-SelfSignedCertificate -Type CodeSigningCert -Subject 'CN=%CERTIFICATE%' -CertStoreLocation '%CERT_PATH%' -NotAfter (Get-Date).AddYears(10) }"

echo:
echo ===== exporting certificate
powershell "Export-Certificate -Cert ( Get-ChildItem '%CERT_PATH%' | where { $_.subject -eq 'CN=%CERTIFICATE%' } ) -FilePath '%CERT_OUT_FILE%'"

echo:
echo ===== adding manifest
mt -manifest "%~dp0/manifest/magnifier.manifest" -outputresource:"%BINARY_FILE%";#1

echo:
echo ===== signing magnifier
SignTool sign /fd sha256 /v /n "%CERTIFICATE%" /t http://timestamp.digicert.com "%BINARY_FILE%"
