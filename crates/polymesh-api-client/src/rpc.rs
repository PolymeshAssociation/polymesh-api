use jsonrpsee::core::client::{ClientT, Subscription, SubscriptionClientT};
use jsonrpsee::types::ParamsSer;
#[cfg(target_arch = "wasm32")]
use jsonrpsee::wasm_client::{Client as WasmClient, WasmClientBuilder};
#[cfg(not(target_arch = "wasm32"))]
use jsonrpsee::{
  http_client::{HttpClient, HttpClientBuilder},
  ws_client::{WsClient, WsClientBuilder},
};

use serde::de::DeserializeOwned;

use crate::error::*;

#[derive(Debug)]
enum InnerRpcClient {
  #[cfg(not(target_arch = "wasm32"))]
  Ws(WsClient),
  #[cfg(not(target_arch = "wasm32"))]
  Http(HttpClient),
  #[cfg(target_arch = "wasm32")]
  Wasm(WasmClient),
}

#[derive(Debug)]
pub struct RpcClient {
  client: InnerRpcClient,
}

impl RpcClient {
  pub async fn new(url: &str) -> Result<Self> {
    #[cfg(not(target_arch = "wasm32"))]
    {
      if url.starts_with("Http") {
        Self::new_http(url)
      } else if url.starts_with("ws") {
        Self::new_ws(url).await
      } else {
        Err(Error::RpcClient(format!("Unsupported url: {url}")))
      }
    }
    #[cfg(target_arch = "wasm32")]
    Self::new_wasm(url).await
  }

  #[cfg(not(target_arch = "wasm32"))]
  async fn new_ws(url: &str) -> Result<Self> {
    let client = WsClientBuilder::default().build(&url).await?;
    Ok(Self {
      client: InnerRpcClient::Ws(client),
    })
  }

  #[cfg(not(target_arch = "wasm32"))]
  fn new_http(url: &str) -> Result<Self> {
    let client = HttpClientBuilder::default().build(&url)?;
    Ok(Self {
      client: InnerRpcClient::Http(client),
    })
  }

  #[cfg(target_arch = "wasm32")]
  async fn new_wasm(url: &str) -> Result<Self> {
    let client = WasmClientBuilder::default().build(&url).await?;
    Ok(Self {
      client: InnerRpcClient::Wasm(client),
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
      #[cfg(not(target_arch = "wasm32"))]
      InnerRpcClient::Ws(ws) => {
        ws.subscribe(subscribe_method, params, unsubscribe_method)
          .await
      }
      #[cfg(not(target_arch = "wasm32"))]
      InnerRpcClient::Http(http) => {
        http
          .subscribe(subscribe_method, params, unsubscribe_method)
          .await
      }
      #[cfg(target_arch = "wasm32")]
      InnerRpcClient::Wasm(http) => {
        http
          .subscribe(subscribe_method, params, unsubscribe_method)
          .await
      }
    }?)
  }

  pub async fn request<'a, R>(&self, method: &'a str, params: Option<ParamsSer<'a>>) -> Result<R>
  where
    R: DeserializeOwned,
  {
    Ok(match &self.client {
      #[cfg(not(target_arch = "wasm32"))]
      InnerRpcClient::Ws(ws) => ws.request(method, params).await,
      #[cfg(not(target_arch = "wasm32"))]
      InnerRpcClient::Http(http) => http.request(method, params).await,
      #[cfg(target_arch = "wasm32")]
      InnerRpcClient::Wasm(http) => http.request(method, params).await,
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
      #[cfg(not(target_arch = "wasm32"))]
      InnerRpcClient::Ws(ws) => ws.batch_request(batch).await,
      #[cfg(not(target_arch = "wasm32"))]
      InnerRpcClient::Http(http) => http.batch_request(batch).await,
      #[cfg(target_arch = "wasm32")]
      InnerRpcClient::Wasm(http) => http.batch_request(batch).await,
    }?)
  }
}
