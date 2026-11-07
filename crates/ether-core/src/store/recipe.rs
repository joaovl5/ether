#![allow(unused_variables)]
use ether_common::{Hasher, HasherFactory, Recipe, Res, StoreHash};
use tracing::debug;

use super::Store;

impl Store {
  fn hash_recipe(&mut self, recipe : &Recipe) -> Res<StoreHash> {
    Ok(self.to_store_hash(self.hasher_fac.make()?.with_t(recipe)?.build()?))
  }

  /// Persist a recipe into the store (this doesn't bake the recipe)
  pub async fn save_recipe(&mut self, recipe : Recipe) -> Res<StoreHash> {
    let recipe_hash = self.hash_recipe(&recipe)?;
    if self.hash_exists(&recipe_hash).await? {
      debug!("Skipping recipe save since object is already in store");
      return Ok(recipe_hash);
    }

    let store_path = self.make_store_path(&recipe_hash)?;
    store_path.into_string();
    // TODO:
    // - serialize json as str, or straight to IO writer (preferrable)
    // - save store object
    // - make process transactional somehow? also for copy_path?

    todo!()
  }
}
