This is a JavaScript-only experiment for the explicit source interpretation
`closed generated JavaScript Unit module 1`. It has no production parser,
checker, codec, emitter or public API registration. Use the experimental
factory and emitter together; the generic public APIs alone do not enforce
this experimental source facet. It does not claim native runtime support or
issue-195 acceptance evidence.

The input is an entire actual generated JavaScript module: an unescaped strict
directive, closed zero-argument helpers returning `null`, one owned `main`
definition with unused constant-bound helper calls, and its final invocation.
Within those proven roles, singleton `null` encodes Unit. Nullable options,
external interfaces, inspected results, undefined helper results and additional
effects are excluded. Generated reserved-prefix locals are admitted only as
owned entry binders. The default JavaScript reader remains unchanged.

The saved proof contains every original lexer token, token spelling, interval
and intervening trivia code unit. It links helper occurrences to actual checked
declarations, local binders to actual ordered effects, and both entry
occurrences to an independent marker on the checked main. The shared production
predicate `acceptJavaScriptLineTerminator` controls return ASI and line comments;
the experimental policy explicitly records its shared policy, source and
four-code-unit assumption (LF, CR, line separator and paragraph separator).
Entry locals cannot shadow any helper or entry, including through a later
temporal-dead-zone binding. Same-target emission renders the complete owned CST; it reproduces the
input exactly until checked semantic edits change live names or targets.

Rust emission re-reads its actual complete emitted artifact under the existing
closed runtime-shell model. Its four successful-runtime assumptions identify
actual operations of that target. Incoming Rust assumptions are retained as
explicit fixture witnesses; comments in a generated JavaScript file never
supply those assumptions. Failure-path equivalence is not asserted. Rust is
not compiled or executed by this experiment.

The fixture generator automatically obtains eight complete actual JavaScript
artifacts from the existing Rust fixtures and real public emitter. It preserves
the original name/call/output oracles and records the unchanged default reader
results. Tests perform three full code cycles for each artifact, execute the
whole JavaScript modules, re-read whole Rust targets, and check linked semantic
edits and adversarial inputs. These are experiment assertions, not ledger
observations.

Run the bounded checks from the repository root with Node 22:

```sh
node experiments/generated-javascript-unit/generate-rules.mjs --check
node experiments/generated-javascript-unit/generate-fixtures.mjs --check
TRANSLATION_CHECK_RUST=0 SELF_TRANSLATION_CHECK_RUST=0 node scripts/with-cache-cleanup.mjs --event test --no-preflight --no-docker -- node --test experiments/generated-javascript-unit/model.test.mjs
```

This finite model accepts ASCII identifiers and the closed syntax above. It
conservatively refuses other syntax and trivia. It uses the existing generic
Links codec, with its explicit experimental reload check, and stores no opaque
source envelope or cached target emission. A public selector, shared production
validation hooks, native pairing and central schema registration require a
separately reviewed integration. Required CI runs this bounded experiment and
both generated-data freshness checks without compiling Rust in the experiment.
