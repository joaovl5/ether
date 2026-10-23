use ether_common as cmm;
use ether_common::{EtherError, MetadataBackend, ResOpt};
use toasty::{Db, models};

use crate::models::StoreObject;
use crate::utils::StoreID;

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
}

impl MetadataBackend for EtherDatabase {
  async fn get_obj_by_id(&mut self,
                         id : ether_common::StoreID)
                         -> ResOpt<ether_common::StoreObject> {
    // this means its incompatible w. old dbs or other
    // metadata-backend data without some sort of
    // migration
    let db_id : StoreID = id.try_into()?;
    // FIXME: handle none case later
    let db_obj = StoreObject::get_by_id(&mut self.db, db_id).await?;

    Ok(Some(db_obj.into()))
  }

  async fn get_obj_by_hash(&mut self,
                           hash : ether_common::StoreHash)
                           -> ResOpt<ether_common::StoreObject> {
    todo!()
  }
}
