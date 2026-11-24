# Owned String accumulation through exclusive source places

This is an unexecuted JavaScript source batch. No tests, behavior/import
execution, generation, validation, compilation, native parity or ledger
observations ran. The explicit model remains `Rust exclusive source loans 1`,
Rust >=1.90.0 <1.100.0, with edition-2024 literals and deferred Node coverage
>=22.15 <23 or >=24 <25. The automatic frontend author/native bridge remains
root integration work; this source draft does not modify native ParseOptions.

Concrete own-code uses include `fields.push_str(&self.ty(&field.ty)?)` and
`fields.push_str(" -> ")` in `rust/src/translation/emit_rocq.rs:667`, and
`text.push_str(&chunks.pop().unwrap_or(0).to_string())` plus formatted chunk
appends in `rust/src/translation/emit_rust.rs:386`. Their enclosing bodies have
independent iterator, residual, formatting, char, Vec-pop and reference source
obligations; they are not claimed admitted as complete modules by this batch.

`rust-exclusive-source-string-rules.js` authenticates the actual standard owned
String receiver and the selected `push_str` or `clear` arity. Suffix admission
reuses the existing Rust String-addition right-operand policy: one actual
shared str/String reference, complete owner, proven source type and correct
wrappers. An owned String argument is not silently borrowed. Shadowed nominal
types and mere string-shaped source facts do not receive standard authority.

`rust-exclusive-source-strings.js` constructs actual source values using
existing checked core concatenation and empty text. It introduces no separate
native String implementation or new byte/scalar conversion. Concatenation
preserves the source Unicode scalar sequences; empty text removes the entire
String value. No UTF-16 index is presented as a Rust UTF-8 byte index. Char
push, truncate, insert, capacity/reserve and String AddAssign retain their
separate source interpretation and abrupt/resource obligations.

The existing receiver capture selects the real mutable lexical root and field
path before reading method arguments. An ordered nonescaping exclusive frame
then checks arguments and rejects overlapping accesses. The String method
reader uses that frame and existing real-place writer; it neither moves the
String out of its owner nor leaves a reference-valued result. Existing source
loan guards and caller-visible result/state writeback remain in force. For a
field receiver, the selected actual String is projected before concatenation;
the writer reconstructs all ancestors from the latest SSA root and preserves
independent sibling state.

Exports are `readRustExclusiveSourceStringMethod(parser,origin,method,path)`
for capture metadata `{method,parameters}` and
`createRustExclusiveSourceStringMutation(parser,place,selected,args,path,span)`
for an owned result value passed to the existing
`recordRustExclusiveSourcePlaceWrite` continuation. The only call-site
integration is in the existing exclusive receiver/method dispatcher. Old
unselected profiles retain their original mutation refusal.

The new fixture `rust-exclusive-source-strings.mjs` contains 12 positive
literal originals and eight independently compiler-classified refusal inputs.
Positive scopes include whole/local String roots, exclusive formals,
Scope-shaped and nested fields, independent owned suffix borrowing,
astral/combining/NUL text with literal UTF-8 byte lengths, repeated clear,
parent reborrows and a foreign-root suffix producer executed once. Stored
mutable aliases, self-borrowing receiver suffixes, mutable str, owned argument
coercion, UTF-8 truncate and String AddAssign preserve explicit boundaries.
The original Rust escape sequences are retained in their literal source text.

Deferred controls cover rule admission/forgery cases, full Rust parse/check to
generated JavaScript output, concrete source text/binary transport, typed
Links text/binary round trips and every emitter. These declarations are not
test observations, public support claims or runtime parity evidence.
