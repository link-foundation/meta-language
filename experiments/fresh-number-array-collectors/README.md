# Fresh Number array collector experiment

This opt-in JavaScript experiment selects `closed fresh Number array collector 1`.
It preserves each complete original `const out=[]; … out.push(value); …` source
as ordered lexical tokens, exact trivia code units and actual binding records.
Production also has a shared fresh-local append proof. This experiment keeps
its independent, explicitly selected source certificate and narrower grammar.
The production proof applies to owned construction; explicit reference selection
retains the original heap operation and its refusal.

The linked grammar accepts exactly one function, one fresh collector, and one
counted loop from zero below the unchanged input length, incrementing by one.
Each iteration contains one ignored, one-argument `push`; the final statement
returns that collector. Five finite argument patterns cover Number input reads,
the current source index, a constant, and the collector length before append.
Aliases, escaped or captured collectors, identity tests, other calls, property
receivers, multiple arguments, observed push results and unknown source tokens
are refused. This is not generic `Array.push` support.

[data/policy.lino](data/policy.lino) records the complete grammar and four
operation-bound premises: standard array intrinsics without accessors or
prototype changes; dense readonly primitive Number input; successful allocation
under a value observation excluding allocation counts and intermediate identity;
and collector length bounded by the unchanged input length. These are retained
premises, not claims that arbitrary external callers satisfy them. The original
single append per iteration keeps the collector below the ordinary array length
limit. Resource failure behavior is outside this selected observation.

[data/lowering.lino](data/lowering.lino) selects internal `let` state and value
append. Only the two certified original intervals are replaced: the allocation
binding and the ignored call. The complete original source remains inspectable;
its `const` lexical binding is never relabeled as originally mutable. The
existing production parser, inference, imperative lowering, checker and emitters
process the complete resulting value program. No reference-semantics override,
native mutation algorithm or production admission change is introduced.

The whole portable grammar/identity matcher is automatically translated into
[data/rule-translation.lino](data/rule-translation.lino). Every complete lowered
program and its complete emitted Rust text are retained in
[data/fixtures.lino](data/fixtures.lino). No emitted definition or runtime prelude
is removed. Native compilation/execution has not been verified. The source and
archive adapter is JavaScript-only and still needs a faithful native boundary;
the generated matcher alone does not implement it. General object/Map projection
and the existing identity bridge's member-array mutations remain separate gaps.

Eleven bounded tests compare independently authored original-source outcomes
with emitted JavaScript, including pre-append argument scope, and repeat those
checks after Links text and binary reload. Actual checked owner and lexical
binder identities are checked separately from value equality. Source and graph
corruptions and excluded source forms stay rejected. The original collector
inputs also exercise production append admission and source-free Links replay;
aliased collectors and observed push results retain separate refusal controls.
An automatically translated reserved-binding table rejects invalid original
JavaScript names. Saved premise links use the separate `premise owner` role;
ordered declarations and the actual mapping must share the same owned nodes.
A separate generated function-role guard excludes the three nonconfigurable
standard global value properties while retaining valid local shadowing.
The policy and each certificate retain an explicit `sourceContext`: Script
execution in a fresh extensible standard realm, default global descriptors and
no ambient declaration changes. The finite guard does not inspect or certify
arbitrary host environments with additional nonconfigurable global properties.
No original-Rust-source reimport or paired-runtime support is claimed.

From the repository root, run only:

```sh
node scripts/with-cache-cleanup.mjs --event test -- node --test experiments/fresh-number-array-collectors/model.test.mjs
node scripts/with-cache-cleanup.mjs --event test -- node experiments/fresh-number-array-collectors/generate.mjs --check
node scripts/with-cache-cleanup.mjs --event test -- node experiments/fresh-number-array-collectors/generate-fixtures.mjs --check
```

The independent operation basis is
[ECMAScript Array.prototype.push](https://tc39.es/ecma262/multipage/indexed-collections.html#sec-array.prototype.push):
its result is the new length, and its writes affect the original receiver.
The selected grammar excludes observing that result or sharing that receiver.
