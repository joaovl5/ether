use derive_more::{From, Into};

#[derive(Debug, PartialEq, Eq, Hash, From, Into)]
pub struct StoreID(String);

#[derive(Debug, PartialEq, Eq, Hash, From, Into)]
pub struct StoreHash(String);

#[derive(Debug, PartialEq, Eq)]
pub struct StoreObject {
  pub id : StoreID,
  pub hash : StoreHash,
  pub comes_from : Option<StoreID>,
  pub children : Vec<StoreID>,
}
