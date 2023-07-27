#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use polymesh_api_codegen_macro::*;

#[cfg_attr(not(feature = "download_metadata"), codegen_api(metadata_file = "specs/polymesh_dev_spec_6000000.meta"))]
#[cfg_attr(feature = "download_metadata", codegen_api(metadata_url = "ws://localhost:9944"))]
mod polymesh {}

pub use polymesh::*;

// re-export core client and common types.
#[cfg(feature = "rpc")]
pub use polymesh_api_client as client;
#[cfg(feature = "rpc")]
pub use polymesh_api_client::{ChainApi, Client};

#[cfg(feature = "ink")]
pub use polymesh_api_ink as ink;

#[cfg(feature = "polymesh_v5")]
pub mod v5_to_v6 {
  // Generate Polymesh V5.x types from chain metadata.
  #[super::codegen_api(metadata_file = "specs/polymesh_dev_spec_5004000.meta")]
  pub mod polymesh {}

  // V6 types.
  use super::polymesh::types::{
      polymesh_primitives::{
          identity_id::{
            PortfolioId as PortfolioIdV6,
            PortfolioKind as PortfolioKindV6,
            PortfolioNumber as PortfolioNumberV6,
          },
          portfolio::{Fund, FundDescription},
          settlement::Leg as LegV6,
          ticker::Ticker as TickerV6,
          Memo as MemoV6,
      },
  };
  // V5 types.
  pub use polymesh::types::{
      pallet_portfolio::MovePortfolioItem,
      pallet_settlement::Leg,
      polymesh_common_utilities::traits::balances,
      polymesh_primitives::{
          identity_id::{
              PortfolioId,
              PortfolioKind,
              PortfolioNumber,
          },
          ticker::Ticker,
      },
  };

  impl From<balances::Memo> for MemoV6 {
    fn from(old: balances::Memo) -> Self {
      Self(old.0)
    }
  }

  impl From<Ticker> for TickerV6 {
    fn from(old: Ticker) -> Self {
      Self(old.0)
    }
  }

  impl From<PortfolioId> for PortfolioIdV6 {
    fn from(old: PortfolioId) -> Self {
      Self {
        did: old.did,
        kind: old.kind.into(),
      }
    }
  }

  impl From<PortfolioKind> for PortfolioKindV6 {
    fn from(old: PortfolioKind) -> Self {
      match old {
        PortfolioKind::Default => Self::Default,
        PortfolioKind::User(num) => Self::User(num.into()),
      }
    }
  }

  impl From<PortfolioNumber> for PortfolioNumberV6 {
    fn from(old: PortfolioNumber) -> Self {
      Self(old.0)
    }
  }

  impl From<MovePortfolioItem> for Fund {
    fn from(old: MovePortfolioItem) -> Self {
      Self {
        description: FundDescription::Fungible {
          ticker: old.ticker.into(),
          amount: old.amount,
        },
        memo: old.memo.map(|m| m.into()),
      }
    }
  }

  impl From<Leg> for LegV6 {
    fn from(old: Leg) -> Self {
      Self::Fungible {
        sender: old.from.into(),
        receiver: old.to.into(),
        ticker: old.asset.into(),
        amount: old.amount,
      }
    }
  }
}
