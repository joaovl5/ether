use std::collections::HashMap;

use super::hash;
use super::url::URL;

mod store_impl;

pub use store_impl::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct StoreId(String);

#[derive(Debug, PartialEq, Eq)]
pub struct StoreObject {
  id : StoreId,
  comes_from : Option<StoreId>,
  references : Vec<StoreId>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Drv {
  name : String,
  src_inputs : Vec<StoreId>,
  src_drvs : HashMap<StoreId, Vec<DrvOutputName>>,
  outputs : HashMap<DrvOutputName, DrvOutput>,
  builder : DrvBuilder,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DrvOutputName(String);

#[derive(Debug, PartialEq, Eq)]
pub struct DrvOutput {
  addressment : DrvAddressment,

  hash_algo : hash::HashAlgo,
  hash : String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DrvBuilder {
  FetchURL {
    url : URL,
  },
  Process {
    store_id : StoreId,
    builder_args : Vec<String>,
    builder_env : HashMap<String, String>,
  },
}

#[derive(Debug, PartialEq, Eq)]
pub enum DrvAddressment {
  Input,
  Output,
  DynamicOutput,
  /// like `DynamicOutput` but runs w.o. sandbox
  Impure,
}

#[derive(Debug, PartialEq, Eq)]
pub enum System {
  X86_64,
  Arm64,
}
