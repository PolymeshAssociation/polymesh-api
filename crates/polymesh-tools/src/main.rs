//! Polymesh Tools CLI
//!
//! Command-line interface for Polymesh chain operations and utilities.

use anyhow::Result;
use clap::{Parser, Subcommand};
use polymesh_tools::{committee_proposal, committee_upgrade, upgrade_chain};

#[derive(Parser)]
#[command(name = "polymesh-tools")]
#[command(version, about = "Helper tools for Polymesh chain operations", long_about = None)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// Upgrade the chain runtime
  UpgradeChain {
    /// Path to the WASM file containing the new runtime
    #[arg(value_name = "WASM_FILE")]
    wasm_file: String,
  },

  /// Committee-driven chain upgrade using UpgradeCommittee and PolymeshCommittee
  CommitteeUpgrade {
    /// Path to the WASM file containing the new runtime
    #[arg(value_name = "WASM_FILE")]
    wasm_file: String,

    /// Runtime release version used to build the proposal URL and description
    #[arg(value_name = "VERSION")]
    version: String,

    /// Committee member keys (private key hex, seed phrase, or //dev shortcut)
    #[arg(value_name = "COMMITTEE_KEY", required = true, num_args = 1..)]
    committee_keys: Vec<String>,
  },

  /// Committee-driven proposal for a hex encoded runtime call
  CommitteeProposal {
    /// Hex encoded runtime call to propose (with or without 0x prefix)
    #[arg(value_name = "PROPOSAL")]
    proposal: String,

    /// Committee member keys (private key hex, seed phrase, or //dev shortcut)
    #[arg(value_name = "COMMITTEE_KEY", required = true, num_args = 1..)]
    committee_keys: Vec<String>,

    /// Optional link to a website for proposal discussion
    #[arg(long, value_name = "URL")]
    url: Option<String>,

    /// Optional short description of the proposal
    #[arg(long, value_name = "DESCRIPTION")]
    description: Option<String>,
  },
}

#[tokio::main]
async fn main() -> Result<()> {
  env_logger::builder()
    .filter_level(log::LevelFilter::Info)
    .try_init()
    .ok();

  let cli = Cli::parse();

  match cli.command {
    Commands::UpgradeChain { wasm_file } => upgrade_chain(&wasm_file).await,
    Commands::CommitteeUpgrade {
      wasm_file,
      version,
      committee_keys,
    } => committee_upgrade(&wasm_file, &version, &committee_keys).await,
    Commands::CommitteeProposal {
      proposal,
      committee_keys,
      url,
      description,
    } => {
      committee_proposal(
        &proposal,
        &committee_keys,
        url.as_deref(),
        description.as_deref(),
      )
      .await
    }
  }
}
