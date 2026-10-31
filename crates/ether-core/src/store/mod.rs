use std::path::{Path, PathBuf};
use std::{fs, io};

use ether_common::{FileLike, GenericHash, Hasher, HasherFactory,
                   MetadataBackend, Res, StoreHash, StoreObject,
                   StoreObjectValue};
use ether_db::EtherDatabase;
use tracing::debug;

mod recipe;

pub struct Blake3Factory;

impl HasherFactory for Blake3Factory {
  type HasherT = blake3::Hasher;
  fn make(&self) -> Res<Self::HasherT> {
    Ok(blake3::Hasher::new())
  }
}

#[derive(Debug)]
pub struct Store<Backend = EtherDatabase, HashFacT = Blake3Factory>
  where Backend : MetadataBackend,
        HashFacT : HasherFactory,
{
  pub root_path : PathBuf,
  pub hasher_fac : HashFacT,

  pub backend : Backend,
}

#[derive(Debug)]
pub struct StoreObjectData {}

impl<Backend, HashFacT> Store<Backend, HashFacT>
  where Backend : MetadataBackend,
        HashFacT : HasherFactory,
{
  pub async fn new(dir : PathBuf) -> Res<Store<EtherDatabase>> {
    fs::create_dir_all(&dir)?;
    let backend = EtherDatabase::new().await?;
    Ok(Store { root_path : dir,
               hasher_fac : Blake3Factory {},
               backend })
  }

  fn to_store_hash(&self, x : GenericHash) -> StoreHash {
    Into::<String>::into(x).into()
  }

  fn hash_path(&mut self, path : &Path) -> Res<StoreHash> {
    Ok(self.to_store_hash(self.hasher_fac.make()?.with_path(path)?.build()?))
  }

  async fn hash_exists(&mut self, x : &StoreHash) -> Res<bool> {
    self.backend
        .get_by_hash(x.clone())
        .await
        .map(|r| r.is_some())
  }

  pub async fn copy_path(&mut self, path : &Path) -> Res<PathBuf> {
    let contents_hash = self.hash_path(path)?;
    let store_path = self.make_store_path(&contents_hash)?;

    if self.hash_exists(&contents_hash).await? {
      debug!("Skipping copy since object is already in store");
      return Ok(store_path);
    }

    self.copy_into(path, &store_path)?;
    let loc : PathBuf = path.into();
    let value =
      StoreObjectValue::FileLike(FileLike { original_location : Some(loc) });
    let obj = StoreObject { hash : contents_hash,
                            value };
    self.backend.mk(obj).await?;

    Ok(store_path)
  }

  fn copy_into(&mut self, from : &Path, to : &Path) -> Res<()> {
    let name = from.file_name()
                   .and_then(|x| x.to_str())
                   .ok_or(io::Error::new(io::ErrorKind::InvalidInput,
                                         "source has no valid name"))?;
    let metadata = fs::symlink_metadata(from)?;
    if !metadata.is_file() && !metadata.is_dir() {
      return Err(io::Error::new(io::ErrorKind::InvalidInput,
                                  "source wasn't a regular file/directory").into());
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

  pub fn make_store_path(&mut self, hash : &StoreHash) -> Res<PathBuf> {
    let hash : String = hash.clone().into();
    let out = self.root_path.join(hash);
    fs::create_dir_all(out.clone())?;
    Ok(out)
  }
}
