// use `cw_storage_plus` to create ORM-like interface to storage
// see: https://crates.io/crates/cw-storage-plus
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub tx_count: u64,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub denom: String,
    pub channel: String,
    pub min_amount: u64,
    pub dest_address: String,
}

pub const STATE: Item<State> = Item::new("state");
pub const CONFIG: Item<Config> = Item::new("config");
