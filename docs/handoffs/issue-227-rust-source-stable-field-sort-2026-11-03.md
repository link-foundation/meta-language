# Rust stable field sort source draft

This source-only draft targets the actual `Type::map_children` field ordering in
`rust/src/translation/types.rs`. It is not a verification or completion report.

`rust-source-vector-sort.js` admits an actual mutable standard `Vec` place with
nominal record elements and a direct standard owned `String` field. The closed
comparator has two distinct immutable source binders, projects that same field
from each in order, and calls `encode_utf16().cmp(...)`. Optional result braces
and a trailing argument comma preserve that expression. Names alone do not
establish Vec, String or field authority: source owners, actual nominal layouts
and the exclusive lexical state place are independently required.

The admitted operation selects a complete ordinary JavaScript stable insertion
sort source algorithm with one opaque payload type parameter. Only that declared
payload slot is specialized. The ordinary checker validates the full library;
the adapter wraps actual keys and payloads, invokes it and unwraps results. Equal
keys retain input order. Internal source algorithms may calculate this pure key
once per element because the admitted comparator has no side effects, captures,
custom dispatch or observable allocation. Arbitrary callbacks require their own
complete interpretation and are refused by this closed profile.

Exclusive source state transport publishes the sorted value back to the actual
root/field and preserves the Unit result. The private source admission cannot be
invented by a supplied descriptor; no private sorting operation should survive
ordinary checking or enter typed Links archives. Library declaration namespaces
must be fresh and collision-checked. Full source libraries must be appended to
the checked program before recursion/effect analysis and emission.

Formal emission selects the existing complete UTF16 column representation for
explicit checked Rust code-unit comparisons. Plain Rust String scalar/UTF8
ordering retains its separate source model. The generated historical string
order rule files are not edited. Lean and Rocq reuse `MLUtf16Text.compare` through
the existing unit adapter, including a supplementary versus private-use boundary.

Ten complete original programs and six refusals are retained in
`js/tests/fixtures/rust-source-vector-sort.mjs`, with Rust >=1.90.0 <1.100.0 and
explicit Node ranges. Deferred controls cover stable ties, payload retention,
UTF16 order, empty/prefix/zero-byte strings, actual source checking, formal
emission, text/binary Links transport and forged admission rejection. Native
original and emitted Rust controls belong to the existing CI-only matrix.

Remaining boundaries: arbitrary comparator effects/panics, reverse/multiple-field
comparison, non-String keys, custom owners, unsized or escaping borrowed values,
and complete generic/serde module admission. The closed sort frontier alone does
not establish whole-project translation or full `Type::map_children` support.

No tests, generation, validation, behavioral execution or compilation were run.
