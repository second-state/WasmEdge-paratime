//! WasmEdge Runtime
//! WasmEdge is a runtime excute the Wasm and eWasm
use oasis_runtime_sdk as sdk;
use once_cell::unsync::Lazy;
use sdk::types::token::Denomination;
use std::collections::BTreeMap;

pub struct Runtime;

impl sdk::Runtime for Runtime {
    const VERSION: sdk::core::common::version::Version = sdk::version_from_cargo!();

    type Modules = (super::eth::Module, sdk::modules::accounts::Module);

    fn genesis_state() -> <Self::Modules as sdk::module::MigrationHandler>::Genesis {
        let eth_genesis = Lazy::new(|| return ());
        (
            *eth_genesis,
            sdk::modules::accounts::Genesis {
                balances: {
                    let mut balances = BTreeMap::new();
                    // Alice.
                    balances.insert(sdk::testing::keys::alice::address(), {
                        let mut denominations = BTreeMap::new();
                        denominations.insert(Denomination::NATIVE, 1_000_000.into());
                        denominations
                    });
                    // Bob.
                    balances.insert(sdk::testing::keys::bob::address(), {
                        let mut denominations = BTreeMap::new();
                        denominations.insert(Denomination::NATIVE, 1_000_000.into());
                        denominations
                    });
                    balances
                },
                total_supplies: {
                    let mut total_supplies = BTreeMap::new();
                    total_supplies.insert(Denomination::NATIVE, 3_000_000.into());
                    total_supplies
                },
                ..Default::default()
            },
        )
    }
}
