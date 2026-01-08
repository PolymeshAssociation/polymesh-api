use std::fs::File;
use std::io::{stdin, Read};
use std::path::Path;

use anyhow::{anyhow, Result};

use codec::{Decode, Encode};

use serde_json::to_string;

use rust_decimal::prelude::*;

use polymesh_api::client::{
  AccountId, Call, ChainApi, DefaultSigner, ExtrinsicV4, PreparedTransaction,
};
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
  /// Sign a prepared transaction offline and return a full signed transaction.
  OfflineSign(OfflineSignArgs),
  /// Submit a signed transaction.
  Submit(SubmitArgs),
}

#[derive(Args)]
struct PrepareArgs {
  /// The account that will be used to sign the transaction.
  #[arg(short, long, value_parser = decode_account)]
  account: AccountId,
  /// Websocket url for the Polymesh node.
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
  /// Prepare an Identity.cddRegisterDidWithCdd.
  IdentityRegisterDid {
    #[arg(value_parser = decode_account)]
    primary_key: AccountId,
    #[arg(short, long)]
    expiry: Option<u64>,
  },
  /// Prepare a transaction from json.
  Json {
    /// Json encoded transaction to prepare (use '-' to read from stdin, or a filename).
    transaction: String,
  },
}

#[derive(Args)]
struct OfflineSignArgs {
  /// The secret key URI.
  #[arg(long = "suri", value_parser = decode_signer)]
  signer: DefaultSigner,
  /// Only return the signature, instead of the full signed transaction.
  #[arg(long)]
  only_signature: bool,
  /// Hex encoded prepared transaction to sign (use '-' to read from stdin, or a filename).
  #[arg(value_parser = decode_prepared_transaction)]
  transaction: PreparedTransaction,
}

#[derive(Args)]
struct SubmitArgs {
  /// Websocket url for the Polymesh node.
  #[arg(short, long)]
  url: String,
  /// Wait for the transaction to be finalized.
  #[arg(short, long, default_value = "false")]
  finalized: bool,
  /// Hex encoded signed transaction to submit (use '-' to read from stdin, or a filename).
  #[arg(value_parser = decode_extrinsic_v4)]
  transaction: ExtrinsicV4,
}

fn string_or_file(s: &str) -> Result<String> {
  match (s, Path::new(s)) {
    ("-", _) => {
      let mut buf = String::new();
      stdin()
        .read_to_string(&mut buf)
        .map_err(|e| anyhow!("Failed to read from stdin: {e:?}"))?;
      Ok(buf.trim().to_string())
    }
    (_, path) if path.exists() => {
      let mut f = File::open(path)?;
      let mut buf = String::new();
      f.read_to_string(&mut buf)
        .map_err(|e| anyhow!("Failed to read from file '{path:?}': {e:?}"))?;
      Ok(buf.trim().to_string())
    }
    (s, _) => Ok(s.to_string()),
  }
}

fn decode_signer(s: &str) -> Result<DefaultSigner> {
  let signer =
    DefaultSigner::from_string(s, None).map_err(|e| anyhow!("Failed to decode: {e:?}"))?;
  Ok(signer)
}

fn decode_account(s: &str) -> Result<AccountId> {
  if s.starts_with("0x") {
    let mut account = AccountId::default();
    hex::decode_to_slice(&s[2..], &mut account.0)
      .map_err(|e| anyhow!("Invalid account id: {e:?}"))?;
    Ok(account)
  } else {
    use sp_core::crypto::Ss58Codec;
    let account = AccountId::from_ss58check(s).map_err(|e| anyhow!("Invalid account id: {e:?}"))?;
    Ok(account)
  }
}

fn decode_prepared_transaction(s: &str) -> Result<PreparedTransaction> {
  let s = string_or_file(s)?;
  let off = if s.starts_with("0x") { 2 } else { 0 };
  let buf =
    hex::decode(&s[off..]).map_err(|e| anyhow!("Prepared transaction not valid hex: {e:?}"))?;
  let prepared = PreparedTransaction::decode_signed_payload::<Api, _>(&mut buf.as_slice())
    .map_err(|e| anyhow!("Invalid prepared transaction: {e:?}"))?;
  Ok(prepared)
}

fn decode_extrinsic_v4(s: &str) -> Result<ExtrinsicV4> {
  let s = string_or_file(s)?;
  let off = if s.starts_with("0x") { 2 } else { 0 };
  let buf =
    hex::decode(&s[off..]).map_err(|e| anyhow!("Signed transaction not valid hex: {e:?}"))?;
  let xt = ExtrinsicV4::decode(&mut buf.as_slice())
    .map_err(|e| anyhow!("Invalid signed transaction: {e:?}"))?;
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
      api
        .call()
        .balances()
        .transfer_with_memo(dest.into(), amount, None)?
    }
    PrepareCommands::IdentityRegisterDid {
      primary_key,
      expiry,
    } => api
      .call()
      .identity()
      .cdd_register_did_with_cdd(primary_key.into(), vec![], expiry)?,
    PrepareCommands::Json { transaction } => {
      let transaction = string_or_file(&transaction)?;
      let tx = serde_json::from_str(&transaction)?;
      Call::new(&api, tx)
    }
  };
  log::info!("tx = {:?}", to_string(&tx.runtime_call()));
  let prepared_tx = tx.prepare(args.account, None).await?;
  log::info!("json = {:?}", to_string(&prepared_tx));
  let encoded = prepared_tx.encode();
  println!("0x{}", hex::encode(encoded));
  Ok(())
}

async fn offline_sign(args: OfflineSignArgs) -> Result<()> {
  let mut signer = args.signer;
  let signed_tx = args.transaction.sign(&mut signer).await?;
  let encoded = if args.only_signature {
    let sig = signed_tx.signature.expect("Signed transaction");
    sig.signature.encode()
  } else {
    signed_tx.encode()
  };
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
