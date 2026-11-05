# Protected String default casing source draft

This source handoff supplements [the vision](../vision.md); it is not another
architecture specification or execution evidence. Source drafting only occurred.
No test, parser/generator, validation gate, represented program or compiler ran.

`javascript-string-method-call-source.js` adds protected toLowerCase/toUpperCase
bodies. RequireObjectCoercible precedes ToString, with the existing represented
string-hint conversion continuation preserving callback order and original
throws. The fifth compiler argument carries the actual active Unicode version;
a missing/empty/oversized authority refuses compilation for these two methods.
Other methods keep their previous arguments and behavior. A nonempty uncovered
profile stays explicit in the body and reports no authored table when invoked.

`javascript-string-method-rules.js` composes the existing complete Unicode 16
source/table algorithm. It does not use native character mapping or an ASCII
substitute. Full expansions, original-string final-sigma context, possessive
Case_Ignorable precedence, supplementary points and isolated UTF16 units retain
the shared definitions. The semantic operation requires exact profile `16.0`.
Unicode 15.0/15.1, 17.0 and other profiles report uncovered coverage; they do not
silently receive Unicode16 behavior. Input and output each have a 1,048,576-unit
work bound; the pinned mappings bound intermediate expansion to three times the
input. A large output reports resources separately from language exceptions.

The new literal fixture has twelve original source cases. It declares Node
>=24.0.0 <25.0.0 **and actual Unicode 16.0**, plus the existing generated Rust
1.90–1.99/edition 2024 profile. Dedicated controls require the active native profile
before original execution, report uncovered versions without skipped cells, and
compare portable VM output against independent original Node output. They cover
generic receivers and string hints, ordinary fallback order, argument effects,
original throws, nullish errors, expansions, contextual sigma, supplementary and
isolated units, own-method shadowing and a Unicode16-specific Cyrillic mapping.
Three deferred rule/body controls cover profile refusal, the authenticated body
coordinate, full mapping values and the input resource boundary. Nothing ran.

## Required root integration

- Add optional `unicodeProfile?:string` to primitive-method ownership metadata.
  Privately capture `process.versions.unicode` at the actual realm boundary and
  attach it to String toLowerCase/toUpperCase owners. Authenticate that captured
  authority during realm verification; caller-supplied metadata is not its source.
- Pass `method.unicodeProfile` as the fifth argument to
  `compileJavaScriptStringMethodFunction` in collection directory compilation.
  Reject invalid/missing profile metadata for the two methods explicitly.
- Existing broad String-body control loops over every admitted method. Pass
  `16.0` as the fifth argument there so the test's profile is explicit; retain all
  former methods, inputs and assertions.
- Register `js/tests/translation-javascript-string-case-profile.test.js` and its
  literal fixture export with the deferred automatic source/native author.
  Existing String rule/body libraries and Unicode rule/table authors remain the
  semantic sources. Regenerate source programs/Links/native artifacts only in
  the later permitted phase; never hand-author Rust replacement mappings.
- Update the vision's protected String integration and generic-receiver scope,
  changelog and common handoff. Keep alternate Unicode profiles, locale casing,
  full input-domain resource profiles, and every inherited obligation explicit.
- Source register, naming/feature declarations, ledger reconciliation and actual
  execution observations remain deferred under the current phase contract.
