use derive_more::From;
use ether_common::{Res, StoreHash};
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
pub struct DbID(UUID);

// Duplicate type from `ether_common` due to `toasty::Embed` derive call
#[derive(Debug, PartialEq, Eq, toasty::Embed)]
pub enum DbStoreRefType {
  /// `lhs` produces `rhs`.
  Produces,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, From, toasty::Embed)]
pub struct DbStoreHash(String);

impl From<DbStoreHash> for StoreHash {
  fn from(value : DbStoreHash) -> StoreHash {
    value.0.into()
  }
}

impl From<StoreHash> for DbStoreHash {
  fn from(value : StoreHash) -> Self {
    let inner_val : String = value.into();
    inner_val.into()
  }
}
