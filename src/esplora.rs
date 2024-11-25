use std::{cell::RefCell, collections::BTreeSet, io::Write, rc::Rc};

use wasm_bindgen::prelude::*;

use crate::bitcoin::Transaction;
use crate::error::EsploraError;
use crate::types::{FullScanRequestWrapper, SyncRequest, UpdateWrapper};

use bdk_esplora::esplora_client::{AsyncClient, Builder};
use bdk_esplora::EsploraAsyncExt;
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

// #[wasm_bindgen]
// pub struct EsploraClient(AsyncClient);
// impl Clone for EsploraClient {
//   fn clone(&self) -> Self {
//       EsploraClient(self.0.clone())
//   }
// }
// impl From<ExploraClient> for BdkFullScanRequest<KeychainKind> {
//   fn from(request: FullScanRequest) -> Self {
//       request.0.lock().unwrap().take().unwrap()
//   }
// }

#[wasm_bindgen]
pub struct EsploraClientWrapper {
  client: Rc<RefCell<AsyncClient>>
}

#[wasm_bindgen]
impl EsploraClientWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new(url: String) -> Self {
        // let client = Builder::new(url.as_str()).build_async().expect("Failed to build Esplora client");
        let client = Builder::new(&url)
        .max_retries(6)
        .build_async()
        .map_err(|e| format!("{:?}", e)).unwrap();

        EsploraClientWrapper {
          client: Rc::new(RefCell::new(client))
        }
    }

    // pub fn get(&self) -> AsyncClient {
    //   self.client.borrow().clone()
    // }

    #[wasm_bindgen]
    pub async fn full_scan(
        &self,
        request: FullScanRequestWrapper,
        parallel_requests: usize,
        stop_gap: usize,
    ) -> Result<UpdateWrapper, String> {
      let request: BdkFullScanRequest<KeychainKind> = request.get().into();

      let client = Rc::clone(&self.client);


      let result = client
        .borrow()
        .full_scan(
            request,
            stop_gap as usize,
            parallel_requests as usize
        )
        .await
        .map_err(|e| format!("{:?}", e))?;


      let update = BdkUpdate {
        last_active_indices: result.last_active_indices,
        tx_update: result.tx_update,
        chain: result.chain_update,
      };
    
            // Ok(UpdateWrapper::new(Update(update)))

            
        Ok(UpdateWrapper::new(update))
    }
}
