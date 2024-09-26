@_default:
  just --list

test *ARGS:
  cargo nextest run --no-fail-fast {{ARGS}}

coverage:
  cargo llvm-cov nextest --no-fail-fast

bench:
  cargo bench

miri-test:
  cargo miri nextest run --no-fail-fast --all-targets


