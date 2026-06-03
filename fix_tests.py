#!/usr/bin/env python3
import re
import sys

def fix_test_file(filename):
    with open(filename, 'r') as f:
        content = f.read()
    
    # Pattern to find handle_add_speed_action and handle_add_lane_change_action calls
    # These functions now take an additional Option<f64> parameter at the end
    
    # For handle_add_speed_action: takes 7 params (was 6)
    # Last required param is duration (f64)
    pattern_speed = r'(handle_add_speed_action\([^)]+?,\s*\n\s*)(\d+\.?\d*),(\s*\n\s*\))'
    replacement_speed = r'\1\2,\n        None,\3'
    
    # For handle_add_lane_change_action: takes 7 params (was 6)  
    # Last required param is duration (f64)
    pattern_lane = r'(handle_add_lane_change_action\([^)]+?,\s*\n\s*)(\d+\.?\d*),(\s*\n\s*\))'
    replacement_lane = r'\1\2,\n        None,\3'
    
    # Apply replacements
    content = re.sub(pattern_speed, replacement_speed, content)
    content = re.sub(pattern_lane, replacement_lane, content)
    
    with open(filename, 'w') as f:
        f.write(content)
    print(f"Fixed {filename}")

if __name__ == '__main__':
    files = [
        'error_edge_case_tests.rs',
        'complex_scenario_tests.rs', 
        'mcp_action_tools_tests.rs',
        'mcp_integration_test.rs'
    ]
    
    for fname in files:
        try:
            fix_test_file(fname)
        except Exception as e:
            print(f"Error processing {fname}: {e}", file=sys.stderr)
