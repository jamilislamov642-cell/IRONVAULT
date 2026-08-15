use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ironvault")]
#[command(about = "IRONVAULT minimal CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new vault at PATH
    Create {
        /// Path to create the vault in
        path: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Create { path } => {
            println!("Creating vault at {}", path);
            ironvault_core::create_vault(path, "please-change-me")?;
            println!("Vault created. Change the default password using the GUI or CLI.");
        }
    }
    Ok(())
}
