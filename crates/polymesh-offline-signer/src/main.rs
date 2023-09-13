use anyhow::{anyhow, Result};

use codec::{Decode, Encode};

use serde_json::to_string;

use rust_decimal::prelude::*;

use polymesh_api::client::{AccountId, ChainApi, DefaultSigner, ExtrinsicV4, PreparedTransaction};
use polymesh_api::Api;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// Prepare a transaction for offline signing.
  Prepare(PrepareArgs),
  /// Sign a prepared transaction offline.
  OfflineSign(OfflineSignArgs),
  /// Submit a signed transaction.
  Submit(SubmitArgs),
}

#[derive(Args)]
struct PrepareArgs {
  #[arg(short, long, value_parser = decode_account)]
  account: AccountId,
  #[arg(short, long)]
  url: String,
  #[command(subcommand)]
  command: PrepareCommands,
}

#[derive(Clone, Subcommand)]
enum PrepareCommands {
  /// Prepare a polyx transfer.
  BalanceTransfer {
    #[arg(value_parser = decode_account)]
    dest: AccountId,
    amount: Decimal,
  },
}

#[derive(Args)]
struct OfflineSignArgs {
  #[arg(long = "suri", value_parser = decode_signer)]
  signer: DefaultSigner,
  #[arg(value_parser = decode_prepared_transaction)]
  transaction: PreparedTransaction,
}

#[derive(Args)]
struct SubmitArgs {
  #[arg(short, long)]
  url: String,
  #[arg(short, long, default_value = "false")]
  finalized: bool,
  #[arg(value_parser = decode_extrinsic_v4)]
  transaction: ExtrinsicV4,
}

fn decode_signer(s: &str) -> Result<DefaultSigner, String> {
  let signer =
    DefaultSigner::from_string(s, None).map_err(|e| format!("Failed to decode: {e:?}"))?;
  Ok(signer)
}

fn decode_account(s: &str) -> Result<AccountId, String> {
  if s.starts_with("0x") {
    let mut account = AccountId::default();
    hex::decode_to_slice(&s[2..], &mut account.0)
      .map_err(|e| format!("Invalid account id: {e:?}"))?;
    Ok(account)
  } else {
    use sp_core::crypto::Ss58Codec;
    let account = AccountId::from_ss58check(s).map_err(|e| format!("Invalid account id: {e:?}"))?;
    Ok(account)
  }
}

fn decode_prepared_transaction(s: &str) -> Result<PreparedTransaction, String> {
  let off = if s.starts_with("0x") { 2 } else { 0 };
  let buf =
    hex::decode(&s[off..]).map_err(|e| format!("Prepared transaction not valid hex: {e:?}"))?;
  let prepared = PreparedTransaction::decode(&mut buf.as_slice())
    .map_err(|e| format!("Invalid prepared transaction: {e:?}"))?;
  Ok(prepared)
}

fn decode_extrinsic_v4(s: &str) -> Result<ExtrinsicV4, String> {
  let off = if s.starts_with("0x") { 2 } else { 0 };
  let buf =
    hex::decode(&s[off..]).map_err(|e| format!("Signed transaction not valid hex: {e:?}"))?;
  let xt = ExtrinsicV4::decode(&mut buf.as_slice())
    .map_err(|e| format!("Invalid signed transaction: {e:?}"))?;
  Ok(xt)
}

async fn prepare(args: PrepareArgs) -> Result<()> {
  let api = Api::new(&args.url).await?;

  let scale: Decimal = Decimal::try_from_i128_with_scale(1_000_000i128, 0)?;
  let tx = match args.command {
    PrepareCommands::BalanceTransfer { dest, amount } => {
      let amount = (amount * scale)
        .to_u128()
        .ok_or_else(|| anyhow!("Failed to convert amount to u128."))?;
      api.call().balances().transfer(dest.into(), amount)?
    }
  };
  log::info!("tx = {:?}", to_string(&tx.runtime_call()));
  let prepared_tx = tx.prepare(args.account).await?;
  log::info!("json = {:?}", to_string(&prepared_tx));
  let encoded = prepared_tx.encode();
  println!("0x{}", hex::encode(encoded));
  Ok(())
}

async fn offline_sign(args: OfflineSignArgs) -> Result<()> {
  let mut signer = args.signer;
  let signed_tx = args.transaction.sign(&mut signer).await?;
  let encoded = signed_tx.encode();
  println!("0x{}", hex::encode(encoded));
  Ok(())
}

async fn submit(args: SubmitArgs) -> Result<()> {
  let api = Api::new(&args.url).await?;

  let mut res = api.submit_and_watch(args.transaction).await?;
  let block = if args.finalized {
    // Wait for the transaction to be finalized.
    res.wait_finalized().await?
  } else {
    // Wait for the transaction to be included in a block.
    res.wait_in_block().await?
  };
  if let Some(block) = block {
    println!("In block: {block:?}");
    match res.ok().await {
      Ok(()) => {
        println!("Successful");
      }
      Err(err) => {
        println!("Failed: {err:?}");
      }
    }
  }
  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let cli = Cli::parse();

  match cli.command {
    Commands::Prepare(args) => {
      prepare(args).await?;
    }
    Commands::OfflineSign(args) => {
      offline_sign(args).await?;
    }
    Commands::Submit(args) => {
      submit(args).await?;
    }
  }
  Ok(())
}
