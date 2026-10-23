use toasty::{Deferred, Model};

use crate::utils::{StoreHash, StoreID};

#[derive(Debug, Model)]
pub struct StoreObject {
  #[key]
  #[auto]
  pub id : StoreID,

  pub hash : StoreHash,

  #[index]
  pub comes_from_id : Option<StoreID>,
  #[belongs_to(key = comes_from_id)]
  pub comes_from : Deferred<Option<StoreObject>>,

  #[has_many(pair = comes_from)]
  pub children : Deferred<Vec<StoreObject>>,
}

type P = ether_common::StoreObject;

impl P {
  fn from(value : StoreObject) -> Self {
    // let obj = cmm::StoreObject { id : db_obj.id.into(),
    //                              hash : db_obj.hash.into(),
    //                              comes_from : None,
    //                              children : Vec::new() };
    P { id : value.id.into(),
        hash : value.hash.into(),
        comes_from : value.comes_from_id.map(|x| x.into()),
        children : value.children }
  }
}
