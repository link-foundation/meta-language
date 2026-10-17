The JavaScript translator now drafts structural records, nullable values,
immutable closures, iterable collection construction and array callbacks.
String methods in Rust output use an explicit UTF-16 code-unit representation,
including isolated surrogates produced by slicing or splitting. Module
translation selects that representation for every sibling item together.

Rust trimming in JavaScript output uses Unicode White_Space, retaining the
NEL/BOM distinction from ECMAScript trimming. Shared fixtures cover both
predicates. Mutable collection aliases, complete out-of-range read semantics,
the pure targets' UTF-16 string runtime and the Rust host compiler's portable
expression parity remain acceptance work.
