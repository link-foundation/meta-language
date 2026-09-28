#!/usr/bin/env bash
# Keeps short-lived processes named rustc starting and exiting, like a cargo
# build does, so process listings keep catching Cargo-family processes mid-exit.
#   bash experiments/cache-cleanup-exit-race/churn.sh SECONDS
set -euo pipefail
dir=$(mktemp -d)
cp "$(type -P sleep)" "$dir/rustc"
end=$((SECONDS + ${1:-60}))
while [ $SECONDS -lt $end ]; do
  for i in $(seq 1 40); do "$dir/rustc" "0.0$((i % 8 + 2))" & done
  wait
done
rm -rf "$dir"
