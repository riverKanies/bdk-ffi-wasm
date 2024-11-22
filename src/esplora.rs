use std::{cell::RefCell, collections::BTreeSet, io::Write, rc::Rc};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::bitcoin::Transaction;
use crate::error::EsploraError;
use crate::types::Update;
use crate::types::{FullScanRequest, SyncRequest};

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
impl EsploraClient {
    #[wasm_bindgen(constructor)]
    pub fn new(url: String) -> Self {
        let client = Builder::new(url.as_str()).build_blocking();
        Self(client)
    }

    #[wasm_bindgen]
    pub fn full_scan(
        &self,
        request: FullScanRequest,
        stop_gap: u64,
        parallel_requests: u64,
    ) -> Result<JsValue, JsValue> {
        let request = Arc::new(request);
        let request: BdkFullScanRequest<KeychainKind> = request
            .0
            .lock()
            .unwrap()
            .take()
            .ok_or(EsploraError::RequestAlreadyConsumed)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

        let result: BdkFullScanResult<KeychainKind> =
            self.0
                .full_scan(request, stop_gap as usize, parallel_requests as usize)
                .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

        let update = BdkUpdate {
            last_active_indices: result.last_active_indices,
            tx_update: result.tx_update,
            chain: result.chain_update,
        };

        serde_wasm_bindgen::to_value(&Update(update))
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)));

        Ok(JsValue::from(Update(update)))
    }

    // #[wasm_bindgen]
    // pub fn sync(
    //     &self,
    //     request: Arc<SyncRequest>,
    //     parallel_requests: u64,
    // ) -> Result<Arc<Update>, EsploraError> {
    //     // using option and take is not ideal but the only way to take full ownership of the request
    //     let request: BdkSyncRequest<(KeychainKind, u32)> = request
    //         .0
    //         .lock()
    //         .unwrap()
    //         .take()
    //         .ok_or(EsploraError::RequestAlreadyConsumed)?;

    //     let result: BdkSyncResult = self.0.sync(request, parallel_requests as usize)?;

    //     let update = BdkUpdate {
    //         last_active_indices: BTreeMap::default(),
    //         tx_update: result.tx_update,
    //         chain: result.chain_update,
    //     };

    //     Ok(Arc::new(Update(update)))
    // }

    // #[wasm_bindgen]
    // pub fn broadcast(&self, transaction: &Transaction) -> Result<(), EsploraError> {
    //     let bdk_transaction: BdkTransaction = transaction.into();
    //     self.0
    //         .broadcast(&bdk_transaction)
    //         .map_err(EsploraError::from)
    // }

    // #[wasm_bindgen]
    // pub fn get_tx(&self, txid: String) -> Result<Option<Arc<Transaction>>, EsploraError> {
    //     let txid = Txid::from_str(&txid)?;
    //     let tx_opt = self.0.get_tx(&txid)?;
    //     Ok(tx_opt.map(|inner| Arc::new(Transaction::from(inner))))
    // }

    // #[wasm_bindgen]
    // pub fn get_height(&self) -> Result<u32, EsploraError> {
    //     self.0.get_height().map_err(EsploraError::from)
    // }
}