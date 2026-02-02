mod error;
mod parse;
mod validate;

use clap::Parser;
use std::path::PathBuf;

use crate::error::AppResult;
use crate::parse::{parse_env_file, parse_req_keys};
use crate::validate::validate_env_file;

#[derive(Parser, Debug)]
#[command(
    name = "envcheck",
    version,
    about = "Check the environment variables of a project"
)]
struct Args {
    env_path: PathBuf,

    #[arg(long)]
    example: Option<PathBuf>,

    #[arg(long)]
    strict_empty: bool,
}
fn main() {
    let code = match run() {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Error: {e}");
            2
        }
    };

    std::process::exit(code);
}

fn run() -> AppResult<i32> {
    let args = Args::parse();

    let env = parse_env_file(&args.env_path)?;

    let required_keys = match args.example {
        Some(path) => parse_req_keys(&path)?,
        None => Vec::new(),
    };

    let report = validate_env_file(&env, &required_keys);

    if !required_keys.is_empty() {
        for key in &required_keys {
            if env.values.contains_key(key) {
                println!("key {key} is present in the environment file");
            } else {
                println!("key {key} is missing from the environment file");
            }
        }
    }

    if !report.missing.is_empty() {
        for key in &report.missing {
            println!("key {key} is missing from the environment file");
        }
    }

    if !report.empty.is_empty() {
        for key in &report.empty {
            println!("key {key} is empty in the environment file");
        }
    }

    if !report.duplicates.is_empty() {
        for key in &report.duplicates {
            println!("key {key} is duplicated in the environment file");
        }
    }

    if report.has_errors() {
        if args.strict_empty && !report.empty.is_empty() {
            return Ok(1);
        }
        return Ok(0);
    }

    Ok(0)
}
