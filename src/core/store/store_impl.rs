use std::env::temp_dir;
use std::path::{Path, PathBuf};

use super::{Drv, StoreId, StoreObject};
use crate::core::hash::{HashAlgo, HashError};

#[derive(Debug, PartialEq, Eq)]
pub struct Store {
  root_path : PathBuf,
  hash : HashAlgo,
}

#[derive(Debug)]
pub struct StoreObjectData {}

pub enum StoreError {
  Hash(HashError),
}

impl From<HashError> for StoreError {
  fn from(value : HashError) -> Self {
    Self::Hash(value)
  }
}

impl Store {
  pub fn new() -> Self {
    Store { root_path : temp_dir(),
            hash : HashAlgo::BLAKE3 }
  }

  pub fn copy_path(&mut self, path : &Path) -> Result<(), StoreError> {
    let contents_hash = self.hash.make_path(path)?;
    // let x = (self.root_path.join(obj_id_str));

    todo!()
  }

  pub fn build_drv(&mut self, drv : &Drv) {}
}
