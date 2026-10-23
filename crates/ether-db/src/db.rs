use ether_common::EtherError;

#[derive(Debug)]
pub struct EtherDatabase {}

impl EtherDatabase {
  pub async fn new() -> Result<Self, EtherError> {
    Ok(Self {})
  }
}
