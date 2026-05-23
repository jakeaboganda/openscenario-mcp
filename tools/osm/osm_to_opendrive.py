#!/usr/bin/env python3
"""
OSM to OpenDRIVE Converter

Downloads OpenStreetMap data for a location and converts it to OpenDRIVE format
using SUMO's netconvert tool.
"""

import argparse
import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Optional, Tuple
import urllib.request
import urllib.parse


class OSMConverter:
    """Convert OpenStreetMap data to OpenDRIVE format"""
    
    def __init__(self, cache_dir: str = "cache/osm"):
        self.cache_dir = Path(cache_dir)
        self.cache_dir.mkdir(parents=True, exist_ok=True)
    
    def download_osm_bbox(
        self, 
        bbox: Tuple[float, float, float, float],
        output_file: str
    ) -> bool:
        """
        Download OSM data for a bounding box using Overpass API.
        
        Args:
            bbox: (min_lon, min_lat, max_lon, max_lat)
            output_file: Output .osm file path
        
        Returns:
            True if successful
        """
        min_lon, min_lat, max_lon, max_lat = bbox
        
        # Overpass QL query for highways only
        query = f"""[out:xml][timeout:25];
(
  way["highway"]({min_lat},{min_lon},{max_lat},{max_lon});
);
(._;>;);
out meta;
"""
        
        overpass_url = "https://overpass-api.de/api/interpreter"
        
        print(f"📥 Downloading OSM data from bbox: {bbox}")
        print(f"   Query area: ~{abs(max_lon-min_lon)*111:.1f}km × {abs(max_lat-min_lat)*111:.1f}km")
        
        try:
            # Create request with proper headers
            req = urllib.request.Request(
                overpass_url,
                data=query.encode('utf-8'),
                headers={
                    'Content-Type': 'application/x-www-form-urlencoded',
                    'User-Agent': 'osc-mcp/1.0 (OpenSCENARIO generator)'
                },
                method='POST'
            )
            
            with urllib.request.urlopen(req, timeout=30) as response:
                data = response.read()
            
            # Write to file
            with open(output_file, 'wb') as f:
                f.write(data)
            
            file_size = os.path.getsize(output_file)
            
            # Basic validation
            if file_size < 100:
                print(f"❌ Downloaded file too small ({file_size} bytes) - probably empty or error")
                return False
            
            # Check if it's valid XML
            with open(output_file, 'r', encoding='utf-8') as f:
                first_line = f.readline()
                if not first_line.startswith('<?xml'):
                    print(f"❌ Downloaded file doesn't appear to be XML")
                    return False
            
            print(f"✅ Downloaded {file_size:,} bytes to {output_file}")
            return True
            
        except urllib.error.HTTPError as e:
            print(f"❌ HTTP Error {e.code}: {e.reason}")
            if e.code == 429:
                print("   Rate limited. Try again in a few minutes.")
            return False
        except Exception as e:
            print(f"❌ Download failed: {e}")
            return False
    
    def convert_osm_to_opendrive(
        self,
        osm_file: str,
        output_xodr: str,
        netconvert_path: str = "netconvert"
    ) -> bool:
        """
        Convert OSM file to OpenDRIVE using SUMO netconvert.
        
        Args:
            osm_file: Input .osm file
            output_xodr: Output .xodr file
            netconvert_path: Path to netconvert binary
        
        Returns:
            True if successful
        """
        print(f"🔄 Converting {osm_file} to OpenDRIVE...")
        
        cmd = [
            netconvert_path,
            "--osm-files", osm_file,
            "--opendrive-output", output_xodr,
            "--junctions.scurve-stretch", "1.0",
            "--geometry.remove", "true",
            "--verbose", "false"
        ]
        
        print(f"   Command: {' '.join(cmd)}")
        
        try:
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                check=True
            )
            
            if os.path.exists(output_xodr):
                file_size = os.path.getsize(output_xodr)
                print(f"✅ Converted successfully: {output_xodr} ({file_size:,} bytes)")
                return True
            else:
                print(f"❌ Output file not created: {output_xodr}")
                return False
                
        except subprocess.CalledProcessError as e:
            print(f"❌ netconvert failed: {e}")
            print(f"   stdout: {e.stdout}")
            print(f"   stderr: {e.stderr}")
            return False
        except FileNotFoundError:
            print(f"❌ netconvert not found at: {netconvert_path}")
            print("   Please install SUMO: https://sumo.dlr.de/docs/Installing/index.html")
            return False
    
    def get_tokyo_location(self, location_name: str) -> Optional[Tuple[float, float, float, float]]:
        """
        Get bounding box for known Tokyo locations.
        
        Returns:
            (min_lon, min_lat, max_lon, max_lat) or None
        """
        tokyo_locations = {
            "nihonbashi": (139.7700, 35.6815, 139.7750, 35.6845),  # ~500m square
            "route1_nihonbashi": (139.7680, 35.6800, 139.7780, 35.6860),  # Route 1 area
            "shuto_c1": (139.7500, 35.6700, 139.7700, 35.6900),  # Inner Circular
            "tokyo_station": (139.7650, 35.6790, 139.7750, 35.6850),
            "ginza": (139.7630, 35.6710, 139.7680, 35.6740),
        }
        
        return tokyo_locations.get(location_name.lower())
    
    def pipeline(
        self,
        location_or_bbox,
        output_name: str,
        force: bool = False
    ) -> Optional[str]:
        """
        Complete pipeline: download OSM → convert to OpenDRIVE.
        
        Args:
            location_or_bbox: Location name (str) or bbox tuple
            output_name: Base name for output files
            force: Force re-download/convert even if cached
        
        Returns:
            Path to .xodr file or None if failed
        """
        # Resolve location to bbox
        if isinstance(location_or_bbox, str):
            bbox = self.get_tokyo_location(location_or_bbox)
            if not bbox:
                print(f"❌ Unknown location: {location_or_bbox}")
                print(f"   Known locations: {', '.join(self.get_tokyo_location.__doc__)}")
                return None
        else:
            bbox = location_or_bbox
        
        # File paths
        osm_file = self.cache_dir / f"{output_name}.osm"
        xodr_file = self.cache_dir / f"{output_name}.xodr"
        
        # Check cache
        if xodr_file.exists() and not force:
            print(f"✅ Using cached file: {xodr_file}")
            return str(xodr_file)
        
        # Download OSM
        if not osm_file.exists() or force:
            if not self.download_osm_bbox(bbox, str(osm_file)):
                return None
        else:
            print(f"✅ Using cached OSM file: {osm_file}")
        
        # Convert to OpenDRIVE
        if self.convert_osm_to_opendrive(str(osm_file), str(xodr_file)):
            return str(xodr_file)
        else:
            return None


