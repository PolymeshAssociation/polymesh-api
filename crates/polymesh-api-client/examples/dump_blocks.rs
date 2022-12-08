use std::collections::BTreeMap;
use std::env;

use anyhow::{anyhow, Result};

use polymesh_api_client::schema::*;
use polymesh_api_client::*;

lazy_static::lazy_static! {
  static ref SYSTEM_EVENTS_KEY: StorageKey = {
    StorageKey(hex::decode("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").unwrap())
  };
}

#[derive(Clone, Debug, Default)]
pub struct BlockData {
  pub number: BlockNumber,
  pub hash: Option<BlockHash>,
  pub version: Option<RuntimeVersion>,
  pub block: Option<Block>,
  pub events: Option<StorageData>,
}

impl BlockData {
  pub async fn get_block(client: &Client, number: BlockNumber) -> Result<Self> {
    let mut data = Self {
      number,
      ..Default::default()
    };
    // Get block hash
    data.hash = client.get_block_hash(number).await?;

    if data.hash.is_some() {
      // Get chain runtime version at this block.
      data.version = client.get_block_runtime_version(data.hash).await?;

      // Get block.
      data.block = client.get_block(data.hash).await?;

      // Get block events
      data.events = client
        .get_storage_data_by_key(SYSTEM_EVENTS_KEY.clone(), data.hash)
        .await?;
    }
    Ok(data)
  }
}

type RxBlockNumber = tokio::sync::mpsc::Receiver<BlockNumber>;
type TxBlockNumber = tokio::sync::mpsc::Sender<BlockNumber>;

type RxBlockData = tokio::sync::mpsc::Receiver<BlockData>;
type TxBlockData = tokio::sync::mpsc::Sender<BlockData>;

struct GetBlocksWorker {
  client: Client,
  rx: RxBlockNumber,
  tx_data: TxBlockData,
}

impl GetBlocksWorker {
  pub async fn new(url: &str, tx_data: TxBlockData) -> Result<TxBlockNumber> {
    let (tx, rx) = tokio::sync::mpsc::channel(1000);
    let client = Client::new(url).await?;
    let worker = Self {
      client,
      rx,
      tx_data,
    };
    worker.spawn();

    Ok(tx)
  }

  fn spawn(self) {
    tokio::spawn(async move {
      if let Err(err) = self.run().await {
        log::error!("GetBlocksWorker error: {err:?}");
      }
    });
  }

  async fn run(mut self) -> Result<()> {
    while let Some(number) = self.rx.recv().await {
      let block = BlockData::get_block(&self.client, number).await?;
      self.tx_data.send(block).await?;
    }
    log::debug!("Get Block worker finished.");

    Ok(())
  }
}

struct GetBlocksWorkerPool {
  rx: RxBlockNumber,
  tx_data: TxBlockData,
  url: String,
  max_workers: usize,
  workers: Vec<TxBlockNumber>,
}

impl GetBlocksWorkerPool {
  pub async fn new(max_workers: usize, url: &str, tx_data: TxBlockData) -> Result<TxBlockNumber> {
    let (tx, rx) = tokio::sync::mpsc::channel(1000);
    let pool = Self {
      rx,
      tx_data,
      url: url.into(),
      max_workers,
      workers: Vec::new(),
    };
    pool.spawn();

    Ok(tx)
  }

  fn spawn(self) {
    tokio::spawn(async move {
      if let Err(err) = self.run().await {
        log::error!("GetBlocksWorkerPool error: {err:?}");
      }
    });
  }

  async fn run(mut self) -> Result<()> {
    // Init workers.
    for _ in 0..self.max_workers {
      self
        .workers
        .push(GetBlocksWorker::new(&self.url, self.tx_data.clone()).await?);
    }

    let mut next_worker = 0;
    while let Some(number) = self.rx.recv().await {
      next_worker += 1;
      if next_worker >= self.workers.len() {
        next_worker = 0;
      }
      self.workers[next_worker].send(number).await?;
    }
    log::debug!("Pool worker finished.");

    Ok(())
  }
}

struct ProcessBlocksWorker {
  blocks: BTreeMap<BlockNumber, BlockData>,
  next_block: BlockNumber,
  skip: BlockNumber,
  rx: RxBlockData,
}

