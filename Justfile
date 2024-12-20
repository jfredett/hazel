@_default:
    just --list

# Run the CI pipeline
ci *ARGS: doctest nextest

# Run the tests
nextest *ARGS:
    cargo nextest run --no-fail-fast {{ARGS}}

# Run the doctests, which aren't done by nextest
doctest:
    cargo test --doc

# Dev Loop
test *ARGS:
    #!/usr/bin/env bash
    set -euo pipefail

    delta_time() {
        local THRESHOLD_FOR_RERUN=300
        local current_time=$(date +%s)
        if [ -e ./.last_run ]; then
            local last_run=$(cat ./.last_run)
            local delta=$((${current_time} - ${last_run}))
            if [ $delta -lt $THRESHOLD_FOR_RERUN ]; then
                echo "Last run was ${delta} < ${THRESHOLD_FOR_RERUN} seconds ago, skipping"
                return 1
            fi
        fi
        echo "${current_time}" > ./.last_run
        return 0
    }




    if [[ -n "{{ARGS}}" ]]; then
        echo "Running tests with args: {{ARGS}}"
        just nextest {{ARGS}}
    else
        echo "Smoke Check"
        just doctest

        if [[ ! -e ./.lcov ]] || delta_time; then
            echo "Refreshing coverage data"
            just coverage
        else 
            echo "Using cached coverage data"
            just nextest
        fi
    fi


coverage:
    cargo llvm-cov nextest --no-fail-fast --lcov --output-path ./.lcov

bench:
    # TODO: Set up a Self-hosted runner with known specs to run benchmarks on on CI.
    cargo bench

miri-test:
    cargo miri nextest run --no-fail-fast --all-targets

cloc *args:
  cloc --vcs=git --exclude-ext=.rc . {{args}}

mutants:
    # FIXME: I don't feel great about excluding all of `engine`, but these routinely fail in ways that are not useful.
    # Probably I could be more judicious with this filter, but I am lazy.
    cargo mutants --exclude "src/engine/*" "src/ui/*" -j 4 --no-shuffle -- --profile=mutants --all-targets


