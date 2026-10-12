Repository tooling now replays structured-data conversion against immutable
original Git inputs in CI, checks every external asset path with actual Git
staging, and runs prepared native snapshots with upstream file and network
access denied. Custom download caches inside a worktree require explicit ignore
rules and cannot overwrite tracked cache inputs. The broader conversion and
offline-delivery ledger cells still require their complete native and consumer
witnesses.
