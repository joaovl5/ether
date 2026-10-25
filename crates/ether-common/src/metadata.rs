use crate::{Res, ResOpt, StoreHash, StoreObject, StoreRefType};

pub trait MetadataBackend: Sized {
  type StoreObjectT;

  async fn mk(&mut self, obj : StoreObject) -> Res<Self::StoreObjectT>;

  /// Get a `StoreObject` by its hash.
  async fn get_by_hash(&mut self, hash : StoreHash) -> ResOpt<StoreObject>;

  /// Get all `StoreObject` references where it's the left-hand side of the
  /// reference; i.e.: where is the "source".
  async fn get_lhs_refs(&mut self,
                        hash : StoreHash)
                        -> Vec<(StoreHash, StoreRefType)>;

  /// Get all `StoreObject` references where it's the right-hand side of the
  /// reference; i.e.: where is the "target".
  async fn get_rhs_refs(&mut self,
                        hash : StoreHash)
                        -> Vec<(StoreHash, StoreRefType)>;
}
