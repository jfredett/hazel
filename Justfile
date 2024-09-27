@_default:
    just --list

# Run the CI pipeline
ci *ARGS: doctest nextest

# Run the tests
nextest:
    cargo nextest run --no-fail-fast

# Run the doctests, which aren't done by nextest
doctest:
    cargo test --doc

# Dev Loop
test:
    #!/usr/bin/env bash
    set -euo pipefail

    echo "Smoke Check"
    just doctest

    if [[ ! -f ./.lcov ]] || [[ $(find ./.lcov -mmin +5) ]]; then
        echo "Refreshing coverage data"
        just coverage
    else 
        echo "Using cached coverage data"
        just nextest
    fi

coverage:
    cargo llvm-cov nextest --no-fail-fast --lcov --output-path ./.lcov

bench:
    # TODO: Set up a Self-hosted runner with known specs to run benchmarks on on CI.
    cargo bench

miri-test:
    cargo miri nextest run --no-fail-fast --all-targets
