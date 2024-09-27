@_default:
    just --list

test *ARGS:
    #!/usr/bin/env bash

    # Run the tests
    cargo nextest run --no-fail-fast {{ARGS}}

    # Run the doctests, which aren't done by nextest
    cargo test --doc

    # this next line if the .lcov file is older than like, 15 minutes, by running coverage
    if [[ ! -f ./.lcov ]] || [[ $(find ./.lcov -mmin +15) ]]; then
        just coverage
    fi

coverage:
    cargo llvm-cov nextest --no-fail-fast --lcov --output-path ./.lcov

bench:
    # TODO: Set up a Self-hosted runner with known specs to run benchmarks on on CI.
    cargo bench

miri-test:
    cargo miri nextest run --no-fail-fast --all-targets
