#!/usr/bin/env python3
"""
OpenDRIVE Scenario Matcher

Analyzes .xodr files and recommends suitable scenarios based on road geometry.
Helps choose the right road network for each test scenario.
"""

import xml.etree.ElementTree as ET
import sys
from typing import List, Dict, Tuple

def analyze_road_for_scenarios(road_info: Dict) -> Dict[str, bool]:
    """Determine which scenarios are suitable for this road."""
    
    capabilities = {
        'highway_merge': False,
        'lane_change_overtaking': False,
        'emergency_braking': False,
        'platooning': False,
        'traffic_light': False,
        'pedestrian_crossing': False,
        'parking': False,
        'multi_vehicle': False,
        'high_speed': False,
        'curved_road': False,
        'junction': False,
    }
    
    # Count lanes
    num_driving_lanes_left = sum(1 for l in road_info['lanes']['left'] if l['type'] == 'driving')
    num_driving_lanes_right = sum(1 for l in road_info['lanes']['right'] if l['type'] == 'driving')
    total_driving_lanes = num_driving_lanes_left + num_driving_lanes_right
    
    # Check geometry
    is_straight = all(g['type'] == 'straight' for g in road_info['geometries'])
    is_curved = any(g['type'] in ['curved', 'spiral'] for g in road_info['geometries'])
    
    road_length = road_info['length']
    
    # Highway scenarios need multiple lanes
    if total_driving_lanes >= 2 and road_length >= 200:
        capabilities['lane_change_overtaking'] = True
        capabilities['emergency_braking'] = True
        capabilities['platooning'] = True
        capabilities['multi_vehicle'] = True
    
    # Highway merge needs 2+ right lanes (one can simulate on-ramp)
    if num_driving_lanes_right >= 2 and road_length >= 200:
        capabilities['highway_merge'] = True
    
    # High-speed scenarios need long straight roads
    if is_straight and road_length >= 300:
        capabilities['high_speed'] = True
    
    # Curved road scenarios
    if is_curved:
        capabilities['curved_road'] = True
    
    return capabilities

def get_scenario_requirements() -> Dict[str, Dict]:
    """Define requirements for each scenario type."""
    return {
        'highway_merge': {
            'name': 'Highway Merge',
            'min_lanes': 2,
            'min_length': 200,
            'prefer_right_lanes': True,
            'straight': True,
            'description': 'Vehicle merges from on-ramp (far-right lane) into normal traffic',
        },
        'lane_change_overtaking': {
            'name': 'Lane Change / Overtaking',
            'min_lanes': 2,
            'min_length': 200,
            'prefer_left_lanes': True,
            'straight': True,
            'description': 'Fast vehicle overtakes slower vehicle using left lane',
        },
        'emergency_braking': {
            'name': 'Emergency Braking',
            'min_lanes': 1,
            'min_length': 100,
            'straight': True,
            'description': 'Following vehicle performs emergency stop',
        },
        'platooning': {
            'name': 'Platooning',
            'min_lanes': 1,
            'min_length': 200,
            'straight': True,
            'description': 'Multiple vehicles maintain formation with time headway',
        },
        'curved_road': {
            'name': 'Curved Road Handling',
            'min_lanes': 1,
            'min_length': 100,
            'curved': True,
            'description': 'Vehicle navigates curved road sections',
        },
    }

