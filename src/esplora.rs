use std::{cell::RefCell, collections::BTreeSet, io::Write, rc::Rc};

use wasm_bindgen::prelude::*;

use crate::bitcoin::Transaction;
use crate::error::EsploraError;
use crate::types::Update;
use crate::types::{FullScanRequestWrapper, SyncRequest, UpdateWrapper};

use bdk_esplora::esplora_client::{BlockingClient, Builder};
use bdk_esplora::EsploraExt;
use bdk_wallet::bitcoin::Transaction as BdkTransaction;
use bdk_wallet::bitcoin::Txid;
use bdk_wallet::chain::spk_client::FullScanRequest as BdkFullScanRequest;
use bdk_wallet::chain::spk_client::FullScanResult as BdkFullScanResult;
use bdk_wallet::chain::spk_client::SyncRequest as BdkSyncRequest;
use bdk_wallet::chain::spk_client::SyncResult as BdkSyncResult;
use bdk_wallet::KeychainKind;
use bdk_wallet::Update as BdkUpdate;

use std::collections::BTreeMap;
use std::str::FromStr;
use std::sync::Arc;

#[wasm_bindgen]
pub struct EsploraClient(BlockingClient);

#[wasm_bindgen]
pub struct EsploraClientWrapper(Rc<RefCell<EsploraClient>>);

#[wasm_bindgen]
impl EsploraClientWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(url: String) -> Self {
        let client = Builder::new(url.as_str()).build_blocking();
        Self(Rc::new(RefCell::new(EsploraClient(client))))
    }

    #[wasm_bindgen]
    pub fn full_scan(
        &self,
        request: FullScanRequestWrapper,
        stop_gap: u64,
        parallel_requests: u64,
    ) -> Result<UpdateWrapper, String> {
        let request: BdkFullScanRequest<KeychainKind> = request.get().into();

        let result = self
            .0
            .borrow()
            .0
            .full_scan(
                request,
                stop_gap as usize,
                parallel_requests as usize
            )
            .map_err(|e| format!("{:?}", e))?;

        let update = BdkUpdate {
          last_active_indices: result.last_active_indices,
          tx_update: result.tx_update,
          chain: result.chain_update,
        };

        Ok(UpdateWrapper::new(Update(update)))
    }
}