impl ProcessBlocksWorker {
  pub fn new(next_block: BlockNumber, skip: BlockNumber) -> (Self, TxBlockData) {
    let (tx, rx) = tokio::sync::mpsc::channel(1000);
    (
      Self {
        blocks: BTreeMap::new(),
        next_block,
        skip,
        rx,
      },
      tx,
    )
  }

  async fn recv(&mut self) -> Option<BlockData> {
    self.rx.recv().await
  }

  pub async fn next_block(&mut self) -> Option<BlockData> {
    let number = self.next_block;
    self.next_block += self.skip;
    // Check if we already received the block.
    let block = self.blocks.remove(&number);
    if block.is_some() {
      // Already have the block.  Return it.
      return block;
    }
    // Keep receiving block, until we get the block we need.
    while let Some(block) = self.recv().await {
      // Check if this is the block we want.
      if block.number == number {
        return Some(block);
      }
      // Not the block we want, save it for later.
      self.blocks.insert(block.number, block);
    }
    None
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  dotenv::dotenv().ok();
  env_logger::init();

  let url = env::args().nth(1).expect("Missing ws url");
  let start_block = env::args()
    .nth(2)
    .and_then(|v| v.parse().ok())
    .unwrap_or_else(|| 0);
  let count = env::args()
    .nth(3)
    .and_then(|v| v.parse().ok())
    .unwrap_or_else(|| 10);
  let end_block = start_block + count;
  let skip = env::args()
    .nth(4)
    .and_then(|v| v.parse().ok())
    .unwrap_or_else(|| 1);
  let num_workers = env::args()
    .nth(5)
    .and_then(|v| v.parse().ok())
    .unwrap_or_else(|| 8);

  let (mut process_blocks, tx) = ProcessBlocksWorker::new(start_block, skip);
  let worker_pool = GetBlocksWorkerPool::new(num_workers, &url, tx).await?;
  tokio::spawn(async move {
    let mut block_number = start_block;
    while block_number < end_block {
      if let Err(err) = worker_pool.send(block_number).await {
        log::error!("Failed to send block number to workers: {err:?}");
        break;
      }
      block_number += skip;
    }
    log::debug!("Finished sending block numbers");
  });

  // Types registery.
  let client = Client::new(&url).await?;
  let types_registry = TypesRegistry::new("./schemas/init_types.json".into(), "schema.json".into());

  let gen_hash = client
    .get_block_hash(0)
    .await?
    .ok_or_else(|| anyhow!("Can't get genesis hash"))?;
  let gen_version = client
    .get_block_runtime_version(Some(gen_hash))
    .await?
    .ok_or_else(|| anyhow!("Can't get runtime version for genesis block"))?;
  let mut stat_counter = 0;
  let mut last_number = 0;
  let mut last_spec = gen_version.spec_version;
  println!("---- Spec version: {}", last_spec);
  let mut last_types = types_registry
    .get_block_types(&client, Some(gen_version), Some(gen_hash))
    .await?;
  let event_records_ty = last_types.resolve("EventRecords");
  println!("event_records_ty = {:?}", event_records_ty);
  let mut event_records_ty = last_types
    .type_codec("EventRecords")
    .expect("Failed to get EventRecords type.");
  last_types.dump_unresolved();
  while let Some(block) = process_blocks.next_block().await {
    /*
    if block.number < last_number {
      println!("Out of order block: {} < {last_number}", block.number);
    }
    */
    if let Some(version) = &block.version {
      if version.spec_version != last_spec {
        last_spec = version.spec_version;
        println!("---- New spec version: {}", last_spec);
        last_types = types_registry
          .get_block_types(&client, block.version, block.hash)
          .await?;
        event_records_ty = last_types
          .type_codec("EventRecords")
          .expect("Failed to get EventRecords type.");
        last_types.dump_unresolved();
      }
    }
    if let Some(events) = block.events {
      let events = event_records_ty.decode(events.0)?;
      match events.as_array() {
        // Skip empty blocks.
        Some(events) if events.len() > 1 => {
          println!(
            "block[{}] events: {}",
            block.number,
            serde_json::to_string_pretty(&events)?
          );
        }
        Some(_) => (),
        None => {
          println!(
            "block[{}] events: {}",
            block.number,
            serde_json::to_string_pretty(&events)?
          );
        }
      }
    }
    last_number = block.number;
    if stat_counter >= 10000 {
      stat_counter = 0;
      println!("block[{}] = {:?}", block.number, block.hash);
    }
    stat_counter += skip;
  }
  println!("last block = {last_number}");

  Ok(())
}
