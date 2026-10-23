#![allow(dead_code, clippy::upper_case_acronyms)]

use std::env::current_dir;
use std::fs::create_dir_all;
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueHint};
use ether_common::EtherError;
use ether_core::store::Store;
use ether_db::setup_db;
use human_panic::setup_panic;
use time::macros::format_description;
use tokio::main;
use tracing::info;

// mod config;
// mod core;
// mod db;
// mod utils;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
  #[arg(short, long, default_value = "/ether/store", value_hint = ValueHint::DirPath)]
  store_path : PathBuf,

  #[command(subcommand)]
  command : CliCommand,
}

#[derive(Subcommand, Debug)]
enum CliCommand {
  CopyPath {
    #[arg(short, long, value_hint = ValueHint::AnyPath)]
    copy_from : PathBuf,
  },
}

fn setup_logging() {
  use tracing_subscriber::fmt::time::LocalTime;
  use tracing_subscriber::{EnvFilter, fmt};

  let time_format = format_description!("[hour]:[minute]:[second]");
  fmt().with_timer(LocalTime::new(time_format))
       .with_env_filter(EnvFilter::from_default_env())
       .compact()
       .init();
}

#[main]
async fn main() -> Result<(), EtherError> {
  setup_panic!();
  setup_logging();
  let args = CliArgs::parse();
  let cwd = current_dir().unwrap();
  let store_dir = cwd.join(args.store_path);
  let mut store = Store::new(store_dir.clone());
  create_dir_all(&store_dir).unwrap();
  let _conn = setup_db().await?;
  match args.command {
    | CliCommand::CopyPath { copy_from, } => {
      let x = store.copy_path(&copy_from).unwrap();
      info!("Copied to {:?}", x.to_string_lossy())
    },
  }

  Ok(())
}
