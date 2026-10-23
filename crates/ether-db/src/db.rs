use std::error::Error;

use ether_common::{self as cmm, EtherError, MetadataBackend, Res, ResOpt};
use toasty::{Db, models};

use crate::models::StoreObject;
use crate::utils::{IntoPrimitive, StoreHash, StoreID};

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

  async fn resopt_map<E : Error + 'static, P, M>(&mut self,
                                                 x : Result<Option<M>, E>)
                                                 -> ResOpt<P>
    where M : IntoPrimitive<P = P>,
  {
    let x = x?;
    Ok(match x {
      | None => None,
      | Some(x_i) => Some(x_i.into_primitive(&mut self.db).await?),
    })
  }
}

impl MetadataBackend for EtherDatabase {
  async fn mk_obj(&mut self, obj : ether_common::StoreObject) -> Res<()> {
    todo!()
  }

  async fn get_obj_by_id(&mut self,
                         id : ether_common::StoreID)
                         -> ResOpt<ether_common::StoreObject> {
    // this means its incompatible w. old dbs or other
    // metadata-backend data without some sort of
    // migration
    let db_id : StoreID = id.try_into()?;
    let res = StoreObject::filter_by_id(db_id).first()
                                              .exec(&mut self.db)
                                              .await;
    self.resopt_map(res).await
  }

  async fn get_obj_by_hash(&mut self,
                           hash : ether_common::StoreHash)
                           -> ResOpt<ether_common::StoreObject> {
    let hash_conv : StoreHash = hash.into();
    let res = StoreObject::filter_by_hash(hash_conv).first()
                                                    .exec(&mut self.db)
                                                    .await;
    self.resopt_map(res).await
  }
}
