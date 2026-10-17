# Agda layout development oracle

This copy of tree-sitter-agda 1.3.3 comes from upstream revision
`47802091de0cb8ac2533d67ac37e65692c5902c4`. The generated parser and grammar
are unchanged. `source-manifest.json` pins every build input and both original
and patched scanner hashes; `src/scanner.c.upstream.gz` and `layout-state.patch`
reproduce the correction. The upstream MIT license is in `LICENSE`.

The correction relocates a circular queue when its modulus grows, serializes
every queued token and the complete pending dedent count, and clears that queue
when restoring scanner state. Those are scanner-state corrections, without a
change to any grammar production. An independent test oracle uses this scanner
to check the complete upstream corpus. The original released oracle remains
available and retains the existing recovery and divergence expectations.

The C parser is compiled only in development tests. JavaScript CI builds its
WebAssembly oracle with `js/scripts/build-agda-layout-oracle.mjs`. Neither
runtime's production parsing uses this parser.
