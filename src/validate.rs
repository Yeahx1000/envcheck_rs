use crate::parse::ParsedEnvFile;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct ValidationResult {
    pub missing: Vec<String>,
    pub empty: Vec<String>,
    pub duplicates: Vec<String>,
}

impl ValidationResult {
    pub fn has_errors(&self) -> bool {
        !self.missing.is_empty() || !self.empty.is_empty() || !self.duplicates.is_empty()
    }
}

pub fn validate_env_file(env: &ParsedEnvFile, required: &[String]) -> ValidationResult {
    let required_set: HashSet<&str> = required.iter().map(|s| s.as_str()).collect();
    let mut report = ValidationResult::default();

    for &key in &required_set {
        if !env.values.contains_key(key) {
            report.missing.push(key.to_string());
        }
    }

    for (key, value) in &env.values {
        if value.trim().is_empty() {
            report.empty.push(key.clone());
        }
    }

    report.duplicates = env.duplicates.clone();

    report.missing.sort();
    report.empty.sort();
    report.duplicates.sort();
    report.duplicates.dedup();

    report
}
