# OpenSCENARIO XSD Schemas

## ⚠️ IMPORTANT: Test Schema vs Production Schema

### Current Status

This directory contains **MINIMAL TEST SCHEMAS** for basic XSD validation during development.

**These are NOT production-ready schemas!**

---

## What's Included

**v1.2/OpenSCENARIO_MINIMAL_TEST.xsd**
- Simplified test schema with ~10% coverage
- Basic element structures
- Many complex types stubbed with `<xs:anyType/>`
- **Use**: Development, basic validation, unit tests
- **DO NOT USE**: Production validation, compliance checking

---

## Why is the schema incomplete?

The bundled schema was created for testing the XSD validation infrastructure:
- Verifies validator loads schemas correctly
- Tests basic element/attribute validation
- Provides fast feedback during development

**Missing** from test schema:
- Full action type definitions (Speed, Lateral, Routing, etc.)
- Complete trigger/condition structures
- Detailed dynamics models
- Controller specifications
- Advanced trajectory definitions
- ~90% of the full specification

---

## How to Get Official XSD Schemas

### Option A: Download from ASAM (Recommended)

1. Visit: https://www.asam.net/standards/detail/openscenario/
2. Download OpenSCENARIO releases:
   - OpenSCENARIO 1.0
   - OpenSCENARIO 1.1
   - OpenSCENARIO 1.2
3. Extract `OpenSCENARIO.xsd` from each archive
4. Place in respective version directories:
   ```
   schemas/v1.0/OpenSCENARIO.xsd
   schemas/v1.1/OpenSCENARIO.xsd
   schemas/v1.2/OpenSCENARIO.xsd
   ```

### Option B: Use esmini Schemas

If you have esmini installed:
```bash
# Find esmini's XSD files
find /path/to/esmini -name "*.xsd"

# Copy to schema directories
cp /path/to/esmini/resources/xsd/OpenSCENARIO_v1.0.xsd schemas/v1.0/OpenSCENARIO.xsd
cp /path/to/esmini/resources/xsd/OpenSCENARIO_v1.1.xsd schemas/v1.1/OpenSCENARIO.xsd
cp /path/to/esmini/resources/xsd/OpenSCENARIO_v1.2.xsd schemas/v1.2/OpenSCENARIO.xsd
```

### Option C: Use OpenSCENARIO GitHub

```bash
cd schemas/v1.2
curl -O https://raw.githubusercontent.com/OpenSCENARIO/OpenSCENARIO/master/schema/OpenSCENARIO.xsd
```

---

## Verification

After placing official schemas:

```bash
cd ../..  # Back to openscenario/ directory
./check-schemas.sh
```

Expected output:
```
✅ Found: schemas/v1.0/OpenSCENARIO.xsd
✅ Found: schemas/v1.1/OpenSCENARIO.xsd  
✅ Found: schemas/v1.2/OpenSCENARIO.xsd

✅ All XSD schema files present!
```

---

## Schema File Naming

**For Uppsala validator to recognize schemas:**
- File MUST be named `OpenSCENARIO.xsd`
- Test schema uses different name: `OpenSCENARIO_MINIMAL_TEST.xsd`
- This prevents accidental use of test schema for production validation

---

## License & Copyright

OpenSCENARIO XSD schemas are copyrighted by ASAM e.V.

- **Standard**: https://www.asam.net/standards/detail/openscenario/
- **License**: Check ASAM website for current terms
- **Usage**: Typically permitted for implementation purposes

**Bundled test schema** is a derivative work created for testing only.

---

## What Happens Without Official Schemas?

The validator falls back to basic validation:
- XML well-formedness check
- FileHeader version verification
- Root element structure

**Limitations**:
- No detailed element validation
- No attribute constraint checking
- No cardinality enforcement
- Invalid documents may pass

**Warning in validation report**:
```
ValidationReport {
    valid: true,
    warnings: vec!["XSD schema not available for OpenSCENARIO v1.2. Performing basic validation only."]
}
```

---

## Summary

| Schema Type | Location | Use Case | Coverage |
|-------------|----------|----------|----------|
| **Test** | v1.2/OpenSCENARIO_MINIMAL_TEST.xsd | Development, unit tests | ~10% |
| **Production** | v1.2/OpenSCENARIO.xsd (not included) | Validation, compliance | 100% |

**Action Required**: Obtain official ASAM schemas for production use.

Run `./check-schemas.sh` to verify setup.
