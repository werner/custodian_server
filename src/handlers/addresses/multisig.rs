use handlers::addresses::base::AddressHandler;
use handlers::handler::GetTransactionParams;
use handlers::handler::JsonResult;
use models::multisig_wallet::HdAddress;
use models::multisig_wallet::MultisigWallet;
use server_state::ServerState;

#[get("/multisig_wallets/<id>/relationships/addresses", format = "application/json")]
pub fn index(state: &ServerState, id: u64) -> JsonResult {
    MultisigWallet::address_index(state, id)
}

#[post(
    "/multisig_wallets/<id>/relationships/addresses",
    format = "application/json",
    data = "<address>"
)]
pub fn create(state: &ServerState, id: u64, address: HdAddress) -> JsonResult {
    MultisigWallet::address_create(state, id, address)
}

#[delete(
    "/multisig_wallets/<id>/relationships/addresses",
    format = "application/json",
    data = "<address>"
)]
pub fn destroy(state: &ServerState, id: u64, address: HdAddress) -> JsonResult {
    MultisigWallet::address_destroy(state, id, address)
}

#[get(
    "/multisig_wallets/relationships/addresses/<address>/balance?<params>", format = "application/json"
)]
pub fn balance(state: &ServerState, address: String, params: GetTransactionParams) -> JsonResult {
    MultisigWallet::balance(&state.executor, address, params.limit, params.since)
}

#[get("/multisig_wallets/relationships/addresses/<address>/get_utxos?<params>", format = "application/json")]
pub fn get_utxos(state: &ServerState, address: String, params: GetTransactionParams) -> JsonResult {
    MultisigWallet::get_utxos(&state.executor, address, params.limit, params.since)
}