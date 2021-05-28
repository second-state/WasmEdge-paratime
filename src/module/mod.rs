mod auth;
mod block;
mod methods;
mod migrate;

use oasis_runtime_sdk as sdk;
use serde::{Deserialize, Serialize};

const MODULE_NAME: &str = "WasmEdge-eth";

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

impl sdk::module::MessageHookRegistrationHandler for Module {}
