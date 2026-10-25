#![allow(unused_variables)]
use ether_common::{Recipe, Res, StoreHash};

use super::Store;

impl Store {
  pub async fn mk_recipe(&mut self, recipe : Recipe) -> Res<StoreHash> {
    todo!()
  }
}
