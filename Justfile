@_default:
  just --list

test *ARGS:
  cargo nextest run --no-fail-fast {{ARGS}}
  cargo test --doc

coverage:
  cargo llvm-cov nextest --no-fail-fast --lcov --output-path ./.lcov

bench:
  cargo bench

miri-test:
  cargo miri nextest run --no-fail-fast --all-targets


