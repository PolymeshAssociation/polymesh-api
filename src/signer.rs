use codec::Encode;

use sp_core::{
  sr25519,
  Pair,
};
use sp_runtime::generic::Era;

use crate::*;

pub struct SimpleSigner {
  pub pair: sr25519::Pair,
  pub nonce: u32,
  pub account: AccountId,
}

impl SimpleSigner {
  pub fn new(pair: sr25519::Pair) -> Self {
    let account = AccountId::new(pair.public().into());
    Self {
      pair,
      nonce: 0,
      account,
    }
  }

  pub async fn submit_and_watch<'api, Api: ChainApi>(
    &mut self,
    call: &Call<'api, Api>,
  ) -> Result<TransactionResults<'api, Api>> {
    let client = call.api.client();
    // Query account nonce.
    if self.nonce == 0 {
      self.nonce = call.api.get_nonce(self.account.clone()).await?;
    }

    let encoded_call = call.encoded();
    let extra = Extra::new(Era::Immortal, self.nonce);
    let payload = SignedPayload::new(&encoded_call, &extra, client.get_signed_extra());

    let sig = payload.using_encoded(|p| self.pair.sign(p));

    let xt = ExtrinsicV4::signed(self.account.clone(), sig.into(), extra, encoded_call);

    let res = call.submit_and_watch(xt).await?;

    // Update nonce if the call was submitted.
    self.nonce += 1;

    Ok(res)
  }
}


