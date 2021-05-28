use super::Module;

use oasis_runtime_sdk::{self as sdk, DispatchContext};

impl sdk::module::BlockHandler for Module {
    fn begin_block(_ctx: &mut DispatchContext<'_>) {
        unimplemented!("attach EVMC `HostContext` to `ctx` or w/e")
    }
}
