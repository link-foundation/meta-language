Retain bounded, credential-redacted command output in CI progress heartbeats so
recent test diagnostics survive a job timeout even when artifact upload cannot run.
Reuse the existing package-specific test optimization in the acceptance suite,
keeping debug assertions, overflow checks, default features and all test targets.
