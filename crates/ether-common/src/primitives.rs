use derive_more::From;

#[derive(Debug, PartialEq, Eq, Hash, From)]
pub struct StoreID(String);

#[derive(Debug, PartialEq, Eq, Hash, From)]
pub struct StoreHash(String);

#[derive(Debug, PartialEq, Eq)]
pub struct StoreObject {
  id : StoreID,
  hash : StoreHash,
  comes_from : Option<StoreID>,
  children : Vec<StoreID>,
}
