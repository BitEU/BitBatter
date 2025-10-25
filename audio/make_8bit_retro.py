#!/usr/bin/env python3
"""
8-Bit Retro Audio Converter
Converts audio files to sound like 90's PC games by reducing sample rate and bit depth.
Based on techniques from: https://www.reddit.com/r/AudioPost/comments/1abgpsp/
"""

import os
import subprocess
from pathlib import Path

# Path to ffmpeg binary
FFMPEG_PATH = r"C:\ffmpeg\bin\ffmpeg.exe"

# Audio quality presets for different retro eras
PRESETS = {
    "8bit_low": {
        "sample_rate": 11025,
        "bit_depth": 8,
        "description": "Very crunchy 8-bit (early 90s PC games)"
    },
    "8bit_medium": {
        "sample_rate": 22050,
        "bit_depth": 8,
        "description": "8-bit with better clarity (mid 90s)"
    },
    "16bit_retro": {
        "sample_rate": 22050,
        "bit_depth": 16,
        "description": "16-bit retro (late 90s, more clarity)"
    },
    "custom": {
        "sample_rate": None,  # Will be set by user
        "bit_depth": None,
        "description": "Custom settings"
    }
}

def convert_to_8bit(input_file, output_file, sample_rate=11025, bit_depth=8):
    """
    Convert an audio file to retro 8-bit/16-bit style using ffmpeg.
    
    Args:
        input_file: Path to the input audio file
        output_file: Path to the output file (will be .wav for compatibility)
        sample_rate: Target sample rate (11025, 22050, or 44100 Hz)
        bit_depth: Target bit depth (8 or 16)
    """
    try:
        # Build ffmpeg command for WAV output (best compatibility with Windows)
        # -i: input file
        # -ar: audio sample rate
        # -acodec pcm_u8: PCM 8-bit unsigned codec
        # -acodec pcm_s16le: PCM 16-bit signed little-endian codec
        # -ac 1: convert to mono (authentic retro sound)
        # -y: overwrite output file if it exists
        
        # Determine codec based on bit depth
        if bit_depth == 8:
            codec = "pcm_u8"  # 8-bit unsigned PCM
        else:
            codec = "pcm_s16le"  # 16-bit signed PCM (little-endian)
        
        command = [
            FFMPEG_PATH,
            "-i", str(input_file),
            "-ar", str(sample_rate),
            "-acodec", codec,
            "-ac", "1",  # Mono for authentic retro sound
            "-y",
            str(output_file)
        ]
        
        print(f"Converting: {input_file.name}")
        print(f"  Settings: {sample_rate}Hz, {bit_depth}-bit, Mono, WAV format")
        
        # Run the command and capture output
        result = subprocess.run(
            command,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        
        if result.returncode == 0:
            print(f"  ✓ Successfully converted to {output_file.name}")
            return True
        else:
            print(f"  ✗ Error converting {input_file.name}")
            print(f"    {result.stderr}")
            return False
            
    except Exception as e:
        print(f"  ✗ Exception converting {input_file.name}: {e}")
        return False

def main():
    """
    Main function to convert audio files to retro 8-bit style.
    """
    # Get the directory where this script is located
    script_dir = Path(__file__).parent
    
    print("=" * 60)
    print("8-BIT RETRO AUDIO CONVERTER")
    print("=" * 60)
    print(f"Directory: {script_dir}")
    print(f"FFmpeg: {FFMPEG_PATH}")
    print("=" * 60)
    print()
    
    # Check if ffmpeg exists
    if not os.path.exists(FFMPEG_PATH):
        print(f"ERROR: ffmpeg not found at {FFMPEG_PATH}")
        print("Please ensure ffmpeg is installed and the path is correct.")
        return
    
    # Display preset options
    print("Available Presets:")
    print("-" * 60)
    preset_keys = list(PRESETS.keys())
    for i, (key, preset) in enumerate(PRESETS.items(), 1):
        if key != "custom":
            print(f"{i}. {key.upper():20} - {preset['description']}")
            print(f"   ({preset['sample_rate']} Hz, {preset['bit_depth']}-bit)")
    print(f"{len(preset_keys)}. CUSTOM - Enter your own settings")
    print("-" * 60)
    print()
    
    # Get user choice
    choice = input("Select a preset (1-4) or press Enter for default [1]: ").strip()
    if not choice:
        choice = "1"
    
    try:
        choice_idx = int(choice) - 1
        selected_preset = preset_keys[choice_idx]
        
        if selected_preset == "custom":
            print("\nCustom Settings:")
            sample_rate = int(input("Enter sample rate (e.g., 11025, 22050, 44100): "))
            bit_depth = int(input("Enter bit depth (8 or 16): "))
        else:
            sample_rate = PRESETS[selected_preset]["sample_rate"]
            bit_depth = PRESETS[selected_preset]["bit_depth"]
            
    except (ValueError, IndexError):
        print("Invalid choice. Using default preset (8bit_low).")
        sample_rate = PRESETS["8bit_low"]["sample_rate"]
        bit_depth = PRESETS["8bit_low"]["bit_depth"]
    
    print()
    print(f"Using settings: {sample_rate} Hz, {bit_depth}-bit, Mono")
    print()
    
    # Find all audio files (MP3, WAV, OGG)
    audio_files = []
    for ext in ["*.mp3", "*.wav", "*.ogg", "*.MP3", "*.WAV", "*.OGG"]:
        audio_files.extend(script_dir.glob(ext))
    
    if not audio_files:
        print("No audio files found in the directory.")
        return
    
    print(f"Found {len(audio_files)} audio file(s) to convert:")
    for file in audio_files:
        print(f"  - {file.name}")
    print()
    
    # Ask if user wants to proceed
    proceed = input("Proceed with conversion? (y/n) [y]: ").strip().lower()
    if proceed and proceed != 'y':
        print("Conversion cancelled.")
        return
    
    print()
    print("=" * 60)
    print("CONVERTING...")
    print("=" * 60)
    print()
    
    # Create output directory for 8-bit versions
    output_dir = script_dir / "retro_8bit"
    output_dir.mkdir(exist_ok=True)
    
    # Convert each file
    successful = 0
    failed = 0
    
    for audio_file in audio_files:
        # Skip files that are already in the output directory
        if "retro_8bit" in str(audio_file):
            continue
            
        # Create output filename - always use .wav for best compatibility
        output_file = output_dir / f"{audio_file.stem}.wav"
        
        # Convert the file
        if convert_to_8bit(audio_file, output_file, sample_rate, bit_depth):
            successful += 1
        else:
            failed += 1
        print()
    
    # Print summary
    print("=" * 60)
    print("CONVERSION COMPLETE!")
    print("=" * 60)
    print(f"Output directory: {output_dir}")
    print(f"Output format: WAV (for Windows compatibility)")
    print(f"Successful: {successful}")
    print(f"Failed: {failed}")
    print("=" * 60)
    print()
    print("TIP: For even more retro sound, try the 8bit_low preset (11025 Hz)")
    print("     All files are saved as WAV for best Windows Media Player compatibility!")

if __name__ == "__main__":
    main()
