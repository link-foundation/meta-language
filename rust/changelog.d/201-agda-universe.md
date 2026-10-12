Preserve Agda universe arguments as children of their universe atom in the
generated native grammar, matching the pinned independent parser. Keep bare
universes valid and cover both forms in the shared JavaScript and Rust fixtures.
Preserve clause-lambda keywords and separators instead of consuming them as
untyped bindings or identifiers, with an independent oracle regression case.
Retain nested module ownership and universal quantifier scope, and resolve
instance-ellipsis module arguments and empty Unicode lambda bindings.
Update the CI debugging artifact uploader to the audited current action version.
