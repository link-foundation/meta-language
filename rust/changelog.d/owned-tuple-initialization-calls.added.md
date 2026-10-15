Add an explicit owned tuple initialization call model for closed Rust String/i32
helpers and complete callers with ordered String moves, i32 copies, replacements
and joins. Linked call targets and source-free edits retain actual declaration
ownership. The existing factory selects the model; other source profiles retain
their boundaries. Native compilation and cross-runtime execution are checked in CI.