def recommend_scenarios(road_info: Dict) -> List[Tuple[str, Dict, str]]:
    """Recommend scenarios suitable for this road with rationale."""
    
    capabilities = analyze_road_for_scenarios(road_info)
    requirements = get_scenario_requirements()
    
    recommendations = []
    
    for scenario_key, scenario_def in requirements.items():
        if capabilities.get(scenario_key, False):
            # Generate rationale
            rationale = []
            
            num_driving_lanes_left = sum(1 for l in road_info['lanes']['left'] if l['type'] == 'driving')
            num_driving_lanes_right = sum(1 for l in road_info['lanes']['right'] if l['type'] == 'driving')
            
            if scenario_def.get('prefer_right_lanes') and num_driving_lanes_right >= 2:
                rationale.append(f"{num_driving_lanes_right} right lanes (can use lane -{num_driving_lanes_right} as on-ramp)")
            
            if scenario_def.get('prefer_left_lanes') and num_driving_lanes_left >= 1:
                rationale.append(f"{num_driving_lanes_left} left lane(s) for overtaking")
            
            if scenario_def.get('straight'):
                is_straight = all(g['type'] == 'straight' for g in road_info['geometries'])
                if is_straight:
                    rationale.append("straight geometry (predictable)")
            
            if scenario_def.get('curved'):
                is_curved = any(g['type'] in ['curved', 'spiral'] for g in road_info['geometries'])
                if is_curved:
                    rationale.append("curved sections present")
            
            if road_info['length'] >= scenario_def.get('min_length', 0):
                rationale.append(f"{road_info['length']:.0f}m length (sufficient space)")
            
            recommendations.append((
                scenario_key,
                scenario_def,
                '; '.join(rationale)
            ))
    
    return recommendations

def suggest_lane_positions(road_info: Dict, scenario_key: str) -> List[Dict]:
    """Suggest specific lane positions for a scenario."""
    
    suggestions = []
    
    left_lanes = [l for l in road_info['lanes']['left'] if l['type'] == 'driving']
    right_lanes = [l for l in road_info['lanes']['right'] if l['type'] == 'driving']
    
    if scenario_key == 'highway_merge':
        if len(right_lanes) >= 2:
            # Use far-right as "on-ramp", merge into next lane
            on_ramp_lane = right_lanes[-1]  # Furthest right
            target_lane = right_lanes[-2]   # One lane to the left
            
            suggestions.append({
                'role': 'merging_vehicle',
                'start_lane': on_ramp_lane['id'],
                'start_s': 10.0,
                'target_lane': target_lane['id'],
                'code': f'Position::lane("{road_info["id"]}", {on_ramp_lane["id"]}, 10.0, 0.0, None)',
                'description': f'Start in far-right lane {on_ramp_lane["id"]} (simulates on-ramp), merge into lane {target_lane["id"]}'
            })
    
    elif scenario_key == 'lane_change_overtaking':
        if len(right_lanes) >= 1 and len(left_lanes) >= 1:
            normal_lane = right_lanes[0]  # Rightmost lane
            overtake_lane = left_lanes[-1] if left_lanes else None  # Leftmost lane
            
            if overtake_lane:
                suggestions.extend([
                    {
                        'role': 'slow_vehicle',
                        'start_lane': normal_lane['id'],
                        'start_s': 100.0,
                        'code': f'Position::lane("{road_info["id"]}", {normal_lane["id"]}, 100.0, 0.0, None)',
                        'description': f'Slow vehicle in normal lane {normal_lane["id"]}'
                    },
                    {
                        'role': 'fast_vehicle',
                        'start_lane': normal_lane['id'],
                        'start_s': 0.0,
                        'overtake_lane': overtake_lane['id'],
                        'code': f'Position::lane("{road_info["id"]}", {normal_lane["id"]}, 0.0, 0.0, None)',
                        'description': f'Fast vehicle starts in lane {normal_lane["id"]}, overtakes via lane {overtake_lane["id"]}'
                    }
                ])
    
    elif scenario_key == 'emergency_braking':
        if right_lanes:
            lane = right_lanes[0]
            suggestions.extend([
                {
                    'role': 'lead_vehicle',
                    'start_lane': lane['id'],
                    'start_s': 100.0,
                    'code': f'Position::lane("{road_info["id"]}", {lane["id"]}, 100.0, 0.0, None)',
                    'description': f'Lead vehicle in lane {lane["id"]}'
                },
                {
                    'role': 'follower_vehicle',
                    'start_lane': lane['id'],
                    'start_s': 50.0,
                    'code': f'Position::lane("{road_info["id"]}", {lane["id"]}, 50.0, 0.0, None)',
                    'description': f'Follower 50m behind in same lane'
                }
            ])
    
    elif scenario_key == 'platooning':
        if right_lanes:
            lane = right_lanes[0]
            suggestions.extend([
                {
                    'role': 'leader',
                    'start_lane': lane['id'],
                    'start_s': 100.0,
                    'code': f'Position::lane("{road_info["id"]}", {lane["id"]}, 100.0, 0.0, None)',
                    'description': 'Platoon leader'
                },
                {
                    'role': 'follower1',
                    'start_lane': lane['id'],
                    'start_s': 60.0,
                    'code': f'Position::lane("{road_info["id"]}", {lane["id"]}, 60.0, 0.0, None)',
                    'description': 'First follower (40m behind leader)'
                },
                {
                    'role': 'follower2',
                    'start_lane': lane['id'],
                    'start_s': 20.0,
                    'code': f'Position::lane("{road_info["id"]}", {lane["id"]}, 20.0, 0.0, None)',
                    'description': 'Second follower (40m behind first)'
                }
            ])
    
    return suggestions

