use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ironvault")]
#[command(about = "IRONVAULT CLI", long_about = None)]
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
    /// Import a file into a vault
    Import {
        /// Path to the vault
        vault: String,
        /// Path of file to import
        file: String,
        /// Optional parent node id
        #[arg(short, long)]
        parent: Option<String>,
    },
    /// Export a node by ID to destination directory
    Export {
        vault: String,
        node_id: String,
        dest: String,
    },
    /// List nodes in vault (root-level)
    List {
        vault: String,
    },
}

fn prompt_password() -> anyhow::Result<String> {
    let pwd = rpassword::prompt_password_stdout("Master password: ")?;
    Ok(pwd)
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Create { path } => {
            println!("Creating vault at {}", path);
            let pwd = rpassword::prompt_password_stdout("Enter master password: ")?;
            let pwd2 = rpassword::prompt_password_stdout("Confirm master password: ")?;
            if pwd != pwd2 { anyhow::bail!("passwords do not match"); }
            ironvault_core::create_vault(path, &pwd)?;
            println!("Vault created.");
        }
        Commands::Import { vault, file, parent } => {
            let pwd = prompt_password()?;
            let session = ironvault_core::unlock_vault(vault, &pwd)?;
            let id = ironvault_core::import_file(&session, file, parent.as_deref())?;
            println!("Imported node id: {}", id);
            session.lock();
        }
        Commands::Export { vault, node_id, dest } => {
            let pwd = prompt_password()?;
            let session = ironvault_core::unlock_vault(vault, &pwd)?;
            ironvault_core::export_file(&session, node_id, dest)?;
            println!("Exported node {}", node_id);
            session.lock();
        }
        Commands::List { vault } => {
            let pwd = prompt_password()?;
            let session = ironvault_core::unlock_vault(vault, &pwd)?;
            let nodes = ironvault_core::list_nodes(&session)?;
            for (id, parent, name) in nodes {
                println!("{}\tparent={}\tname={}", id, parent.unwrap_or_default(), name);
            }
            session.lock();
        }
    }
    Ok(())
}
