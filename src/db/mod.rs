use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::{instrument, trace};

use crate::utils::EtherError;

pub mod entity;

#[instrument]
pub async fn setup_db() -> Result<DatabaseConnection, EtherError> {
  // move later to a config reader thingy
  let conn_str = "sqlite:///ether/state.db?mode=rwc";
  let mut conn_opts = ConnectOptions::new(conn_str);
  conn_opts.sqlx_logging(false);
  trace!("Connecting to DB...");
  let conn = Database::connect(conn_opts).await?;
  setup_schema(&conn).await?;
  Ok(conn)
}

#[instrument]
async fn setup_schema(conn : &DatabaseConnection) -> Result<(), EtherError> {
  const ENTITY_MOD : &str = "ether::db::entity::*";
  trace!("Setting up schema...");
  conn.get_schema_registry(ENTITY_MOD).sync(conn).await?;

  Ok(())
}
