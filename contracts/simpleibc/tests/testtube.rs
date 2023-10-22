use cosmwasm_std::Coin;
use osmosis_test_tube::{Account, Module, OsmosisTestApp, Wasm};

pub fn test_init() {
    let app = OsmosisTestApp::new();
    
    // create new account with initial funds
    let accs = app
    .init_accounts(
        &[
            Coin::new(1_000_000_000_000, "uatom"),
            Coin::new(1_000_000_000_000, "uosmo"),
        ],
        2,
    )
    .unwrap();
    let admin = &accs[0];
    let new_admin = &accs[1];

    let wasm = Wasm::new(&app);
    let wasm_byte_code = std::fs::read("../../target/wasm32-unknown-unknown/release/simpleibc.wasm").unwrap();
    let code_id = wasm.store_code(&wasm_byte_code, None,admin ).unwrap();
}


pub fn main() {
    test_init();
}