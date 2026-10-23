use ether_common::Res;
use toasty::{Db, Deferred, Model};

use crate::utils::{IntoPrimitive, StoreHash, StoreID};

#[derive(Debug, Model)]
pub struct StoreObject {
  #[key]
  #[auto]
  pub id : StoreID,

  #[index]
  pub hash : StoreHash,

  #[index]
  pub comes_from_id : Option<StoreID>,
  #[belongs_to(key = comes_from_id)]
  pub comes_from : Deferred<Option<StoreObject>>,

  #[has_many(pair = comes_from)]
  pub children : Deferred<Vec<StoreObject>>,
}

impl IntoPrimitive for StoreObject {
  type P = ether_common::StoreObject;
  async fn into_primitive(self, db : &mut Db) -> Res<Self::P> {
    // let obj = cmm::StoreObject { id : db_obj.id.into(),
    //                              hash : db_obj.hash.into(),
    //                              comes_from : None,
    //                              children : Vec::new() };
    let children = self.children()
                       .exec(db)
                       .await?
                       .iter()
                       .map(|x| x.id.clone().into())
                       .collect();
    let comes_from = self.comes_from_id.clone().map(|x| x.into());
    Ok(Self::P { id : self.id.clone().into(),
                 hash : self.hash.clone().into(),
                 comes_from,
                 children })
  }
}
