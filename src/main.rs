#![allow(dead_code,
         clippy::upper_case_acronyms,
         unused_variables,
         unused_imports)]

use std::env::current_dir;

use self::core::store::Store;

mod config;
mod core;

fn main() {
  let store = Store::new();
  let cwd = current_dir().unwrap().into_string().unwrap();
  println!("Current dir is {:?}", cwd);
}
