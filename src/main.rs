use std::env;
use std::error::Error;
use std::fs::{create_dir_all, File, remove_dir_all};
use std::io::{BufReader, BufWriter, copy};
use std::path::{Path, PathBuf};
use std::process::Command;

use colored::*;
use reqwest::blocking::get;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "openapi_cli", about = "A CLI for OpenAPI")]
enum Cli {
    #[structopt(about = "Generate code from OpenAPI specification")]
    Generate {
        /// OpenAPI generator version
        #[structopt(short = "v", long = "version", default_value = "6.2.0")]
        version: String,

        /// OpenAPI specification file
        #[structopt(short = "i", long = "input", parse(from_os_str))]
        spec_file: PathBuf,

        /// Output directory for the generated code
        #[structopt(short = "o", long = "output", parse(from_os_str))]
        output_dir: PathBuf,

        /// Target language for the generated code
        #[structopt(short = "l", long = "language")]
        language: String,

        /// Configuration options for the generated code
        #[structopt(long = "config-options")]
        config_options: Option<Vec<String>>,

        /// Configuration values for the generated code
        #[structopt(long = "config-values")]
        config_values: Option<Vec<String>>,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    match Cli::from_args() {
        Cli::Generate { version, spec_file, output_dir, language, config_values, config_options } => {
            println!("{}", "🚀 Starting the OpenAPI code generation...".cyan());

            let openapi_generator_jar_url = format!("https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/{}/openapi-generator-cli-{}.jar", &version, &version);
            let build_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "./target".into());
            let openapi_generator_jar_file = format!("{}/openapi-generator-cli-{}.jar", build_dir, &version);

            if !PathBuf::from(&openapi_generator_jar_file).exists() {
                println!("{}", "📥 OpenAPI generator JAR file not found. Downloading...".yellow());
                download_file(&openapi_generator_jar_url, &openapi_generator_jar_file)?;
                println!("{}", "📦 OpenAPI generator JAR downloaded successfully.".green());
            } else {
                println!("{}", "OpenAPI generator JAR file found.".green());
            }

            generate_code(&openapi_generator_jar_file, &spec_file, &output_dir, &language, config_options, config_values)
        }
    }
}

fn download_file(url: &str, dest_path: &str) -> Result<(), Box<dyn Error>> {
    // Ensure the parent directory exists
    let parent_dir = Path::new(dest_path).parent().ok_or("Invalid destination path")?;
    if !parent_dir.exists() {
        create_dir_all(&parent_dir)?;
    }

    println!("{}", format!("📥 Downloading file from {}...", url).yellow());
    let response = get(url)?;
    let mut source = BufReader::new(response);
    let mut dest = BufWriter::new(File::create(dest_path)?);
    copy(&mut source, &mut dest)?;
    println!("{}", format!("📦 File downloaded and saved to {}!", dest_path).yellow());
    Ok(())
}

fn generate_code(
    jar_file: &str,
    spec_file: &PathBuf,
    output_dir: &PathBuf,
    language: &str,
    config_options: Option<Vec<String>>,
    config_values: Option<Vec<String>>,
) -> Result<(), Box<dyn Error>> {
    // Try to remove the output directory
    if output_dir.exists() {
        println!("{}", "Removing existing output directory...".yellow());
        remove_dir_all(output_dir)?;
    }

    println!("{}", "Generating code from OpenAPI specification...".cyan());

    let mut cmd = Command::new("java");
    cmd.arg("-jar")
        .arg(jar_file)
        .arg("generate")
        .arg("-i")
        .arg(spec_file.as_os_str())
        .arg("-g")
        .arg(language)
        .arg("-o")
        .arg(output_dir.as_os_str());

    if let Some(options) = config_options {
        for option in options {
            cmd.arg(option);
        }
    }

    if let Some(values) = config_values {
        for value in values {
            cmd.arg(value);
        }
    }

    let status = cmd.status()?;

    if !status.success() {
        return Err("Failed to generate code.".into());
    }

    println!("{}", "✅ Code generation successful. Check the output directory for the generated code.".green());
    Ok(())
}

