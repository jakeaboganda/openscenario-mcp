#!/bin/bash
# Check if OpenSCENARIO XSD schema files are present

SCHEMA_DIR="$(dirname "$0")/schemas"
MISSING=()

for version in v1.0 v1.1 v1.2; do
    xsd_file="$SCHEMA_DIR/$version/OpenSCENARIO.xsd"
    if [ ! -f "$xsd_file" ]; then
        MISSING+=("$version")
    else
        echo "✅ Found: $xsd_file"
    fi
done

if [ ${#MISSING[@]} -gt 0 ]; then
    echo ""
    echo "⚠️  Missing XSD files for versions: ${MISSING[*]}"
    echo ""
    echo "To obtain XSD files:"
    echo "1. Visit: https://www.asam.net/standards/detail/openscenario/"
    echo "2. Download OpenSCENARIO releases for each version"
    echo "3. Extract OpenSCENARIO.xsd to schemas/<version>/"
    echo ""
    echo "Or if you have esmini installed:"
    echo "  cp /path/to/esmini/resources/xsd/OpenSCENARIO_v1.0.xsd schemas/v1.0/OpenSCENARIO.xsd"
    echo "  cp /path/to/esmini/resources/xsd/OpenSCENARIO_v1.1.xsd schemas/v1.1/OpenSCENARIO.xsd"
    echo "  cp /path/to/esmini/resources/xsd/OpenSCENARIO_v1.2.xsd schemas/v1.2/OpenSCENARIO.xsd"
    exit 1
else
    echo ""
    echo "✅ All XSD schema files present!"
    exit 0
fi
