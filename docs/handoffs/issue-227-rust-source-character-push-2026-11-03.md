# Rust literal character append draft

Source-only continuation of the authoritative vision. `String::push` of a
canonical scalar character literal now has a source reader draft, targeting
`rust/src/translation/emit_javascript/names.rs` and the generated numeric text
helpers. It reuses the existing canonical character lexer, actual standard owned
String receiver authority, exclusive mutable root/field selection, concatenation
and caller state writeback. It introduces no native replacement semantics or
general character-to-String coercion.

The literal must be the actual source token, retain exact original coordinates
and spelling, and denote a valid Unicode scalar. Byte characters, integers,
strings, missing/extra arguments and immutable places refuse. The token's scalar
becomes its exact internal text value only within this closed standard method.
No user binding receives an invented character type or Copy/ownership proof.

Sixteen complete literal originals cover ASCII, non-ASCII, supplementary values,
zero/newline, Unicode escapes, quote/backslash, consecutive calls, clear/append,
actual record fields, exclusive methods and zero-byte-boundary prefix insertion.
The actual zero `usize` literal is always a valid UTF8 boundary, including empty
text; this closed insertion reuses the same text state read with concatenation
operands reversed. Other byte indices require their separate panic/boundary
interpretation. Nine refusal inputs preserve the
distinct argument and receiver domains. Deferred tests cover JavaScript products
and text/binary Links at Rust 1.90 and 1.99; the CI-only original/product matrix
selects the active compiler within >=1.90 <1.100. Node ranges are explicit.

Remaining required character work includes typed parameters/locals/results,
computed scalar values, general byte-index insertion, character iteration and scalar operations, mutable
temporaries and general method callbacks. Those remain distinct obligations;
this literal source path does not claim their admission or full emitter support.

No tests, generation, validation, compilation or represented execution ran.
