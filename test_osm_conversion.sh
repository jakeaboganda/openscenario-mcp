#!/bin/bash
# Test OSM to OpenDRIVE conversion
cd ~/.openclaw/workspace/osc-mcp
python3 tools/osm/osm_to_opendrive.py "$@"
