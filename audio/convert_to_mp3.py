#!/usr/bin/env python3
"""
Audio Converter Script
Converts all WAV and OGG files in the current directory to MP3 format using ffmpeg.
"""

import os
import subprocess
from pathlib import Path

# Path to ffmpeg binary
FFMPEG_PATH = r"C:\ffmpeg\bin\ffmpeg.exe"

def convert_to_mp3(input_file, output_file):
    """
    Convert an audio file to MP3 format using ffmpeg.
    
    Args:
        input_file: Path to the input audio file
        output_file: Path to the output MP3 file
    """
    try:
        # Run ffmpeg command
        # -i: input file
        # -codec:a libmp3lame: use MP3 encoder
        # -qscale:a 2: quality setting (0-9, lower is better, 2 is high quality)
        # -y: overwrite output file if it exists
        command = [
            FFMPEG_PATH,
            "-i", str(input_file),
            "-codec:a", "libmp3lame",
            "-qscale:a", "2",
            "-y",
            str(output_file)
        ]
        
        print(f"Converting: {input_file.name} -> {output_file.name}")
        
        # Run the command and capture output
        result = subprocess.run(
            command,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True
        )
        
        if result.returncode == 0:
            print(f"  ✓ Successfully converted {input_file.name}")
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
    Main function to find and convert all WAV and OGG files to MP3.
    """
    # Get the directory where this script is located
    script_dir = Path(__file__).parent
    
    print(f"Audio Converter")
    print(f"=" * 50)
    print(f"Directory: {script_dir}")
    print(f"FFmpeg: {FFMPEG_PATH}")
    print(f"=" * 50)
    print()
    
    # Check if ffmpeg exists
    if not os.path.exists(FFMPEG_PATH):
        print(f"ERROR: ffmpeg not found at {FFMPEG_PATH}")
        print("Please ensure ffmpeg is installed and the path is correct.")
        return
    
    # Find all WAV and OGG files
    audio_files = []
    audio_files.extend(script_dir.glob("*.wav"))
    audio_files.extend(script_dir.glob("*.ogg"))
    audio_files.extend(script_dir.glob("*.WAV"))
    audio_files.extend(script_dir.glob("*.OGG"))
    
    if not audio_files:
        print("No WAV or OGG files found in the directory.")
        return
    
    print(f"Found {len(audio_files)} audio file(s) to convert:")
    for file in audio_files:
        print(f"  - {file.name}")
    print()
    
    # Convert each file
    successful = 0
    failed = 0
    
    for audio_file in audio_files:
        # Create output filename (same name but with .mp3 extension)
        output_file = audio_file.with_suffix(".mp3")
        
        # Skip if the file is already MP3
        if audio_file.suffix.lower() == ".mp3":
            print(f"Skipping {audio_file.name} (already MP3)")
            continue
        
        # Convert the file
        if convert_to_mp3(audio_file, output_file):
            successful += 1
        else:
            failed += 1
    
    # Print summary
    print()
    print(f"=" * 50)
    print(f"Conversion complete!")
    print(f"  Successful: {successful}")
    print(f"  Failed: {failed}")
    print(f"=" * 50)

if __name__ == "__main__":
    main()
