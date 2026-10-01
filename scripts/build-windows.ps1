$ErrorActionPreference = "Stop"

$Root = Split-Path -Parent $PSScriptRoot
$Engine = Join-Path $Root "engines\zsftp"
$Build = Join-Path $Engine "build"

cmake -S $Engine -B $Build -DRARFTP_BUILD_LIBRARY=ON -DRARFTP_BUILD_TESTS=OFF
cmake --build $Build --config Release
$env:ZSFTP_LIB_DIR = Join-Path $Build "Release"
if (-not (Test-Path (Join-Path $env:ZSFTP_LIB_DIR "zsftpcore.dll"))) { $env:ZSFTP_LIB_DIR = $Build }
Push-Location $Root
try { npm install; npm run tauri build } finally { Pop-Location }