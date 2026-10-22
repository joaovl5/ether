use std::error::Error;

#[derive(Debug)]
pub enum EtherError {
  Other(Box<dyn Error>),
}

impl<T : Error + 'static> From<T> for EtherError {
  fn from(value : T) -> Self {
    EtherError::Other(Box::new(value))
  }
}
