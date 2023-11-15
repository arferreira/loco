//! command-line interface for running various tasks and commands
//! related to the application. It allows developers to interact with the
//! application via the command line.
//!
//! # Example
//!
//! ```rust,ignore
//! 
//! use myapp::app::App;
//! use framework::cli;
//! use migration::Migrator;
//!
//! #[tokio::main]
//! async fn main() {
//!     cli::main::<App, Migrator>().await
//! }
//! ```

use std::collections::BTreeMap;

use clap::{Parser, Subcommand};
use sea_orm_migration::MigratorTrait;

use crate::{
    app::Hooks,
    boot::{create_app, create_context, run_db, run_task, start, RunDbCommand, StartMode},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Specify the environment
    #[arg(short, long, global = true, default_value = "development")]
    environment: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Start an app
    Start {
        /// start worker
        #[arg(short, long, action)]
        worker: bool,
        /// start same-process server and worker
        #[arg(short, long, action)]
        server_and_worker: bool,
    },
    /// Perform DB operations
    Db {
        #[command(subcommand)]
        command: DbCommands,
    },
    /// Run a custom task
    Task {
        /// Task name (identifier)
        name: Option<String>,
        /// Task params (e.g. <my_task> foo=bar baz=qux)
        #[clap(value_parser = parse_key_val::<String,String>)]
        params: Vec<(String, String)>,
    },
}
#[derive(Subcommand)]
enum DbCommands {
    /// Migrate schema (up)
    Migrate,
    /// Drop all tables, then reapply all migrations
    Reset,
    /// Migration status
    Status,
    /// Generate entity .rs files from database schema
    Entities,
    /// Truncate data in tables (without dropping)
    Truncate,
}

impl From<DbCommands> for RunDbCommand {
    fn from(value: DbCommands) -> Self {
        match value {
            DbCommands::Migrate => Self::Migrate,
            DbCommands::Reset => Self::Reset,
            DbCommands::Status => Self::Status,
            DbCommands::Entities => Self::Entities,
            DbCommands::Truncate => Self::Truncate,
        }
    }
}

/// Parse a single key-value pair
fn parse_key_val<T, U>(s: &str) -> Result<(T, U), Box<dyn std::error::Error + Send + Sync>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: std::error::Error + Send + Sync + 'static,
{
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=value: no `=` found in `{s}`"))?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

/// # Main CLI Function
///
/// The `main` function is the entry point for the command-line interface (CLI)
/// of the application. It parses command-line arguments, interprets the
/// specified commands, and performs corresponding actions. This function is
/// generic over `H` and `M`, where `H` represents the application hooks and `M`
/// represents the migrator trait for handling database migrations.
///
/// # Errors
///
/// Returns an any error indicating success or failure during the CLI execution.
///
/// # Example
///
/// ```rust,ignore
/// use myapp::app::App;
/// use framework::cli;
/// use migration::Migrator;
///
/// #[tokio::main]
/// async fn main()  {
///     cli::main::<App, Migrator>().await
/// }
/// ```
pub async fn main<H: Hooks, M: MigratorTrait>() -> eyre::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Start {
            worker,
            server_and_worker,
        } => {
            let start_mode = if worker {
                StartMode::WorkerOnly
            } else if server_and_worker {
                StartMode::ServerAndWorker
            } else {
                StartMode::ServerOnly
            };

            let boot_result = create_app::<H, M>(start_mode, &cli.environment).await?;
            start(boot_result).await?;
        }
        Commands::Db { command } => {
            let app_context = create_context(&cli.environment).await?;
            run_db::<H, M>(&app_context, command.into()).await?;
        }
        Commands::Task { name, params } => {
            let mut hash = BTreeMap::new();
            for (k, v) in params {
                hash.insert(k, v);
            }
            let app_context = create_context(&cli.environment).await?;
            run_task::<H>(&app_context, name.as_ref(), &hash).await?;
        }
    }
    Ok(())
}
