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

treesitter FORMAT="utxt":
    @bundle install &>/dev/null
    bundle exec ruby ./scripts/treesitter.rb | tee uml.out
    plantuml -t{{FORMAT}} uml.out

coverage:
    cargo llvm-cov nextest --no-fail-fast --lcov --output-path ./.lcov

bench:
    # TODO: Set up a Self-hosted runner with known specs to run benchmarks on on CI.
    cargo bench

miri-test:
    cargo miri nextest run --no-fail-fast --all-targets

cloc *args:
  cloc --vcs=git --exclude-ext=.rc . {{args}}

mutants *ARGS:
    # cargo mutants -t 90 -j 8 -E 'bitboard' -E "intrinsics" -E "Mask" -E "tokio" -E "Stockfish" -E "ui" -E "PEXTBoard" --test-tool nextest -- --cargo-profile=mutants --all-targets {{ARGS}} -j 4
    cargo mutants -j 4 -E 'bitboard' -E "intrinsics" -E "Mask" -E "tokio" -E "Stockfish" -E "ui" -E "PEXTBoard" --test-tool nextest -- --cargo-profile=mutants --all-targets {{ARGS}} -j 4


taghunt:
    @just _taghunt "BUG" "FIXME" "HACK" "NOTE" "TODO" "OQ"

_taghunt *TAGS:
    #!/usr/bin/env bash
    for tag in {{TAGS}}; do
        echo -n "$tag=$(rg --glob \!Justfile $tag . | wc -l)<br/>"
    done
    echo

