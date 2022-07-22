use jsonrpsee::core::client::{ClientT, Subscription, SubscriptionClientT};
use jsonrpsee::rpc_params;
use jsonrpsee::types::ParamsSer;
use jsonrpsee::ws_client::{WsClient, WsClientBuilder};
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};

use hex::FromHex;
use serde::de::DeserializeOwned;

use codec::Decode;
use frame_metadata::RuntimeMetadataPrefixed;
use sp_version::RuntimeVersion;

use crate::block::BlockHash;
use crate::error::*;

#[derive(Debug)]
enum InnerRpcClient {
  Ws(WsClient),
  Http(HttpClient),
}

#[derive(Debug)]
pub struct RpcClient {
  client: InnerRpcClient,
}

impl RpcClient {
  pub async fn new(url: &str) -> Result<Self> {
    if url.starts_with("Http") {
      Self::new_http(url)
    } else if url.starts_with("ws") {
      Self::new_ws(url).await
    } else {
      Err(Error::RpcClient(format!("Unsupported url: {url}")))
    }
  }

  async fn new_ws(url: &str) -> Result<Self> {
    let client = WsClientBuilder::default().build(&url).await?;
    Ok(Self {
      client: InnerRpcClient::Ws(client),
    })
  }

  fn new_http(url: &str) -> Result<Self> {
    let client = HttpClientBuilder::default().build(&url)?;
    Ok(Self {
      client: InnerRpcClient::Http(client),
    })
  }

  pub async fn subscribe<'a, Notif>(
    &self,
    subscribe_method: &'a str,
    params: Option<ParamsSer<'a>>,
    unsubscribe_method: &'a str,
  ) -> Result<Subscription<Notif>>
  where
    Notif: DeserializeOwned,
  {
    Ok(match &self.client {
      InnerRpcClient::Ws(ws) => ws.subscribe(subscribe_method, params, unsubscribe_method).await,
      InnerRpcClient::Http(http) => http.subscribe(subscribe_method, params, unsubscribe_method).await,
    }?)
  }

  pub async fn request<'a, R>(
    &self,
    method: &'a str,
    params: Option<ParamsSer<'a>>,
  ) -> Result<R>
  where
    R: DeserializeOwned,
  {
    Ok(match &self.client {
      InnerRpcClient::Ws(ws) => ws.request(method, params).await,
      InnerRpcClient::Http(http) => http.request(method, params).await,
    }?)
  }

  pub async fn batch_request<'a, R>(
    &self,
    batch: Vec<(&'a str, Option<ParamsSer<'a>>)>,
  ) -> Result<Vec<R>>
  where
    R: DeserializeOwned + Default + Clone,
  {
    Ok(match &self.client {
      InnerRpcClient::Ws(ws) => ws.batch_request(batch).await,
      InnerRpcClient::Http(http) => http.batch_request(batch).await,
    }?)
  }

  pub async fn get_block_hash(&self, block_number: u32) -> Result<BlockHash> {
    let params = rpc_params!(block_number);
    Ok(self.request("chain_getBlockHash", params).await?)
  }

  pub async fn get_runtime_version(&self, block: Option<BlockHash>) -> Result<RuntimeVersion> {
    let params = rpc_params!(block);
    Ok(self.request("state_getRuntimeVersion", params).await?)
  }

  pub async fn get_metadata(&self, block: Option<BlockHash>) -> Result<RuntimeMetadataPrefixed> {
    let params = rpc_params!(block);
    let hex: String = self.request("state_getMetadata", params).await?;

    let bytes = Vec::from_hex(&hex[2..])?;
    Ok(RuntimeMetadataPrefixed::decode(&mut bytes.as_slice())?)
  }
}
