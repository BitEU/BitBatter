@echo off
echo Building Baseball TUI...
echo.

cargo build --release

if %errorlevel% neq 0 (
    echo.
    echo Build failed! Make sure Rust is installed:
    echo https://rustup.rs/
    pause
    exit /b %errorlevel%
)

echo.
echo Build successful!
echo.
echo Executable location:
echo target\release\terminalbball.exe
echo.
echo To run the game:
echo   cargo run --release
echo or
echo   .\target\release\terminalbball.exe
echo.
pause
