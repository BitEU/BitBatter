#!/usr/bin/env python3
"""Analyze baseball game log to find balance issues"""

import re
from collections import Counter

def analyze_log(filename):
    with open(filename, 'r') as f:
        content = f.read()
    
    # Extract all pitches
    pitches = content.split('PITCH #')[1:]
    
    print("=" * 80)
    print("BASEBALL GAME LOG ANALYSIS")
    print("=" * 80)
    print(f"\nTotal Pitches: {len(pitches)}\n")
    
    # Analyze results
    initial_results = []
    fielding_results = []
    contact_qualities = []
    success_chances = []
    timing_diffs = []
    
    for pitch in pitches:
        # Get contact quality
        cq_match = re.search(r'CONTACT QUALITY: (\d+)/100', pitch)
        if cq_match:
            contact_qualities.append(int(cq_match.group(1)))
        
        # Get initial result
        result_match = re.search(r'RESULT: (.+)', pitch)
        if result_match:
            initial_results.append(result_match.group(1))
        
        # Get fielding success chance
        success_match = re.search(r'Success Chance: ([\d.]+)%', pitch)
        if success_match:
            success_chances.append(float(success_match.group(1)))
        
        # Get timing diff
        timing_match = re.search(r'Timing Diff: (\d+) frames', pitch)
        if timing_match:
            timing_diffs.append(int(timing_match.group(1)))
        
        # Get fielding result
        fielding_match = re.search(r'FIELDING RESULT: (.+)', pitch)
        if fielding_match:
            fielding_results.append(fielding_match.group(1))
    
    # Print statistics
    print("INITIAL RESULTS (Before Fielding):")
    for result, count in Counter(initial_results).most_common():
        print(f"  {result}: {count} ({count/len(initial_results)*100:.1f}%)")
    
    print("\nFIELDING RESULTS (Final Outcome):")
    for result, count in Counter(fielding_results).most_common():
        print(f"  {result}: {count} ({count/len(fielding_results)*100:.1f}%)")
    
    print(f"\nCONTACT QUALITY:")
    print(f"  Average: {sum(contact_qualities)/len(contact_qualities):.1f}")
    print(f"  Min: {min(contact_qualities)}")
    print(f"  Max: {max(contact_qualities)}")
    print(f"  90+: {sum(1 for cq in contact_qualities if cq >= 90)} ({sum(1 for cq in contact_qualities if cq >= 90)/len(contact_qualities)*100:.1f}%)")
    print(f"  75-89: {sum(1 for cq in contact_qualities if 75 <= cq < 90)} ({sum(1 for cq in contact_qualities if 75 <= cq < 90)/len(contact_qualities)*100:.1f}%)")
    print(f"  55-74: {sum(1 for cq in contact_qualities if 55 <= cq < 75)} ({sum(1 for cq in contact_qualities if 55 <= cq < 75)/len(contact_qualities)*100:.1f}%)")
    print(f"  35-54: {sum(1 for cq in contact_qualities if 35 <= cq < 55)} ({sum(1 for cq in contact_qualities if 35 <= cq < 55)/len(contact_qualities)*100:.1f}%)")
    print(f"  <35: {sum(1 for cq in contact_qualities if cq < 35)} ({sum(1 for cq in contact_qualities if cq < 35)/len(contact_qualities)*100:.1f}%)")
    
    print(f"\nFIELDING SUCCESS CHANCE:")
    print(f"  Average: {sum(success_chances)/len(success_chances):.1f}%")
    print(f"  80% (good timing): {sum(1 for sc in success_chances if sc == 80.0)} ({sum(1 for sc in success_chances if sc == 80.0)/len(success_chances)*100:.1f}%)")
    print(f"  5% (bad timing): {sum(1 for sc in success_chances if sc == 5.0)} ({sum(1 for sc in success_chances if sc == 5.0)/len(success_chances)*100:.1f}%)")
    print(f"  Other: {sum(1 for sc in success_chances if sc != 5.0 and sc != 80.0)} ({sum(1 for sc in success_chances if sc != 5.0 and sc != 80.0)/len(success_chances)*100:.1f}%)")
    
    print(f"\nTIMING DIFF:")
    print(f"  Average: {sum(timing_diffs)/len(timing_diffs):.1f} frames")
    print(f"  Perfect (0-2 frames): {sum(1 for td in timing_diffs if td <= 2)} ({sum(1 for td in timing_diffs if td <= 2)/len(timing_diffs)*100:.1f}%)")
    print(f"  Good (3-5 frames): {sum(1 for td in timing_diffs if 3 <= td <= 5)} ({sum(1 for td in timing_diffs if 3 <= td <= 5)/len(timing_diffs)*100:.1f}%)")
    print(f"  Bad (6+ frames): {sum(1 for td in timing_diffs if td >= 6)} ({sum(1 for td in timing_diffs if td >= 6)/len(timing_diffs)*100:.1f}%)")
    
    # Count outcome changes
    outcome_changes = 0
    outs_to_hits = 0
    hits_to_outs = 0
    
    for pitch in pitches:
        initial = re.search(r'RESULT: (.+)', pitch)
        fielding = re.search(r'FIELDING RESULT: (.+)', pitch)
        if initial and fielding:
            init_str = initial.group(1)
            field_str = fielding.group(1)
            if init_str != field_str:
                outcome_changes += 1
                if 'OUT' in init_str and 'HIT' in field_str:
                    outs_to_hits += 1
                elif 'HIT' in init_str and 'OUT' in field_str:
                    hits_to_outs += 1
    
    print(f"\nOUTCOME CHANGES:")
    print(f"  Total changes: {outcome_changes} ({outcome_changes/len([p for p in pitches if 'FIELDING RESULT' in p])*100:.1f}%)")
    print(f"  Outs changed to Hits: {outs_to_hits} ⚠️")
    print(f"  Hits changed to Outs: {hits_to_outs}")
    
    print("\n" + "=" * 80)
    print("CONCLUSION:")
    print("=" * 80)
    print("1. Fielding success rate is WAY too low (average {:.1f}%)".format(sum(success_chances)/len(success_chances)))
    print(f"2. {outs_to_hits} OUTS were incorrectly changed to HITS by fielding")
    print("3. Contact quality distribution seems reasonable")
    print("4. The problem is FIELDING, not batting!")

if __name__ == '__main__':
    import sys
    filename = sys.argv[1] if len(sys.argv) > 1 else 'game_log_20251026_022021.txt'
    analyze_log(filename)
