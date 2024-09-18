@_default:
  just --list

test:
  cargo nextest run --no-fail-fast --all-targets

coverage:
  cargo llvm-cov nextest --text --no-fail-fast

bench:
  cargo bench

miri-test:
  cargo miri nextest run --no-fail-fast --all-targets


