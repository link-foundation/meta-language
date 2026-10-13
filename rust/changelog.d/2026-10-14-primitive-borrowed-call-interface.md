Add an explicit closed primitive borrowed Rust call interface model. It retains
owned primitive leaf/caller source headers, materializes arguments once in
order under hygienic scalar names, and restores the callee's exact immutable
reference layers. Source-free scalar and header edits change emitted calls.
Existing default and leaf-only models retain their separate behavior. Borrowed
results, static callee borrows, call chains, effects, imports and broader ABI
equivalence require independent proofs. Native compilation and observation
coverage run in CI.
