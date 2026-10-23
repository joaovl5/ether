use crate::{ResOpt, StoreHash, StoreID, StoreObject};

pub trait MetadataBackend: Sized {
  async fn get_obj_by_id(&mut self, id : StoreID) -> ResOpt<StoreObject>;

  async fn get_obj_by_hash(&mut self, hash : StoreHash) -> ResOpt<StoreObject>;
}
