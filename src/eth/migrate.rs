use super::Module;

use oasis_runtime_sdk::{self as sdk, modules::core::types::Metadata, DispatchContext};

impl sdk::module::MigrationHandler for Module {
    type Genesis = ();

    fn init_or_migrate(
        _ctx: &mut DispatchContext<'_>,
        _meta: &mut Metadata,
        _genesis: &Self::Genesis,
    ) -> bool {
        true
    }
}
