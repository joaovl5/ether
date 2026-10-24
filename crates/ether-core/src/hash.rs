use std::path::Path;

use ether_common::{EtherError, Res};

#[derive(Debug, PartialEq, Eq)]
pub enum HashAlgo {
  BLAKE3,
}

impl HashAlgo {
  pub fn make_path(&self, path : &Path) -> Res<String> {
    match self {
      | HashAlgo::BLAKE3 => self.blake3_make_path(path),
    }
  }

  fn get_file_name<'a>(&self, path : &'a Path) -> Res<&'a str> {
    let file_name = path.file_name()
                        .and_then(|x| x.to_str())
                        .ok_or(EtherError::BadPath)?;
    Ok(file_name)
  }

  fn blake3_make_path(&self, path : &Path) -> Res<String> {
    let mut hasher = blake3::Hasher::new();
    self.blake3_handle_hash(path, &mut hasher)?;
    let hash = hasher.finalize();
    Ok(hash.to_string())
  }

  fn blake3_handle_hash(&self,
                        path : &Path,
                        hasher : &mut blake3::Hasher)
                        -> Res<()> {
    let file_name = self.get_file_name(path)?;

    if path.is_dir() {
      hasher.update(format!("<dir name='{}'>", file_name).as_bytes());
      let mut dirs = path.read_dir()?
                         .map(|x| x.map(|entry| entry.path()))
                         .collect::<std::io::Result<Vec<_>>>()?;
      dirs.sort();

      for subpath in dirs {
        self.blake3_handle_hash(&subpath, hasher)?;
      }
      hasher.update(b"</dir>");
    } else {
      // can be parametrized by heuristics since this may be slower for HDDs and
      // when CPUs are already busy
      // and also maybe
      hasher.update(format!("<file name='{}'>", file_name).as_bytes())
            .update_mmap_rayon(path)?
            .update(b"</file>");
    }

    Ok(())
  }
}
