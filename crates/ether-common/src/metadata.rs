use crate::{Res, ResOpt, StoreHash, StoreID, StoreObject};

pub trait MetadataBackend: Sized {
  type StoreObjectT;

  fn mk_id(&self) -> StoreID;

  async fn mk_obj(&mut self, obj : StoreObject) -> Res<Self::StoreObjectT>;

  async fn get_obj_by_id(&mut self, id : StoreID) -> ResOpt<StoreObject>;

  async fn get_obj_by_hash(&mut self, hash : StoreHash) -> ResOpt<StoreObject>;
}
