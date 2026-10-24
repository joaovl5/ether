use std::cmp::Ordering;
use std::path::Path;

use crate::{EtherError, Res};

pub fn get_file_name(path : &Path) -> Res<&str> {
  let x = path.file_name()
              .and_then(|x| x.to_str())
              .ok_or(EtherError::BadPath)?;
  Ok(x)
}

pub fn cmp_res<X : Ord, Y>(a : &Result<X, Y>, b : &Result<X, Y>) -> Ordering {
  match (a, b) {
    | (Err(_ea), Err(_eb)) => Ordering::Equal,
    | (Ok(_), Err(_e)) => Ordering::Greater,
    | (Err(_e), Ok(_)) => Ordering::Less,
    | (Ok(oa), Ok(ob)) => oa.cmp(ob),
  }
}
