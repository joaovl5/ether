use std::error::Error;

#[derive(Debug)]
pub enum EtherError {
  BadPath,
  Other(Box<dyn Error>),
}

impl<T : Error + 'static> From<T> for EtherError {
  fn from(value : T) -> Self {
    EtherError::Other(Box::new(value))
  }
}

pub type Res<T> = Result<T, EtherError>;
pub type ResOpt<T> = Result<Option<T>, EtherError>;
