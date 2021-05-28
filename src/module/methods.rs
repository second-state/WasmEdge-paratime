use super::Module;

use oasis_runtime_sdk as sdk;

use sdk::{
    context::{DispatchContext, TxContext},
    core::common::cbor,
    error::RuntimeError,
    module::{CallableMethodInfo, MethodRegistrationHandler, MethodRegistry, QueryMethodInfo},
    types::transaction::CallResult,
};

impl MethodRegistrationHandler for Module {
    fn register_methods(methods: &mut MethodRegistry) {
        methods.register_callable(CallableMethodInfo {
            name: "vm.Transact",
            handler: Self::_call_contract_handler,
        });
        methods.register_query(QueryMethodInfo {
            name: "vm.Call",
            handler: Self::_query_contract_handler,
        });
        methods.register_query(QueryMethodInfo {
            name: "accounts.Balance",
            handler: Self::_query_balance_handler,
        });
        methods.register_query(QueryMethodInfo {
            name: "accounts.Nonce",
            handler: Self::_query_nonce_handler,
        });
        methods.register_query(QueryMethodInfo {
            name: "accounts.Code",
            handler: Self::_query_code_handler,
        });
        methods.register_query(QueryMethodInfo {
            name: "accounts.Storage",
            handler: Self::_query_storage_handler,
        });
    }
}

impl Module {
    pub(crate) fn _call_contract_handler(
        _mi: &CallableMethodInfo,
        _ctx: &mut TxContext<'_, '_>,
        _body: cbor::Value,
    ) -> CallResult {
        unimplemented!("decode `body` and do actual work")
    }

    pub(crate) fn _query_contract_handler(
        _mi: &QueryMethodInfo,
        _ctx: &mut DispatchContext<'_>,
        _body: cbor::Value,
    ) -> Result<cbor::Value, RuntimeError> {
        unimplemented!("decode `body` and do actual work")
    }

    pub(crate) fn _query_balance_handler(
        _mi: &QueryMethodInfo,
        _ctx: &mut DispatchContext<'_>,
        _body: cbor::Value,
    ) -> Result<cbor::Value, RuntimeError> {
        unimplemented!("decode `body` and do actual work")
    }

    pub(crate) fn _query_nonce_handler(
        _mi: &QueryMethodInfo,
        _ctx: &mut DispatchContext<'_>,
        _body: cbor::Value,
    ) -> Result<cbor::Value, RuntimeError> {
        unimplemented!("decode `body` and do actual work")
    }

    pub(crate) fn _query_code_handler(
        _mi: &QueryMethodInfo,
        _ctx: &mut DispatchContext<'_>,
        _body: cbor::Value,
    ) -> Result<cbor::Value, RuntimeError> {
        unimplemented!("decode `body` and do actual work")
    }

    pub(crate) fn _query_storage_handler(
        _mi: &QueryMethodInfo,
        _ctx: &mut DispatchContext<'_>,
        _body: cbor::Value,
    ) -> Result<cbor::Value, RuntimeError> {
        unimplemented!("decode `body` and do actual work")
    }
}
