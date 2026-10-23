mod store_impl;

use std::collections::HashMap;

pub use self::store_impl::*;
use super::hash;
use super::url::URL;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct StoreId(String);

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
