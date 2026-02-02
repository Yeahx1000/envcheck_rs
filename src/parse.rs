use crate::error::{AppError, AppResult};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ParsedEnvFile {
    pub values: HashMap<String, String>,
    pub duplicates: Vec<String>,
}

// parse env file
// 1. read the file
// 2. parse the file
// 3. return the parsed file
// 4. handle errors
// 5. handle duplicates
// 6. handle comments
// 7. handle empty lines
// 8. handle whitespace
// 9. handle trailing whitespace

pub fn parse_env_file(path: &Path) -> AppResult<ParsedEnvFile> {
    let content = fs::read_to_string(path)?;
    let mut values: HashMap<String, String> = HashMap::new();
    let mut duplicates: Vec<String> = Vec::new();

    for (index, raw_line) in content.lines().enumerate() {
        let line = raw_line.trim();
        let line_number = index + 1;

        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        let line = if let Some(rem) = line.strip_prefix("export") {
            rem.trim()
        } else {
            line
        };

        let (key, value) = line.split_once("=").ok_or_else(|| AppError::ParseError {
            line: line_number,
            msg: "expected KEY=VALUE".to_string(),
        })?;

        let key = key.trim();
        if key.is_empty() {
            return Err(AppError::ParseError {
                line: line_number,
                msg: "empty key".to_string(),
            });
        }

        // validating key
        if !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(AppError::ParseError {
                line: line_number,
                msg: format!("invalid key: '{key}' (expected alphanumeric or underscore)"),
            });
        }

        let mut value = value.trim().to_string();

        // "unquoting" value

        if value.starts_with('"') && value.ends_with('"') && value.len() >= 2
            || value.starts_with('\'') && value.ends_with('\'') && value.len() >= 2
        {
            value = value[1..value.len() - 1].to_string();
        }

        if values.contains_key(key) {
            duplicates.push(key.to_string());
        }

        values.insert(key.to_string(), value);
    }

    Ok(ParsedEnvFile { values, duplicates })
}

pub fn parse_req_keys(path: &Path) -> AppResult<Vec<String>> {
    let parsed = parse_env_file(path)?;
    let mut keys: Vec<String> = parsed.values.keys().cloned().collect();
    keys.sort();
    keys.dedup();
    Ok(keys)
}
