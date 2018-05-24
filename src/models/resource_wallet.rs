use models::resource_address::ResourceAddress;
use jsonapi::model::*;
use models::wallets::Wallets;
use std;

pub trait ResourceWallet: std::marker::Sized + JsonApiModel + Clone + std::fmt::Debug {
    type A: ResourceAddress;
    fn raw_id(&self) -> Option<u64>;

    fn id(&self) -> u64 {
        self.raw_id().unwrap_or(0)
    }

    fn set_id(self, new_id: u64) -> Self;

    fn set_auto_id_if_needed(self, last_id: u64) -> Self {
      if self.raw_id().is_none() {
        self.set_id(last_id + 1 )
      }else{
        self
      }
    }

    fn merge(self, newer: Self) -> Self;

    fn add_address(&mut self, address: Self::A);

    fn get_addresses(&self) -> Vec<Self::A>;

    fn default_query() -> Query {
      Query::from_params(&format!(
          "include=[]&fields[{}]={}",
          Self::jsonapi_type(),
          Self::default_fields()
      ))
    }

    fn default_fields() -> &'static str;

    fn collection_from_wallets<'a>(wallets: &'a mut Wallets) -> &'a mut Vec<Self>;

    fn remove_address(&mut self, address: Self::A) -> Result<bool, String>;
}
