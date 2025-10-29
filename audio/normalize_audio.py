#!/usr/bin/env python3
"""
Audio LUFS Analyzer and Normalizer

This script analyzes the LUFS (Loudness Units Full Scale) of audio files,
uses bat.wav as the reference standard, and normalizes other audio files
to match within a reasonable tolerance range.

Requirements:
- ffmpeg-python
- numpy
- scipy (optional, for additional audio processing)

Install with: pip install ffmpeg-python numpy scipy
"""

import os
import sys
import glob
import subprocess
import json
import argparse
from pathlib import Path
from typing import Dict, List, Tuple, Optional

try:
    import ffmpeg
    import numpy as np
except ImportError as e:
    print(f"Missing required package: {e}")
    print("Please install with: pip install ffmpeg-python numpy scipy")
    sys.exit(1)


class AudioLUFSAnalyzer:
    def __init__(self, reference_file: str = "bat.wav", tolerance: float = 1.0):
        """
        Initialize the LUFS analyzer.
        
        Args:
            reference_file: The reference audio file (default: bat.wav)
            tolerance: Acceptable LUFS difference range in dB (default: ±1.0 dB)
        """
        self.reference_file = reference_file
        self.tolerance = tolerance
        self.reference_lufs = None
        self.audio_dir = Path(__file__).parent
        
    def get_lufs(self, audio_file: str) -> Optional[float]:
        """
        Get LUFS measurement for an audio file using ffmpeg.
        
        Args:
            audio_file: Path to the audio file
            
        Returns:
            LUFS value as float, or None if measurement fails
        """
        try:
            # Use ffmpeg to measure LUFS
            cmd = [
                'ffmpeg',
                '-i', audio_file,
                '-af', 'loudnorm=print_format=json',
                '-f', 'null',
                '-'
            ]
            
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True
            )
            
            # Parse the JSON output from stderr (ffmpeg outputs to stderr)
            output = result.stderr
            
            # Find the JSON block in the output
            json_start = output.rfind('{')
            json_end = output.rfind('}') + 1
            
            if json_start != -1 and json_end != -1:
                json_str = output[json_start:json_end]
                data = json.loads(json_str)
                return float(data.get('input_i', None))
            
            return None
            
        except (subprocess.SubprocessError, json.JSONDecodeError, ValueError) as e:
            print(f"Error measuring LUFS for {audio_file}: {e}")
            return None
    
    def analyze_reference(self) -> bool:
        """
        Analyze the reference file to get the target LUFS.
        
        Returns:
            True if successful, False otherwise
        """
        ref_path = self.audio_dir / self.reference_file
        
        if not ref_path.exists():
            print(f"Error: Reference file '{self.reference_file}' not found!")
            return False
        
        print(f"Analyzing reference file: {self.reference_file}")
        self.reference_lufs = self.get_lufs(str(ref_path))
        
        if self.reference_lufs is None:
            print(f"Error: Could not measure LUFS for reference file!")
            return False
        
        print(f"Reference LUFS: {self.reference_lufs:.2f} dB")
        print(f"Target range: {self.reference_lufs - self.tolerance:.2f} to {self.reference_lufs + self.tolerance:.2f} dB")
        
        return True
    
    def analyze_all_files(self) -> Dict[str, float]:
        """
        Analyze LUFS for all audio files in the directory.
        
        Returns:
            Dictionary mapping filenames to LUFS values
        """
        audio_extensions = ['*.wav', '*.mp3', '*.flac', '*.ogg', '*.m4a']
        audio_files = []
        
        for ext in audio_extensions:
            audio_files.extend(glob.glob(str(self.audio_dir / ext)))
        
        results = {}
        
        print(f"\nAnalyzing {len(audio_files)} audio files...")
        
        for file_path in audio_files:
            filename = Path(file_path).name
            
            # Skip the reference file
            if filename == self.reference_file:
                results[filename] = self.reference_lufs
                continue
                
            lufs = self.get_lufs(file_path)
            if lufs is not None:
                results[filename] = lufs
                difference = lufs - self.reference_lufs
                status = "✓" if abs(difference) <= self.tolerance else "⚠"
                print(f"{status} {filename:25} {lufs:6.2f} dB (diff: {difference:+6.2f} dB)")
            else:
                print(f"✗ {filename:25} Failed to analyze")
        
        return results
    
    def normalize_file(self, input_file: str, target_lufs: float, dry_run: bool = False) -> bool:
        """
        Normalize an audio file to the target LUFS.
        
        Args:
            input_file: Path to input audio file
            target_lufs: Target LUFS value
            dry_run: If True, don't actually process files
            
        Returns:
            True if successful, False otherwise
        """
        if dry_run:
            print(f"[DRY RUN] Would normalize {Path(input_file).name} to {target_lufs:.2f} LUFS")
            return True
        
        try:
            # Create backup
            backup_file = input_file.replace('.wav', '_backup.wav')
            subprocess.run(['copy', input_file, backup_file], shell=True, check=True)
            
            # Normalize using ffmpeg loudnorm filter
            temp_file = input_file.replace('.wav', '_temp.wav')
            
            (
                ffmpeg
                .input(input_file)
                .audio
                .filter('loudnorm', I=target_lufs, TP=-1.0, LRA=7.0)
                .output(temp_file)
                .overwrite_output()
                .run(quiet=True)
            )
            
            # Replace original with normalized version
            subprocess.run(['move', temp_file, input_file], shell=True, check=True)
            
            return True
            
        except (subprocess.SubprocessError, ffmpeg.Error) as e:
            print(f"Error normalizing {input_file}: {e}")
            return False
    
    def normalize_all_files(self, dry_run: bool = False) -> None:
        """
        Normalize all audio files that are outside the tolerance range.
        
        Args:
            dry_run: If True, show what would be done without actually doing it
        """
        if self.reference_lufs is None:
            print("Error: Reference LUFS not set. Run analyze_reference() first.")
            return
        
        results = self.analyze_all_files()
        files_to_normalize = []
        
        print(f"\n{'='*60}")
        print("NORMALIZATION PLAN")
        print(f"{'='*60}")
        
        for filename, lufs in results.items():
            if filename == self.reference_file:
                continue
                
            difference = lufs - self.reference_lufs
            
            if abs(difference) > self.tolerance:
                files_to_normalize.append((filename, lufs, difference))
                status = "WILL NORMALIZE" if not dry_run else "WOULD NORMALIZE"
                print(f"{status:15} {filename:25} {lufs:6.2f} → {self.reference_lufs:6.2f} dB")
            else:
                print(f"{'OK':15} {filename:25} {lufs:6.2f} dB (within tolerance)")
        
        if not files_to_normalize:
            print("\n✓ All files are already within tolerance range!")
            return
        
        if dry_run:
            print(f"\n[DRY RUN] Would normalize {len(files_to_normalize)} files")
            return
        
        print(f"\nNormalizing {len(files_to_normalize)} files...")
        
        success_count = 0
        for filename, current_lufs, difference in files_to_normalize:
            file_path = str(self.audio_dir / filename)
            
            print(f"Processing {filename}...")
            if self.normalize_file(file_path, self.reference_lufs):
                success_count += 1
                print(f"✓ {filename} normalized successfully")
            else:
                print(f"✗ {filename} normalization failed")
        
        print(f"\nCompleted: {success_count}/{len(files_to_normalize)} files normalized successfully")
    
    def generate_report(self) -> None:
        """Generate a detailed report of all audio file LUFS levels."""
        if self.reference_lufs is None:
            if not self.analyze_reference():
                return
        
        results = self.analyze_all_files()
        
        print(f"\n{'='*80}")
        print("DETAILED LUFS ANALYSIS REPORT")
        print(f"{'='*80}")
        print(f"Reference file: {self.reference_file} ({self.reference_lufs:.2f} LUFS)")
        print(f"Tolerance range: ±{self.tolerance:.1f} dB")
        print(f"Target range: {self.reference_lufs - self.tolerance:.2f} to {self.reference_lufs + self.tolerance:.2f} LUFS")
        print("-" * 80)
        print(f"{'Filename':<25} {'LUFS':<10} {'Difference':<12} {'Status'}")
        print("-" * 80)
        
        within_tolerance = 0
        total_files = len(results)
        
        for filename, lufs in sorted(results.items()):
            if filename == self.reference_file:
                difference = 0.0
                status = "REFERENCE"
            else:
                difference = lufs - self.reference_lufs
                if abs(difference) <= self.tolerance:
                    status = "OK"
                    within_tolerance += 1
                else:
                    status = "NEEDS NORM"
            
            print(f"{filename:<25} {lufs:6.2f} dB  {difference:+6.2f} dB    {status}")
        
        print("-" * 80)
        print(f"Summary: {within_tolerance}/{total_files - 1} files within tolerance")
        if within_tolerance < total_files - 1:
            print(f"Recommendation: Run normalization to fix {total_files - 1 - within_tolerance} files")