def print_scenario_recommendations(road_info: Dict):
    """Print scenario recommendations for the road."""
    
    print("\n" + "="*80)
    print("SCENARIO RECOMMENDATIONS")
    print("="*80 + "\n")
    
    recommendations = recommend_scenarios(road_info)
    
    if not recommendations:
        print("⚠️  This road is not suitable for common test scenarios.")
        print("   Consider roads with:")
        print("   - Multiple lanes (2+)")
        print("   - Sufficient length (200m+)")
        print("   - Straight or curved geometry")
        return
    
    print(f"✅ This road is suitable for {len(recommendations)} scenario type(s):\n")
    
    for i, (scenario_key, scenario_def, rationale) in enumerate(recommendations, 1):
        print(f"{i}. {scenario_def['name']}")
        print(f"   {scenario_def['description']}")
        print(f"   Why: {rationale}")
        print()
        
        # Show suggested positions
        suggestions = suggest_lane_positions(road_info, scenario_key)
        if suggestions:
            print("   Suggested vehicle positions:")
            for sugg in suggestions:
                print(f"   - {sugg['role']}: {sugg['description']}")
                print(f"     {sugg['code']}")
            print()

def print_scenario_code_template(road_info: Dict, scenario_key: str):
    """Generate code template for a specific scenario."""
    
    requirements = get_scenario_requirements()
    scenario_def = requirements.get(scenario_key)
    
    if not scenario_def:
        return
    
    print("\n" + "="*80)
    print(f"CODE TEMPLATE: {scenario_def['name']}")
    print("="*80 + "\n")
    
    suggestions = suggest_lane_positions(road_info, scenario_key)
    
    if not suggestions:
        print("No specific suggestions for this scenario on this road.")
        return
    
    print("```rust")
    print(f"// {scenario_def['description']}")
    print()
    
    for sugg in suggestions:
        vehicle_name = sugg['role']
        print(f"// {sugg['description']}")
        print(f'scenario.add_vehicle("{vehicle_name}", vehicle_params.clone())?;')
        print(f'scenario.set_initial_position("{vehicle_name}", {sugg["code"]})?;')
        print(f'scenario.set_initial_speed("{vehicle_name}", 25.0)?;  // TODO: Adjust speed')
        print()
    
    print("// TODO: Add actions and conditions")
    print("```")
    print()

def main():
    import argparse
    
    parser = argparse.ArgumentParser(description='Analyze OpenDRIVE for scenario suitability')
    parser.add_argument('xodr_file', help='Path to .xodr file')
    parser.add_argument('--template', choices=['highway_merge', 'lane_change_overtaking', 'emergency_braking', 'platooning'],
                       help='Generate code template for specific scenario')
    
    args = parser.parse_args()
    
    try:
        from analyze_opendrive import parse_xodr, describe_road
        
        root = parse_xodr(args.xodr_file)
        roads = root.findall('.//road')
        
        if not roads:
            print("No roads found in file", file=sys.stderr)
            sys.exit(1)
        
        # Analyze first road (or all if multiple)
        for road in roads:
            road_info = describe_road(road)
            print_scenario_recommendations(road_info)
            
            if args.template:
                print_scenario_code_template(road_info, args.template)
    
    except ImportError:
        print("Error: analyze_opendrive.py not found in the same directory", file=sys.stderr)
        print("Make sure both scripts are in the same location", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        import traceback
        traceback.print_exc()
        sys.exit(1)

if __name__ == '__main__':
    main()
