set positional-arguments

@_:
  just --list

run *args:
  RUST_LOG=trace cargo run "$@"