def main():
    parser = argparse.ArgumentParser(
        description="Analyze and normalize audio file LUFS levels",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python normalize_audio.py --analyze                    # Just analyze files
  python normalize_audio.py --normalize --dry-run       # Show what would be normalized
  python normalize_audio.py --normalize                 # Actually normalize files
  python normalize_audio.py --reference hit.wav         # Use different reference file
  python normalize_audio.py --tolerance 2.0             # Use 2dB tolerance instead of 1dB
        """
    )
    
    parser.add_argument(
        '--reference', 
        default='bat.wav',
        help='Reference audio file (default: bat.wav)'
    )
    
    parser.add_argument(
        '--tolerance', 
        type=float, 
        default=1.0,
        help='LUFS tolerance in dB (default: 1.0)'
    )
    
    parser.add_argument(
        '--analyze', 
        action='store_true',
        help='Only analyze files, don\'t normalize'
    )
    
    parser.add_argument(
        '--normalize', 
        action='store_true',
        help='Normalize files outside tolerance range'
    )
    
    parser.add_argument(
        '--dry-run', 
        action='store_true',
        help='Show what would be done without actually doing it'
    )
    
    args = parser.parse_args()
    
    # If no action specified, default to analyze
    if not args.analyze and not args.normalize:
        args.analyze = True
    
    analyzer = AudioLUFSAnalyzer(
        reference_file=args.reference,
        tolerance=args.tolerance
    )
    
    # Always analyze reference first
    if not analyzer.analyze_reference():
        sys.exit(1)
    
    if args.analyze:
        analyzer.generate_report()
    
    if args.normalize:
        analyzer.normalize_all_files(dry_run=args.dry_run)


if __name__ == "__main__":
    main()
