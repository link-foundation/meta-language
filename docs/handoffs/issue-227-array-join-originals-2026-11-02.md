# Issue 227 Array join original source draft

The assigned fixture `js/tests/fixtures/javascript-array-join-source.mjs` contains
41 independent literal source programs. Each exports `read`, takes no arguments,
and declares `expected: true`. This is a source draft with no execution evidence.
The exports follow the construction fixture shape: originals, source versions,
and assertion policy. The selected language edition is ECMAScript2025 and the
Node release ranges are `>=22.15 <23 || >=24 <25`. Other Node versions are
uncovered by these declarations.

The programs preserve complete assertions for length access/conversion ordering,
empty receivers and separator conversion, separator defaults and implicit Symbol
errors, sparse inherited getters, Proxy Get without HasProperty, nullish fields,
length snapshots and indexed mutation, nested array separators, original thrown
values, UTF16 String receivers, primitive receivers, actual join selection and
Call, Object toString fallback and Symbol.toStringTag effects, cyclic arrays,
generic receiver reentry, alias reuse and cleanup after throws.

The Proxy constructor original is intentionally retained. Source Proxy
construction remains a broader implementation gap; this draft does not claim
that the portable compiler can execute that input yet. Similar source/native
integration and every actual assertion outcome remain for the parent task.

Algorithm references consulted as source text were
[ECMAScript2025 join](https://tc39.es/ecma262/2025/multipage/indexed-collections.html#sec-array.prototype.join),
[ECMAScript2025 Array toString](https://tc39.es/ecma262/2025/multipage/indexed-collections.html#sec-array.prototype.tostring),
and [V8 join source](https://raw.githubusercontent.com/v8/v8/main/src/builtins/array-join.tq).
The V8 source corroborates receiver-based traversal suppression; it is current
source text, not proof for each Node release in the selected ranges. Attempts to
retrieve the exact Node22.15/24.0 source paths through the web tool failed.
Direct and mutual cycles and same-receiver reentry therefore remain explicitly
Node-bounded expectations requiring native observations on selected versions;
they are not represented as a cycle requirement from ECMAScript2025.

No fixture imports, tests, behavior execution, source generation, validation,
compilation or Rust commands were run. No ledger/source register was read or
changed, and no observation records were produced. Only the two assigned files
were edited. Their text was reread before commit.
