use oasis_runtime_sdk::{
    self as sdk,
    context::DispatchContext,
    module::{MethodRegistrationHandler, MethodRegistry, MigrationHandler},
    modules::core::{types::Metadata, Error as CoreError},
    types::transaction::Transaction,
};
use serde::{Deserialize, Serialize};

const MODULE_NAME: &str = "WasmEdge-account";

pub struct Module;

impl sdk::Module for Module {
    const NAME: &'static str = MODULE_NAME;
    type Error = Error;
    type Event = Event;
    type Parameters = ();
}

#[derive(Debug, thiserror::Error, sdk::Error)]
pub enum Error {
    #[sdk_error(code = 1)]
    #[error("unknow error")]
    UnknowError,
}

#[derive(Debug, Serialize, Deserialize, sdk::Event)]
pub enum Event {
    // TODO
}

impl MigrationHandler for Module {
    type Genesis = ();
    fn init_or_migrate(
        _ctx: &mut DispatchContext<'_>,
        _meta: &mut Metadata,
        _genesis: &Self::Genesis,
    ) -> bool {
        true
    }
}

impl sdk::module::AuthHandler for Module {
    fn authenticate_tx(_ctx: &mut DispatchContext<'_>, _tx: &Transaction) -> Result<(), CoreError> {
        unimplemented!("put base gas and nonce checks here")
    }
}
impl sdk::module::BlockHandler for Module {
    fn begin_block(_ctx: &mut DispatchContext<'_>) {
        unimplemented!("attach EVMC `HostContext` to `ctx` or w/e")
    }
}
impl sdk::module::MessageHookRegistrationHandler for Module {}
impl MethodRegistrationHandler for Module {
    fn register_methods(_methods: &mut MethodRegistry) {}
}

impl Module {}
