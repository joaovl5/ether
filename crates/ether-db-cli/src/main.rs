use toasty::{Db, models};
use toasty_cli::{Config, ToastyCli};
use tokio::main;

// TODO: CHANGE LATER
const ETHER_DB_PATH : &str = "sqlite:/ether/state.db";

#[main]
async fn main() -> anyhow::Result<()> {
  let model_set = models!(ether_db::*);

  let cfg = Config::load()?;
  let db = Db::builder().models(model_set)
                        .connect(ETHER_DB_PATH)
                        .await?;
  let toasty_cli = ToastyCli::with_config(db, cfg);
  toasty_cli.parse_and_run().await?;

  Ok(())
}