def main():
    parser = argparse.ArgumentParser(description="Convert OSM to OpenDRIVE")
    parser.add_argument(
        "location",
        help="Location name (e.g., 'nihonbashi') or bbox 'lon1,lat1,lon2,lat2'"
    )
    parser.add_argument(
        "-o", "--output",
        default="output",
        help="Output file base name (default: output)"
    )
    parser.add_argument(
        "-f", "--force",
        action="store_true",
        help="Force re-download/convert even if cached"
    )
    parser.add_argument(
        "--cache-dir",
        default="cache/osm",
        help="Cache directory for OSM files"
    )
    
    args = parser.parse_args()
    
    # Parse location
    if "," in args.location:
        try:
            bbox = tuple(map(float, args.location.split(",")))
            if len(bbox) != 4:
                raise ValueError("Bbox must have 4 values")
        except ValueError as e:
            print(f"❌ Invalid bbox format: {e}")
            print("   Expected: min_lon,min_lat,max_lon,max_lat")
            sys.exit(1)
    else:
        bbox = args.location
    
    # Run pipeline
    converter = OSMConverter(cache_dir=args.cache_dir)
    result = converter.pipeline(bbox, args.output, force=args.force)
    
    if result:
        print(f"\n🎉 Success! OpenDRIVE file: {result}")
        sys.exit(0)
    else:
        print(f"\n❌ Failed to convert {args.location}")
        sys.exit(1)


if __name__ == "__main__":
    main()
