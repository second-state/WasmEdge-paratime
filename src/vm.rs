use ethereum_types::{Address, U256};
use serde::Deserialize;
#[cfg(test)]
use serde::Serialize;

#[derive(Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, Default, Serialize))]
pub(crate) struct Transaction {
    pub(crate) gas_price: U256,
    pub(crate) gas: U256,
    pub(crate) action: Action,
    pub(crate) value: U256,
    pub(crate) data: bytes::Bytes,
}

#[derive(Deserialize)]
#[cfg_attr(test, derive(Clone, Debug, Serialize))]
pub(crate) enum Action {
    Create,
    Create2,
    Call(Address),
}

impl Default for Action {
    fn default() -> Self {
        Self::Create
    }
}
