use std::path::{Path, PathBuf};
use std::{fs, io};

use ether_common::{EtherError, MetadataBackend};
use ether_db::EtherDatabase;

use super::Drv;
use crate::hash::{HashAlgo, HashError};

#[derive(Debug)]
pub struct Store<Backend = EtherDatabase>
  where Backend : MetadataBackend,
{
  pub root_path : PathBuf,
  pub hash : HashAlgo,

  pub backend : Backend,
}

#[derive(Debug)]
pub struct StoreObjectData {}

#[derive(Debug)]
pub enum StoreError {
  Hash(HashError),
  IO(io::Error),
}

impl From<io::Error> for StoreError {
  fn from(value : io::Error) -> Self {
    Self::IO(value)
  }
}

impl From<HashError> for StoreError {
  fn from(value : HashError) -> Self {
    Self::Hash(value)
  }
}

impl Store {
  pub async fn new(dir : PathBuf) -> Result<Store<EtherDatabase>, EtherError> {
    fs::create_dir_all(&dir)?;
    let backend = EtherDatabase::new().await?;
    Ok(Store { root_path : dir,
               hash : HashAlgo::BLAKE3,
               backend })
  }

  pub fn copy_path(&mut self, path : &Path) -> Result<PathBuf, StoreError> {
    let contents_hash = self.hash.make_path(path)?;
    let store_path = self.make_store_path(contents_hash)?;
    self.copy_into(path, &store_path)?;
    Ok(store_path)
  }

  fn copy_into(&mut self, from : &Path, to : &Path) -> Result<(), StoreError> {
    let name = from.file_name()
                        .and_then(|x| x.to_str())
                        .ok_or_else(|| {
                          const MSG : &str = "source has no valid name";
                          io::Error::new(io::ErrorKind::InvalidInput, MSG)
                        })?;
    let metadata = fs::symlink_metadata(from)?;
    if !metadata.is_file() && !metadata.is_dir() {
      const MSG : &str = "source wasn't a regular file or directory";
      let io_err = io::Error::new(io::ErrorKind::InvalidInput, MSG);
      return Err(StoreError::IO(io_err));
    }
    fs::create_dir_all(to)?;
    let destination = to.join(name);
    if metadata.is_dir() {
      fs::create_dir_all(&destination)?;
      for entry in fs::read_dir(from)? {
        self.copy_into(&entry?.path(), &destination)?;
      }
    } else {
      fs::copy(from, &destination)?;
    }

    Ok(())
  }

  fn make_store_path(&mut self,
                     contents_hash : String)
                     -> Result<PathBuf, StoreError> {
    let out = self.root_path.join(contents_hash);
    fs::create_dir_all(out.clone())?;
    Ok(out)
  }

  pub fn build_drv(&mut self, _drv : &Drv) {}
}
