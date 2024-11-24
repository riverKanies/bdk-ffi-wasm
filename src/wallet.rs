use wasm_bindgen::prelude::*;
use std::{cell::RefCell, collections::BTreeSet, io::Write, rc::Rc};


use crate::bitcoin::{Psbt, Transaction};
use crate::descriptor::Descriptor;
use crate::error::{
    CalculateFeeError, CannotConnectError, CreateWithPersistError, LoadWithPersistError,
    SignerError, SqliteError, TxidParseError,
};
use crate::types::{
    AddressInfo, Balance, CanonicalTx, FullScanRequest, FullScanRequestWrapper, KeychainAndIndex, LocalOutput,
    SentAndReceivedValues, SyncRequestBuilder, Update, UpdateWrapper,
};

use bitcoin_ffi::{Amount, FeeRate, OutPoint, Script};

use bdk_wallet::bitcoin::{Network, Txid};
// use rusqlite::Connection as BdkConnection;
use bdk_wallet::{KeychainKind, PersistedWallet, SignOptions, Wallet as BdkWallet};

use std::borrow::BorrowMut;
use std::str::FromStr;
use std::sync::{Arc, Mutex, MutexGuard};

#[wasm_bindgen]
pub struct Wallet {
    inner_mutex: Mutex<BdkWallet>,
}

#[wasm_bindgen]
impl Wallet {
    #[wasm_bindgen(constructor)]
    pub fn new(
        descriptor: String,
        change_descriptor: String,
        network: String
    ) -> Result<Self, String> {

        let network = match network.as_str() {
            "mainnet" => Network::Bitcoin,
            "testnet" => Network::Testnet,
            "testnet4" => Network::Testnet4,
            "signet" => Network::Signet,
            "regtest" => Network::Regtest,
            _ => return Err("Invalid network".into()),
        };

        let wallet: BdkWallet =
            BdkWallet::create(descriptor, change_descriptor)
                .network(network)
                .create_wallet_no_persist()
                .expect("create wallet");

        Ok(Wallet {
            inner_mutex: Mutex::new(wallet),
        })
    }

    // pub fn load(
    //     descriptor: Arc<Descriptor>,
    //     change_descriptor: Arc<Descriptor>,
    //     connection: Arc<Connection>,
    // ) -> Result<Wallet, LoadWithPersistError> {
    //     let descriptor = descriptor.to_string_with_secret();
    //     let change_descriptor = change_descriptor.to_string_with_secret();
    //     let mut binding = connection.get_store();
    //     let db: &mut BdkConnection = binding.borrow_mut();

    //     let wallet: PersistedWallet<BdkConnection> = BdkWallet::load()
    //         .descriptor(KeychainKind::External, Some(descriptor))
    //         .descriptor(KeychainKind::Internal, Some(change_descriptor))
    //         .extract_keys()
    //         .load_wallet(db)?
    //         .ok_or(LoadWithPersistError::CouldNotLoad)?;

    //     Ok(Wallet {
    //         inner_mutex: Mutex::new(wallet),
    //     })
    // }

    pub(crate) fn get_wallet(&self) -> MutexGuard<BdkWallet> {
        self.inner_mutex.lock().expect("wallet")
    }

    // #[wasm_bindgen]
    // pub fn derivation_of_spk(&self, spk: Arc<Script>) -> Option<KeychainAndIndex> {
    //     self.get_wallet()
    //         .derivation_of_spk(spk.0.clone())
    //         .map(|(k, i)| KeychainAndIndex {
    //             keychain: k,
    //             index: i,
    //         })
    // }

    // #[wasm_bindgen]
    // pub fn cancel_tx(&self, tx: &Transaction) {
    //     self.get_wallet().cancel_tx(&tx.into())
    // }

    // #[wasm_bindgen]
    // pub fn get_utxo(&self, op: OutPoint) -> Option<LocalOutput> {
    //     self.get_wallet()
    //         .get_utxo(op)
    //         .map(|local_output| local_output.into())
    // }

    // #[wasm_bindgen]
    // pub fn reveal_next_address(&self, keychain: KeychainKind) -> AddressInfo {
    //     self.get_wallet().reveal_next_address(keychain).into()
    // }

    #[wasm_bindgen]
    pub fn peek_address(&self, keychain_kind: String, index: u32) -> String {
        let keychain = match keychain_kind.as_str() {
            "external" => KeychainKind::External,
            "internal" => KeychainKind::Internal,
            _ => return "Invalid keychain kind".into(),
        };

        self.get_wallet().peek_address(keychain, index).to_string()
    }

    // #[wasm_bindgen]
    // pub fn next_derivation_index(&self, keychain: KeychainKind) -> u32 {
    //     self.get_wallet().next_derivation_index(keychain)
    // }

    // #[wasm_bindgen]
    // pub fn next_unused_address(&self, keychain: KeychainKind) -> AddressInfo {
    //     self.get_wallet().next_unused_address(keychain).into()
    // }

    // #[wasm_bindgen]
    // pub fn mark_used(&self, keychain: KeychainKind, index: u32) -> bool {
    //     self.get_wallet().mark_used(keychain, index)
    // }

    // #[wasm_bindgen]
    // pub fn reveal_addresses_to(&self, keychain: KeychainKind, index: u32) -> Vec<AddressInfo> {
    //     self.get_wallet()
    //         .reveal_addresses_to(keychain, index)
    //         .map(|address_info| address_info.into())
    //         .collect()
    // }

