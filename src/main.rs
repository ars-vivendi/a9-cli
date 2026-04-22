use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser, Clone, ValueEnum)]
enum InstallTool {
    /// Install a9-lint
    Lint,
    /// Install a9-prettyplease
    Prettyplease,
}

/// Arguments for install subcommand
#[derive(Parser)]
struct InstallArgs {
    /// Which tool to install (lint, prettyplease)
    tool: InstallTool,
    /// Git tag to install
    #[arg(long)]
    tag: Option<String>,
    /// Force reinstall
    #[arg(long)]
    force: bool,
}

#[derive(Parser)]
enum Commands {
    /// Install an a9 tool
    Install(InstallArgs),
}

/// CLI for a9 tools (a9-lint, etc)
#[derive(Parser)]
struct Cli {
    /// Output as JSON
    #[arg(global = true, long)]
    json: bool,
    #[command(subcommand)]
    command: Commands,
}

/// Result of install operation
#[derive(Serialize, Deserialize)]
struct InstallResult {
    success: bool,
    message: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Install(args) => {
            let result = handle_install(args, cli.json);

            if cli.json {
                println!("{}", serde_json::to_string(&result).unwrap());
            } else {
                println!("{}", result.message);
            }
        }
    }
}

fn handle_install(args: &InstallArgs, _json: bool) -> InstallResult {
    let tool_name = match &args.tool {
        InstallTool::Lint => "a9-lint",
        InstallTool::Prettyplease => "a9-prettyplease",
    };

    InstallResult {
        success: false,
        message: format!(
            "not_implemented: install {} {:?} force={}",
            tool_name, args.tag, args.force
        ),
    }
}
