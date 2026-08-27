use api_example_linter::{
    CheckOptions, FileConfig, OutputFormat, check, error_json, load_config, render_report,
    write_starter_config,
};
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "api-example-linter", version, about = "Keep human-facing API examples aligned with OpenAPI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Validate Markdown and OpenAPI examples
    Check(CheckArgs),
    /// Write a starter .api-example-linter.json without overwriting
    Init {
        #[arg(default_value = ".api-example-linter.json")]
        path: PathBuf,
    },
}

#[derive(Args)]
struct CheckArgs {
    /// Markdown file, OpenAPI document, or directory (repeatable)
    #[arg(value_name = "INPUT")]
    inputs: Vec<PathBuf>,
    /// OpenAPI 3.x JSON or YAML contract
    #[arg(long)]
    spec: Option<PathBuf>,
    /// Match an OpenAPI operationId
    #[arg(long, conflicts_with = "schema")]
    operation: Option<String>,
    /// Match a components.schemas name
    #[arg(long, conflicts_with = "operation")]
    schema: Option<String>,
    /// Validate the operation request or successful response
    #[arg(long)]
    direction: Option<api_example_linter::Direction>,
    /// Output for humans, scripts, or GitHub Actions
    #[arg(long, value_enum)]
    format: Option<OutputFormat>,
    /// Send valid request examples to this opt-in HTTP mock server
    #[arg(long)]
    mock_base_url: Option<String>,
    /// Explicitly allow an additional mock hostname (repeatable)
    #[arg(long)]
    allow_host: Vec<String>,
    /// Configuration file; loaded automatically when present
    #[arg(long, default_value = ".api-example-linter.json")]
    config: PathBuf,
    /// Ignore any configuration file
    #[arg(long)]
    no_config: bool,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Init { path } => match write_starter_config(&path) {
            Ok(()) => println!("Wrote {}", path.display()),
            Err(error) => {
                eprintln!("error: {error}");
                std::process::exit(2);
            }
        },
        Command::Check(args) => run_check(args),
    }
}

fn run_check(args: CheckArgs) {
    let config = if !args.no_config && args.config.exists() {
        match load_config(&args.config) {
            Ok(value) => value,
            Err(error) => {
                emit_error(&error.message, args.format.unwrap_or_default());
            }
        }
    } else {
        FileConfig::default()
    };
    let format = args.format.or(config.format).unwrap_or_default();
    let options = CheckOptions {
        inputs: if args.inputs.is_empty() {
            config.inputs
        } else {
            args.inputs
        },
        spec: args.spec.or(config.spec),
        operation: args.operation.or(config.operation),
        schema: args.schema.or(config.schema),
        direction: args.direction.or(config.direction),
        format,
        mock_base_url: args.mock_base_url.or(config.mock_base_url),
        allow_hosts: if args.allow_host.is_empty() {
            config.allow_hosts
        } else {
            args.allow_host
        },
    };
    match check(&options) {
        Ok(report) => {
            let resolved = if format == OutputFormat::Auto
                && std::env::var("GITHUB_ACTIONS").as_deref() == Ok("true")
            {
                OutputFormat::Github
            } else if format == OutputFormat::Auto {
                OutputFormat::Text
            } else {
                format
            };
            print!("{}", render_report(&report, resolved));
            if !report.diagnostics.is_empty() {
                std::process::exit(1);
            }
        }
        Err(error) => emit_error(&error.message, format),
    }
}

fn emit_error(message: &str, format: OutputFormat) -> ! {
    if format == OutputFormat::Json {
        eprint!("{}", error_json(message));
    } else if format == OutputFormat::Github
        || (format == OutputFormat::Auto
            && std::env::var("GITHUB_ACTIONS").as_deref() == Ok("true"))
    {
        eprintln!(
            "::error title=CONFIGURATION_ERROR::{}",
            message.replace('%', "%25").replace('\n', "%0A")
        );
    } else {
        eprintln!("error: {message}");
    }
    std::process::exit(2)
}
