// Generate Polymesh V5.x types from chain metadata.
#[super::codegen_api(metadata_file = "specs/polymesh_dev_spec_5004000.meta")]
pub mod polymesh {}

// V6 types.
use super::polymesh::types::polymesh_primitives::{
  identity_id::{
    PortfolioId as PortfolioIdV6, PortfolioKind as PortfolioKindV6,
    PortfolioNumber as PortfolioNumberV6,
  },
  portfolio::{Fund, FundDescription},
  settlement::Leg as LegV6,
  ticker::Ticker as TickerV6,
  Memo as MemoV6,
};
// V5 types.
pub use polymesh::types::{
  pallet_portfolio::MovePortfolioItem,
  pallet_settlement::Leg,
  polymesh_common_utilities::traits::balances,
  polymesh_primitives::{
    identity_id::{PortfolioId, PortfolioKind, PortfolioNumber},
    ticker::Ticker,
  },
};

impl From<balances::Memo> for MemoV6 {
  fn from(other: balances::Memo) -> Self {
    Self(other.0)
  }
}

impl From<Ticker> for TickerV6 {
  fn from(other: Ticker) -> Self {
    Self(other.0)
  }
}

impl From<TickerV6> for Ticker {
  fn from(other: TickerV6) -> Self {
    Self(other.0)
  }
}

impl From<PortfolioId> for PortfolioIdV6 {
  fn from(other: PortfolioId) -> Self {
    Self {
      did: other.did,
      kind: other.kind.into(),
    }
  }
}

impl From<PortfolioIdV6> for PortfolioId {
  fn from(other: PortfolioIdV6) -> Self {
    Self {
      did: other.did,
      kind: other.kind.into(),
    }
  }
}

impl From<PortfolioKind> for PortfolioKindV6 {
  fn from(other: PortfolioKind) -> Self {
    match other {
      PortfolioKind::Default => Self::Default,
      PortfolioKind::User(num) => Self::User(num.into()),
    }
  }
}

impl From<PortfolioKindV6> for PortfolioKind {
  fn from(other: PortfolioKindV6) -> Self {
    match other {
      PortfolioKindV6::Default => Self::Default,
      PortfolioKindV6::User(num) => Self::User(num.into()),
    }
  }
}

impl From<PortfolioNumber> for PortfolioNumberV6 {
  fn from(other: PortfolioNumber) -> Self {
    Self(other.0)
  }
}

impl From<PortfolioNumberV6> for PortfolioNumber {
  fn from(other: PortfolioNumberV6) -> Self {
    Self(other.0)
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
