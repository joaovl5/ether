use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
pub enum HashAlgo {
  BLAKE3,
}

#[derive(Debug)]
pub enum HashError {
  FileMissing,
  PermissionDenied,
  BadPath,
  UnexpectedIOError(Box<std::io::Error>),
}

impl From<std::io::Error> for HashError {
  fn from(value : std::io::Error) -> Self {
    use std::io::ErrorKind::*;
    match value.kind() {
      | NotFound => Self::FileMissing,
      | PermissionDenied => Self::PermissionDenied,
      | _ => Self::UnexpectedIOError(Box::new(value)),
    }
  }
}

impl HashAlgo {
  pub fn make_path(&self, path : &Path) -> Result<String, HashError> {
    match self {
      | HashAlgo::BLAKE3 => self.blake3_make_path(path),
    }
  }

  fn get_file_name<'a>(&self, path : &'a Path) -> Result<&'a str, HashError> {
    let Some(file_name) = path.file_name().and_then(|x| x.to_str()) else {
      return Err(HashError::BadPath);
    };
    Ok(file_name)
  }

  fn blake3_make_path(&self, path : &Path) -> Result<String, HashError> {
    let mut hasher = blake3::Hasher::new();
    self.blake3_handle_hash(path, &mut hasher)?;
    let hash = hasher.finalize();
    Ok(hash.to_string())
  }

  fn blake3_handle_hash(&self,
                        path : &Path,
                        hasher : &mut blake3::Hasher)
                        -> Result<(), HashError> {
    let file_name = self.get_file_name(path)?;

    if path.is_dir() {
      hasher.update(format!("<dir name='{}'>", file_name).as_bytes());
      let dir_reader = path.read_dir()?;
      for entry in dir_reader {
        let subpath = entry?.path().as_path();
        self.blake3_handle_hash(path, hasher)?;
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
