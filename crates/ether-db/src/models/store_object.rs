use toasty::{Deferred, Model};

use crate::utils::{StoreHash, StoreID};

#[derive(Debug, Model)]
pub struct StoreObject {
  #[key]
  #[auto]
  id : StoreID,

  hash : StoreHash,

  #[index]
  comes_from_id : Option<StoreID>,
  #[belongs_to(key = comes_from_id)]
  comes_from : Deferred<Option<StoreObject>>,

  #[has_many(pair = comes_from)]
  children : Deferred<Vec<StoreObject>>,
}
