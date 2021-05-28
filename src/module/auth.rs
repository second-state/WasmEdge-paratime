use super::Module;

use oasis_runtime_sdk::{
    self as sdk, modules::core::Error as CoreError, types::transaction::Transaction,
    DispatchContext,
};

impl sdk::module::AuthHandler for Module {
    fn authenticate_tx(_ctx: &mut DispatchContext<'_>, _tx: &Transaction) -> Result<(), CoreError> {
        unimplemented!("put base gas and nonce checks here")
    }
}
