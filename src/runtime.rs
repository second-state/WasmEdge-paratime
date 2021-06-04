//! WasmEdge Runtime
//! WasmEdge is a runtime excute the Wasm and eWasm
use oasis_runtime_sdk as sdk;
use once_cell::unsync::Lazy;

pub struct Runtime;

impl sdk::Runtime for Runtime {
    const VERSION: sdk::core::common::version::Version = sdk::version_from_cargo!();

    type Modules = (super::eth::Module, super::account::Module);

    fn genesis_state() -> <Self::Modules as sdk::module::MigrationHandler>::Genesis {
        let eth_genesis = Lazy::new(|| return ());
        let account_genesis = Lazy::new(|| return ());
        (*eth_genesis, *account_genesis)
    }
}
