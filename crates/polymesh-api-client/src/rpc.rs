use jsonrpsee::core::{
  client::{BatchResponse, ClientT, Subscription, SubscriptionClientT},
  params::{ArrayParams, BatchRequestBuilder},
};
#[cfg(target_arch = "wasm32")]
use jsonrpsee::wasm_client::{Client as WasmClient, WasmClientBuilder};
#[cfg(not(target_arch = "wasm32"))]
use jsonrpsee::{
  http_client::{HttpClient, HttpClientBuilder},
  ws_client::{WsClient, WsClientBuilder},
};

use sp_std::prelude::*;

#[cfg(feature = "serde")]
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
      if url.starts_with("http") {
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
    // Check if url has a `port`.  The websocket backend doesn't support a default port.
    let url = url.parse::<http::Uri>()?;
    let url = if url.port().is_none() {
      // Need to rebuild the url to add the default port `80` (ws) or `443` (wss).
      let (scheme, port) = match url.scheme().map(|s| s.as_str()) {
        Some("wss") => ("wss", 443),
        _ => ("ws", 80),
      };
      let host = url.authority().map(|a| a.as_str()).unwrap_or_else(|| "");
      let authority = format!("{}:{}", host, port);
      let path = url
        .path_and_query()
        .map(|p| p.as_str())
        .unwrap_or_else(|| "");
      let url = http::Uri::builder()
        .scheme(scheme)
        .authority(authority)
        .path_and_query(path)
        .build()?;
      url.to_string()
    } else {
      url.to_string()
    };
    let client = WsClientBuilder::default()
      .max_concurrent_requests(16 * 1024)
      .max_request_size(1024 * 1024 * 1024)
      .build(&url)
      .await?;
    Ok(Self {
      client: InnerRpcClient::Ws(client),
    })
  }

  #[cfg(not(target_arch = "wasm32"))]
  fn new_http(url: &str) -> Result<Self> {
    let client = HttpClientBuilder::default()
      .max_request_size(1024 * 1024 * 1024)
      .build(&url)?;
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

  #[cfg(feature = "serde")]
  pub async fn subscribe<'a, Notif>(
    &self,
    subscribe_method: &'a str,
    params: ArrayParams,
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

  #[cfg(feature = "serde")]
  pub async fn request<'a, R>(&self, method: &'a str, params: ArrayParams) -> Result<R>
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

  #[cfg(feature = "serde")]
  pub async fn batch_request<'a, R>(
    &self,
    batch: BatchRequestBuilder<'a>,
  ) -> Result<BatchResponse<'a, R>>
  where
    R: DeserializeOwned + Default + Clone + alloc::fmt::Debug + 'a,
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
