use ether_common::Res;
use toasty::{Db, Deferred, Model};

use crate::utils::{FromPrimitive, IntoPrimitive, StoreHash, StoreID,
                   StoreRefType};

#[derive(Debug, Model)]
pub struct StoreObject {
  #[key]
  #[auto]
  pub id : StoreID,

  #[index]
  pub hash : StoreHash,
  // #[has_many(pair = comes_from)]
  // pub children : Deferred<Vec<StoreObject>>,
}

#[derive(Debug, Model)]
#[key(lhs_id, rhs_id)]
pub struct StoreObjectRefs {
  #[index]
  pub lhs_id : StoreID,

  #[index]
  pub rhs_id : StoreID,

  #[belongs_to(key = lhs_id, references = id)]
  pub lhs : Deferred<StoreObject>,

  #[belongs_to(key = rhs_id, references = id)]
  pub rhs : Deferred<StoreObject>,

  #[index]
  pub ref_type : StoreRefType,
}

impl IntoPrimitive for StoreObject {
  type P = ether_common::StoreObject;
  async fn into_primitive(self, _db : &mut Db) -> Res<Self::P> {
    Ok(Self::P { hash : self.hash.clone().into() })
  }
}

impl FromPrimitive for StoreObject {
  type P = ether_common::StoreObject;
  async fn from_primitive(db : &mut Db, prim : Self::P) -> Res<Self> {
    let obj = Self::create();
    let obj_hash : StoreHash = prim.hash.into();

    let obj = obj.hash(obj_hash).exec(db).await?;
    Ok(obj)
  }
}
