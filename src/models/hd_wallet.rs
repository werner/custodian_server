use std::io::Read;

use bitprim::executor::Executor;
use jsonapi::model::*;

use models::resource_address::ResourceAddress;
use models::resource_wallet::ResourceWallet;
use models::wallet::Wallet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HdWallet {
    pub id: Option<String>,
    pub version: String,
    #[serde(default)]
    pub addresses: Vec<HdAddress>,
    pub xpub: String,
}

jsonapi_model!(HdWallet; "hd_wallet");

pub struct HdUtxo {
    pub prev_hash: String,
    pub prev_index: u64,
    pub address: HdAddress,
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HdAddress {
    pub id: Option<String>,
    pub address: String,
    pub path: Vec<u64>,
}

jsonapi_model!(HdAddress; "hd_address");

impl Wallet for HdWallet {
    type Utxo = HdUtxo;

    fn get_utxos(&self, _exec: &Executor) -> Vec<Self::Utxo> {
        vec![]
        /*
            prev_hash: "abc".to_string(),
            prev_index: 1,
            address: HdAddress {
                id: Some("1".to_string()),
                address: "abc".to_string(),
                path: vec![0, 1, 0],
            },
            amount: 100000000,
        }]
        */
    }
}

from_data_wallet!(HdWallet);
from_data_wallet!(HdAddress);

impl ResourceWallet<HdAddress> for HdWallet {
    fn raw_id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    fn merge(self, newer: Self) -> Self {
      let addresses = self.addresses;
      HdWallet{ addresses, ..newer }
    }

    /*
    fn add_address(&mut self, address: HdAddress) -> Result<bool, String> {
        match self.addresses.clone().into_iter().find(|in_address| in_address.id == address.id) {
            Some(_) => Err(format!("Duplicate address {:?}", address)),
            None    => { self.addresses.push(address); Ok(true) }
        }
    }

    fn get_addresses(&self) -> Vec<HdAddress> {
        self.addresses.clone()
    }

    fn remove_address(&mut self, address: HdAddress) -> Result<bool, String> {
        match self.addresses.clone().into_iter().position(|in_address| in_address.id == address.id) {
            Some(index) => { self.addresses.remove(index); Ok(true) },
            None        => Err(format!("Address {:?} does not exists", address))
        }
    }
    */
}

impl ResourceAddress for HdAddress {}
