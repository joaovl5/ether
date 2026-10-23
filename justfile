set positional-arguments

@_:
  just --list

run *args:
  RUST_LOG=trace cargo run "$@"

db-cli *args:
  RUST_LOG=trace cargo run --bin ether-db-cli -- "$@"

