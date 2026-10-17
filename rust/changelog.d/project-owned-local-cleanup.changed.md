Clarify local cleanup scope: inherited Cargo target paths require verified
meta-language ownership before cleanup or wrapped checks. Manual local cleanup
disables low-disk escalation and container cleanup and retains its receipt.
Rust compilation remains exclusive to CI/CD.
