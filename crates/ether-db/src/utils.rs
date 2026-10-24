use derive_more::From;
use ether_common::Res;
use toasty::Db;

pub(crate) trait IntoPrimitive {
  type P;
  async fn into_primitive(self, db : &mut Db) -> Res<Self::P>;
}

pub(crate) trait FromPrimitive: Sized {
  type P;
  async fn from_primitive(db : &mut Db, prim : Self::P) -> Res<Self>;
}

pub(crate) type UUID = uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, From, toasty::Embed)]
pub struct StoreID(UUID);

#[derive(Debug, Clone, PartialEq, Eq, Hash, From, toasty::Embed)]
pub struct StoreHash(String);

impl From<StoreHash> for ether_common::StoreHash {
  fn from(value : StoreHash) -> ether_common::StoreHash {
    value.0.into()
  }
}

impl From<ether_common::StoreHash> for StoreHash {
  fn from(value : ether_common::StoreHash) -> Self {
    let inner_val : String = value.into();
    inner_val.into()
  }
}
