# Experimental Rust text value projection

This is a separately selected JavaScript prototype, `closed standard Rust text
value projection 1`. It interprets complete Rust modules with flat optionally
public text functions and an independent empty `main`. Each function has zero
to two parameters of exactly `String`, `&String` or `&str`, an owned standard
`String` result, and one of the operation forms recorded in
[data/policy.lino](data/policy.lino). All reference lifetimes are elided.

The actual production Rust lexer and parser read the entire token stream with
`Rust source function signature 1`. A finite source-operation certificate checks
the source leaf type before references and constructors are projected into the
value core. It reuses the existing standard-owner and source-signature checks.
A borrowed `str` clone does not become an owned String clone; moving a String
through a shared reference is refused. Standard `String::from` in this scope
requires an actual `&str` input or a raw/cooked string literal. The other selected
forms are an owned String move, String or single-reference String clone, and
borrowed-str/literal `to_owned` or `to_string`.

Every original token and trivia code unit is retained. The source-operation
facts reference the actual declaration, source signature, result position,
checked expression and referenced parameter position. The ordinary typed codec
stores these as a separate inspectable provenance graph with real shared core
LinkIds. The explicit APIs in [model.mjs](model.mjs) revalidate that certificate
at check, encode, decode and emission; the generic public codec alone does not
promise to validate this experimental interpretation.

Text and binary Links reload preserve the identities. Coherent function and
parameter renames, literal value edits and valid String/&String clone retargets
remain executable. Retargeting the same erased read to `&str` refuses. Original
literal spellings replay exactly, and edited literals use the shared Rust
emitter quoter. The complete emitted Rust source is interpreted again before
conditions are attached to its new source intervals. No function text is
extracted, no alternate `main` is substituted, and no cached source payload is
used as a semantic oracle.

The observation is Unicode scalar text under explicitly represented input
conditions and successful allocation for each actual String constructor or
clone. An owned move has no allocation or clone premise. Input guards reject
nonprimitive strings, unpaired surrogates and incorrect arity. The shared
JavaScript character-reading realm condition remains explicit. Pointer identity,
aliasing, ABI, allocation failures/counts and destructor timing are outside this
value observation. This does not compose with the independent generic UTF16
code-unit model.

The fixtures retain thirteen complete original modules, twenty-six exact source
refusals, the actual complete generic Rust emitter artifacts and their receipts,
and protected old fixture hashes. The generic emitter's thread wrapper remains
an explicit refused composition boundary; it is never silently erased. Imports,
modules, custom types/traits, generics, explicit lifetimes, mutation, extra
reference layers, arbitrary calls/effects and borrowed results need additional
proofs. This experiment supplies no native implementation, production selector,
ledger evidence or broad borrow-checking claim.

Run the bounded checks from the repository root:

```sh
node experiments/rust-text-value-projection/generate-rules.mjs --check
node experiments/rust-text-value-projection/generate-fixtures.mjs --check
node --test experiments/rust-text-value-projection/model.test.mjs
```

The generated [Rust decision artifact](data/rules.rs) is produced automatically
from the same linked policy through the actual translator and recorded decorator
styles. It is an experimental artifact, not a native runtime integration.
