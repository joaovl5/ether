set positional-arguments

@_:
  just --list

RUST_LOG := "ether=trace,toasty=warn,debug"

run *args:
  RUST_LOG={{RUST_LOG}} cargo run --bin ether -- "$@"

db-cli *args:
  RUST_LOG={{RUST_LOG}} cargo run --bin ether-db-cli -- "$@"

