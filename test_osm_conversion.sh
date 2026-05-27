#!/bin/bash
# Test OSM to OpenDRIVE conversion
SCRIPT_PATH="$(realpath "$0")"
cd "$(dirname "$SCRIPT_PATH")/osc-mcp"
python3 tools/osm/osm_to_opendrive.py "$@"