    // #[wasm_bindgen]
    // pub fn list_unused_addresses(&self, keychain: KeychainKind) -> Vec<AddressInfo> {
    //     self.get_wallet()
    //         .list_unused_addresses(keychain)
    //         .map(|address_info| address_info.into())
    //         .collect()
    // }

    // #[wasm_bindgen]
    // pub fn apply_update(&self, update: Arc<Update>) -> Result<(), CannotConnectError> {
    //     self.get_wallet()
    //         .apply_update(update.0.clone())
    //         .map_err(CannotConnectError::from)
    // }

    #[wasm_bindgen]
    pub fn apply_update_at(&self, update: UpdateWrapper, timestamp: u64) -> (Result<(), String>) {
        let update = update.get();
        Ok(self.get_wallet()
            .apply_update_at(update.0.clone(), Some(timestamp))
            .map_err(|e| format!("{:?}", e))?)
    }

    pub(crate) fn derivation_index(&self, keychain: KeychainKind) -> Option<u32> {
        self.get_wallet().derivation_index(keychain)
    }

    // #[wasm_bindgen]
    // pub fn descriptor_checksum(&self, keychain: KeychainKind) -> String {
    //     self.get_wallet().descriptor_checksum(keychain)
    // }

    // #[wasm_bindgen]
    // pub fn network(&self) -> Network {
    //     self.get_wallet().network()
    // }

    #[wasm_bindgen]
    pub fn balance(&self) -> u64 {
        let bdk_balance = self.get_wallet().balance();
        // Balance::from(bdk_balance)
        bdk_balance.total().to_sat()
    }

    // #[wasm_bindgen]
    // pub fn is_mine(&self, script: Arc<Script>) -> bool {
    //     self.get_wallet().is_mine(script.0.clone())
    // }

    pub(crate) fn sign(
        &self,
        psbt: Arc<Psbt>,
        // sign_options: Option<SignOptions>,
    ) -> Result<bool, SignerError> {
        let mut psbt = psbt.0.lock().unwrap();
        self.get_wallet()
            .sign(&mut psbt, SignOptions::default())
            .map_err(SignerError::from)
    }

    // #[wasm_bindgen]
    // pub fn finalize_psbt(&self, psbt: Arc<Psbt>) -> Result<bool, SignerError> {
    //     let mut psbt = psbt.0.lock().unwrap();
    //     self.get_wallet()
    //         .finalize_psbt(&mut psbt, SignOptions::default())
    //         .map_err(SignerError::from)
    // }

    // #[wasm_bindgen]
    // pub fn sent_and_received(&self, tx: &Transaction) -> SentAndReceivedValues {
    //     let (sent, received) = self.get_wallet().sent_and_received(&tx.into());
    //     SentAndReceivedValues {
    //         sent: Arc::new(sent.into()),
    //         received: Arc::new(received.into()),
    //     }
    // }

    // #[wasm_bindgen]
    // pub fn transactions(&self) -> Vec<CanonicalTx> {
    //     self.get_wallet()
    //         .transactions()
    //         .map(|tx| tx.into())
    //         .collect()
    // }

    // #[wasm_bindgen]
    // pub fn get_tx(&self, txid: String) -> Result<Option<CanonicalTx>, TxidParseError> {
    //     let txid =
    //         Txid::from_str(txid.as_str()).map_err(|_| TxidParseError::InvalidTxid { txid })?;
    //     Ok(self.get_wallet().get_tx(txid).map(|tx| tx.into()))
    // }

    // #[wasm_bindgen]
    // pub fn calculate_fee(&self, tx: &Transaction) -> Result<Arc<Amount>, CalculateFeeError> {
    //     self.get_wallet()
    //         .calculate_fee(&tx.into())
    //         .map(Amount::from)
    //         .map(Arc::new)
    //         .map_err(|e| e.into())
    // }

    // #[wasm_bindgen]
    // pub fn calculate_fee_rate(&self, tx: &Transaction) -> Result<Arc<FeeRate>, CalculateFeeError> {
    //     self.get_wallet()
    //         .calculate_fee_rate(&tx.into())
    //         .map(|bdk_fee_rate| Arc::new(FeeRate(bdk_fee_rate)))
    //         .map_err(|e| e.into())
    // }

    // #[wasm_bindgen]
    // pub fn list_unspent(&self) -> Vec<LocalOutput> {
    //     self.get_wallet().list_unspent().map(|o| o.into()).collect()
    // }

    // #[wasm_bindgen]
    // pub fn list_output(&self) -> Vec<LocalOutput> {
    //     self.get_wallet().list_output().map(|o| o.into()).collect()
    // }

    #[wasm_bindgen]
    pub fn start_full_scan(&self) -> FullScanRequestWrapper {
        let builder = self.get_wallet().start_full_scan();
        FullScanRequestWrapper::new(FullScanRequest(Arc::new(Mutex::new(Some(builder.build())))))
    }

    // #[wasm_bindgen]
    // pub fn start_sync_with_revealed_spks(&self) -> Arc<SyncRequestBuilder> {
    //     let builder = self.get_wallet().start_sync_with_revealed_spks();
    //     Arc::new(SyncRequestBuilder(Mutex::new(Some(builder))))
    // }



    // pub fn persist(&self, connection: Connection) -> Result<bool, FfiGenericError> {
    // pub fn persist(&self, connection: Arc<Connection>) -> Result<bool, SqliteError> {
    //     let mut binding = connection.get_store();
    //     let db: &mut BdkConnection = binding.borrow_mut();
    //     self.get_wallet()
    //         .persist(db)
    //         .map_err(|e| SqliteError::Sqlite {
    //             rusqlite_error: e.to_string(),
    //         })
    // }
}
