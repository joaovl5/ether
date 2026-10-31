use std::error::Error;

use ether_common::{EtherError, MetadataBackend, Res, ResOpt, StoreHash,
                   StoreObject, StoreRefType};
use toasty::{Db, models};

use crate::models::DbStoreObject;
use crate::utils::{DbStoreHash, FromPrimitive, IntoPrimitive};

// TODO: CHANGE LATER
const ETHER_DB_PATH : &str = "sqlite:/ether/state.db";

pub async fn get_db() -> toasty::Result<Db> {
  Db::builder().models(models!(crate::*))
               .connect(ETHER_DB_PATH)
               .await
}

#[derive(Debug)]
pub struct EtherDatabase {
  db : Db,
}

impl EtherDatabase {
  pub async fn new() -> Result<Self, EtherError> {
    let db = get_db().await?;
    Ok(Self { db })
  }

  async fn resopt_map<E, P, M>(&mut self, x : Result<Option<M>, E>) -> ResOpt<P>
    where E : Error + 'static,
          M : IntoPrimitive<P = P>,
  {
    let x = x?;
    Ok(match x {
      | None => None,
      | Some(x_i) => Some(x_i.into_primitive(&mut self.db).await?),
    })
  }
}

impl MetadataBackend for EtherDatabase {
  type StoreObjectT = DbStoreObject;

  async fn mk(&mut self, prim : StoreObject) -> Res<DbStoreObject> {
    DbStoreObject::from_primitive(&mut self.db, prim).await
  }

  async fn get_by_hash(&mut self, hash : StoreHash) -> ResOpt<StoreObject> {
    let hash_conv : DbStoreHash = hash.into();
    let res = DbStoreObject::filter_by_hash(hash_conv).first()
                                                      .exec(&mut self.db)
                                                      .await;
    self.resopt_map(res).await
  }

  async fn get_lhs_refs(&mut self,
                        _hash : StoreHash)
                        -> Vec<(StoreHash, StoreRefType)> {
    todo!()
  }

  async fn get_rhs_refs(&mut self,
                        _hash : StoreHash)
                        -> Vec<(StoreHash, StoreRefType)> {
    todo!()
  }
}
