@echo off
echo Building Baseball TUI...
echo.

echo Running unit tests...
echo.
cargo test

if %errorlevel% neq 0 (
    echo.
    echo Tests failed! Please fix the failing tests before building.
    pause
    exit /b %errorlevel%
)

echo.
echo All tests passed!
echo.
echo Building release version...
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
echo target\release\BitBatter.exe
echo.
echo To run the game:
echo   cargo run --release
echo or
echo   .\target\release\BitBatter.exe
echo.
pause
