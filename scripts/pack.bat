@echo off
setlocal EnableExtensions
cd /d "%~dp0.."

rustup target add wasm32-wasip2
if errorlevel 1 exit /b 1
cargo build --features guest --target wasm32-wasip2 --release
if errorlevel 1 exit /b 1

if not exist dist mkdir dist
copy /Y plugin.json dist\ >nul
copy /Y ui.json dist\ >nul
copy /Y USER.md dist\ >nul
if exist USER.zh.md copy /Y USER.zh.md dist\ >nul
if exist icon.svg copy /Y icon.svg dist\ >nul

set "WASM="
for %%F in (target\wasm32-wasip2\release\*.wasm) do set "WASM=%%F"
if not defined WASM (
  echo no wasm in target\wasm32-wasip2\release
  exit /b 1
)
copy /Y "%WASM%" dist\plugin.wasm >nul
echo packed dist\
