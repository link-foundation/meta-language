Draft JavaScript-first source lowering for single-constructor Rust place matches,
with actual mixed move/ref/Copy arm bindings, readonly owning guard views,
deferred successful transfers and hygienic failed-guard continuations. Preserve
single scrutinee evaluation, actual partial owners and scoped mutable arm storage.
Native originals, forged-graph controls and Links/target round trips are authored
and unexecuted; general refutable patterns and effectful guards remain required.
