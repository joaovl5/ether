use derive_more::From;
use ether_common::{EtherError, Res};
use toasty::Db;

pub(crate) trait IntoPrimitive {
  type P;
  async fn into_primitive(self, db : &mut Db) -> Res<Self::P>;
}

pub(crate) type UUID = uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, From, toasty::Embed)]
pub struct StoreID(UUID);

#[derive(Debug, Clone, PartialEq, Eq, Hash, From, toasty::Embed)]
pub struct StoreHash(String);

impl From<StoreID> for ether_common::StoreID {
  fn from(value : StoreID) -> ether_common::StoreID {
    value.0.to_string().into()
  }
}

impl From<StoreHash> for ether_common::StoreHash {
  fn from(value : StoreHash) -> ether_common::StoreHash {
    value.0.into()
  }
}

impl TryFrom<ether_common::StoreID> for StoreID {
  type Error = EtherError;
  fn try_from(value : ether_common::StoreID) -> Result<Self, Self::Error> {
    let inner_val : String = value.into();
    Ok(UUID::try_parse(&inner_val)?.into())
  }
}

impl From<ether_common::StoreHash> for StoreHash {
  fn from(value : ether_common::StoreHash) -> Self {
    let inner_val : String = value.into();
    inner_val.into()
  }
}

// #[instrument]
// pub(crate) async fn setup_db() -> Result<DatabaseConnection, EtherError> {
//   // move later to a config reader thingy
//   let conn_str = "sqlite:///ether/state.db?mode=rwc";
//   let mut conn_opts = ConnectOptions::new(conn_str);
//   conn_opts.sqlx_logging(false);
//   trace!("Connecting to DB...");
//   let conn = Database::connect(conn_opts).await?;
//   setup_schema(&conn).await?;
//   Ok(conn)
// }
//
// #[instrument]
// async fn setup_schema(conn : &DatabaseConnection) -> Result<(), EtherError> {
//   const ENTITY_MOD : &str = "ether::db::entity::*";
//   trace!("Setting up schema...");
//   conn.get_schema_registry(ENTITY_MOD).sync(conn).await?;
//
//   Ok(())
// }
