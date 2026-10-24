use crate::{Res, ResOpt, StoreHash, StoreObject};

pub trait MetadataBackend: Sized {
  type StoreObjectT;

  async fn mk_obj(&mut self, obj : StoreObject) -> Res<Self::StoreObjectT>;

  async fn get_obj_by_hash(&mut self, hash : StoreHash) -> ResOpt<StoreObject>;
}
