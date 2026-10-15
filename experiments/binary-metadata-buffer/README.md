# Binary metadata buffering experiment

This isolated candidate buffers one record's metadata before appending its
length and payload. It removes the old rewrite of the accumulated output
prefix. Production rules and native generated files retain their prior bytes.

`prior-rules.mjs` is the exact project source at
`b96e4ae8f5e0b4d3d9cd938838f15b67948fdec4`, path
`js/src/link-network-binary-rules.js`, Git blob
`521b26c0261fcec3725474d23726bd26cb7fc7a4`, SHA-256
`4f0debafedcc6e1985ccad8ed0dd673261e511e705d4d7535db008d446cf11a4`.
The test authenticates this frozen oracle before comparing it with
`candidate.mjs`.

The bounded comparison retains both directions and v1/v2 archives, real graph
metadata, cyclic references, caches and allocation history, all 22 refusal
codes with their offsets, every fixture truncation, byte and field mutations,
and resource-error precedence. It compares full returned bytes or fields and
checks that input arrays remain unchanged. It does not replace any existing
test or fixture.

Run from the repository root:

```sh
TRANSLATION_CHECK_RUST=0 SELF_TRANSLATION_CHECK_RUST=0 node scripts/with-cache-cleanup.mjs --event test -- node --test experiments/binary-metadata-buffer/model.test.mjs
```

The candidate passed the actual JavaScript parser, checker and Rust emitter:
53 checked declarations produced 53 native definitions. Conservative native
generation then stopped at the existing exact-context guard in
`js/scripts/generate-link-network-binary-rules.mjs:357`, reporting
`a reviewed binary declaration context is stale`.
The recorded `binary-network-native-reviewed-style` and follow-up layer retain
their authentic published source and diagnostic contexts. A changed framing
continuation cannot silently inherit those full-declaration corrections.
The attempted production and style edits were restored; no native output was
written. `generation-gap.lino` records the exact attempt and input digests.

This is a native-generation style migration gap, rather than a source grammar
admission refusal. It needs a separately reviewed mapping of the preserved
corrections to the new actual generated declarations. No Rust compilation,
large experiment, local timing profile or CI speedup claim accompanies this
candidate.

The subsequent explicit candidate generation mode preserves that failed
attempt and the original production pipeline. It first emits the complete
frozen prior source and validates all 36 historical replacement contexts.
It then derives separate styles from complete current declarations: exact
matches and bijective bound-name variants may reuse their validated prior
correction, while changed declarations receive fresh structural library
styles. Introduced bindings cannot capture current names; a locally constructed
header may be borrowed only by the actual checked slice parameter.
The conservative mapper also keeps qualified callees distinct from local
bindings, requires the actual generated sibling path for header calls, and
checks unary-condition and negative-receiver grouping before reusing shared
spelling operations. Regression controls retain each independently reproduced
counterexample; the Boolean branch case runs all four inputs through the
actual Rust frontend and JavaScript emission.

`native-policy.lino` selects the established checked-program UTF-16 String
flag as an explicit **native generation input**. This experiment does not
claim that input flag is serialized as a source-free program model. The
generator also checks the exact previous String prelude. Its receipt binds
the source, policy, generator, style builder, fresh styles, complete declaration
order and generated native bytes. Every check recomputes expected styles;
stale candidate styles are rejected.

Generate and check from the root:

```sh
node js/scripts/generate-link-network-binary-rules.mjs --metadata-buffer-candidate --update-styles
node js/scripts/generate-link-network-binary-rules.mjs --metadata-buffer-candidate --check
node js/scripts/generate-link-network-binary-rules.mjs --check
```

The generated candidate lives in `native/`; production source and native
rules remain unchanged. Native compilation and byte equivalence still require
the separate CI oracle. There is no runtime improvement claim.
