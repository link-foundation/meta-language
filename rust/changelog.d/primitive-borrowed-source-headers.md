Add an explicit `primitive borrowed Rust function header 1` emission model for
owned linked Rust source signatures on flat leaf functions. Direct `f64` and
`bool` results and direct or immutable-reference primitive parameters retain
their declared reference layers and ordered ASCII input lifetime binders.
Scoped copies project referenced parameters into the existing checked value
body, with source-free edits and reimport of the actual emitted definitions.
Existing emission defaults remain unchanged. Calls, effects, allocations,
borrowed results, custom owners, pointer observations, resource equivalence and
general ABI equivalence require separate models and proofs.
