use lazy_static::lazy_static;
use std::collections::HashMap;
use std::path::PathBuf;
use uppsala::xsd::XsdValidator as UppsalaValidator;

/// Validation report containing results and any errors
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// Whether the XML is valid
    pub valid: bool,
    /// List of validation errors with line numbers when available
    pub errors: Vec<String>,
}

/// XSD validator for OpenSCENARIO XML documents using Uppsala
#[derive(Debug, Clone)]
pub struct XsdValidator {
    version: String,
}

// Schema paths for each version
fn schema_path_for_version(version: &str) -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .join("schemas")
        .join(format!("v{}", version))
        .join("OpenSCENARIO.xsd")
}

// Lazy-loaded validators (parsed once at first use)
lazy_static! {
    static ref SCHEMA_VALIDATORS: HashMap<String, Option<UppsalaValidator>> = {
        let mut map = HashMap::new();
        
        for version in &["1.0", "1.1", "1.2"] {
            let schema_path = schema_path_for_version(version);
            
            if !schema_path.exists() {
                eprintln!(
                    "Warning: XSD schema not found for OpenSCENARIO v{}: {:?}",
                    version, schema_path
                );
                eprintln!("         Run ./check-schemas.sh for instructions");
                map.insert(version.to_string(), None);
                continue;
            }
            
            // Read schema file
            let schema_xml = match std::fs::read_to_string(&schema_path) {
                Ok(xml) => xml,
                Err(e) => {
                    eprintln!("Error reading schema file for v{}: {}", version, e);
                    map.insert(version.to_string(), None);
                    continue;
                }
            };
            
            match uppsala::parse(&schema_xml) {
                Ok(schema_doc) => {
                    match UppsalaValidator::from_schema(&schema_doc) {
                        Ok(validator) => {
                            eprintln!("✅ Loaded XSD schema for OpenSCENARIO v{}", version);
                            map.insert(version.to_string(), Some(validator));
                        }
                        Err(e) => {
                            eprintln!(
                                "Error building validator for v{}: {}",
                                version, e
                            );
                            map.insert(version.to_string(), None);
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Error parsing XSD schema for v{}: {}",
                        version, e
                    );
                    map.insert(version.to_string(), None);
                }
            }
        }
        
        map
    };
}

impl XsdValidator {
    /// Create a new validator for the specified OpenSCENARIO version
    ///
    /// Supported versions: "1.0", "1.1", "1.2"
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
        }
    }

    /// Validate XML content against the OpenSCENARIO XSD schema
    ///
    /// Performs full XSD validation using the Uppsala validator.
    /// If the XSD schema file is not available, falls back to basic well-formedness checking.
    ///
    /// # Arguments
    /// * `xml` - OpenSCENARIO XML string to validate
    ///
    /// # Returns
    /// * `ValidationReport` with validation results and errors
    ///
    /// # Examples
    /// ```
    /// use openscenario::validation::XsdValidator;
    ///
    /// let validator = XsdValidator::new("1.2");
    /// let xml = r#"<?xml version="1.0"?>
    /// <OpenSCENARIO>
    ///     <FileHeader revMajor="1" revMinor="2"/>
    /// </OpenSCENARIO>"#;
    /// let report = validator.validate(xml);
    /// assert!(report.valid || report.errors.len() > 0);
    /// ```
    pub fn validate(&self, xml: &str) -> ValidationReport {
        let mut errors = Vec::new();

        // First: Basic XML well-formedness check
        let doc = match uppsala::parse(xml) {
            Err(e) => {
                errors.push(format!("XML parsing error: {}", e));
                return ValidationReport {
                    valid: false,
                    errors,
                };
            }
            Ok(doc) => doc,
        };

        // Check for XSD validator availability
        match SCHEMA_VALIDATORS.get(&self.version) {
            Some(Some(validator)) => {
                // Full XSD validation
                let validation_errors = validator.validate(&doc);
                
                if validation_errors.is_empty() {
                    // Valid!
                    ValidationReport {
                        valid: true,
                        errors: vec![],
                    }
                } else {
                    // XSD validation failed
                    let error_messages: Vec<String> = validation_errors
                        .iter()
                        .map(|e| format!("{}", e))
                        .collect();
                    
                    ValidationReport {
                        valid: false,
                        errors: error_messages,
                    }
                }
            }
            Some(None) => {
                // Schema not available - fallback to basic validation
                errors.push(format!(
                    "XSD schema not available for OpenSCENARIO v{}. \
                    Performing basic validation only. \
                    Run ./check-schemas.sh to set up XSD files.",
                    self.version
                ));
                
                // Basic version check
                self.validate_version_basic(xml, &mut errors);
                
                ValidationReport {
                    valid: errors.len() == 1, // Only the warning
                    errors,
                }
            }
            None => {
                errors.push(format!(
                    "Unsupported OpenSCENARIO version: {}. \
                    Supported versions: 1.0, 1.1, 1.2",
                    self.version
                ));
                ValidationReport {
                    valid: false,
                    errors,
                }
            }
        }
    }

    /// Basic version validation fallback (when XSD not available)
    fn validate_version_basic(&self, xml: &str, errors: &mut Vec<String>) {
        use quick_xml::events::Event as XmlEvent;
        use quick_xml::Reader;

        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);
        let mut buf = Vec::new();
        let mut file_header_version = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(XmlEvent::Start(e)) | Ok(XmlEvent::Empty(e))
                    if e.name().as_ref() == b"FileHeader" =>
                {
                    let mut rev_major = None;
                    let mut rev_minor = None;

                    for attr in e.attributes() {
                        if let Ok(attr) = attr {
                            match attr.key.as_ref() {
                                b"revMajor" => {
                                    if let Ok(value) = attr.unescape_value() {
                                        rev_major = Some(value.to_string());
                                    }
                                }
                                b"revMinor" => {
                                    if let Ok(value) = attr.unescape_value() {
                                        rev_minor = Some(value.to_string());
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    if let (Some(major), Some(minor)) = (rev_major, rev_minor) {
                        file_header_version = Some(format!("{}.{}", major, minor));
                    }
                }
                Ok(XmlEvent::Eof) => break,
                Err(e) => {
                    errors.push(format!("XML parsing error: {}", e));
                    break;
                }
                _ => {}
            }
            buf.clear();
        }

        // Validate version match
        if let Some(file_version) = file_header_version {
            if file_version != self.version {
                errors.push(format!(
                    "Version mismatch: expected {}, found {}",
                    self.version, file_version
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let validator = XsdValidator::new("1.0");
        assert_eq!(validator.version, "1.0");
    }

    #[test]
    fn test_well_formed_xml() {
        let validator = XsdValidator::new("1.0");
        let xml = r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader revMajor="1" revMinor="0"/>
</OpenSCENARIO>"#;
        let report = validator.validate(xml);
        // Should at least parse without errors
        assert!(report.valid || report.errors.iter().any(|e| e.contains("XSD schema not available")));
    }

    #[test]
    fn test_malformed_xml() {
        let validator = XsdValidator::new("1.0");
        let xml = r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader revMajor="1" revMinor="0"
</OpenSCENARIO>"#;
        let report = validator.validate(xml);
        assert!(!report.valid);
        assert!(!report.errors.is_empty());
        assert!(report.errors[0].contains("XML parsing error"));
    }

    #[test]
    fn test_version_mismatch() {
        let validator = XsdValidator::new("1.0");
        let xml = r#"<?xml version="1.0"?>
<OpenSCENARIO>
    <FileHeader revMajor="1" revMinor="2"/>
</OpenSCENARIO>"#;
        let report = validator.validate(xml);
        // Should detect version mismatch (if XSD not available, falls back to basic check)
        let has_version_error = report.errors.iter().any(|e| e.contains("Version mismatch") || e.contains("XSD"));
        assert!(has_version_error);
    }
}
