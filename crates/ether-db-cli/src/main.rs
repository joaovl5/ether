use toasty_cli::{Config, ToastyCli};
use tokio::main;

#[main]
async fn main() -> anyhow::Result<()> {
  let db = ether_db::get_db().await?;
  let cfg = Config::load()?;
  let toasty_cli = ToastyCli::with_config(db, cfg);
  toasty_cli.parse_and_run().await?;

  Ok(())
}
