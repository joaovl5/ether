use crate::{ResOpt, StoreHash, StoreID, StoreObject};

pub trait MetadataBackend: Sized {
  async fn get_obj_by_id(self, id : StoreID) -> ResOpt<StoreObject>;

  async fn get_obj_by_hash(self, hash : StoreHash) -> ResOpt<StoreObject>;
}
