use ether_common::Res;
use toasty::{Db, Deferred, Model};

use crate::utils::{FromPrimitive, IntoPrimitive, StoreHash, StoreID};

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
    let comes_from = self.comes_from().exec(db).await?.map(|x| x.hash.into());
    Ok(Self::P { hash : self.hash.clone().into(),
                 comes_from })
  }
}

impl FromPrimitive for StoreObject {
  type P = ether_common::StoreObject;
  async fn from_primitive(db : &mut Db, prim : Self::P) -> Res<Self> {
    let obj = Self::create();
    let obj_hash : StoreHash = prim.hash.into();
    let obj_comes_from_id : Option<StoreID> = match prim.comes_from {
      | None => None,
      | Some(x) => {
        let hash : StoreHash = x.into();
        Some(Self::get_by_hash(db, hash).await?.id)
      },
    };

    let obj = obj.hash(obj_hash)
                 .comes_from_id(obj_comes_from_id)
                 .exec(db)
                 .await?;
    Ok(obj)
  }
}
