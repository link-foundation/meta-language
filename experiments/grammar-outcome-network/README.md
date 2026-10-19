# Owned grammar outcome transport experiment

This JavaScript-first experiment exercises the public parser registry with a
source-free global grammar archive. It addresses the public CST transport gap
described in [the authoritative vision](../../docs/vision.md#current-state).

Four complete shipped JSON grammars are composed through existing declarative
field-range and JSON-string mappings. After the original grammar maps are
discarded, the archived parser produces one complete outcome with four ordered
source planes. The default JSON CST owns the host tokens. An explicit Syntax
association attaches that owner, an ordered physical-token manifest, the
structured-data root, and every member of the closed outcome graph.

The reader isolates that graph, uses the unchanged structured-data decoder,
rechecks the actual mapping derivations, and binds the physical CST bytes to
the manifest's tokens. Each token identity must also belong to the actual
owner's Syntax/Document containment graph, under the codec's node bound;
equal bytes from an unrelated owner do not establish ownership. Physical
coordinates start at zero for this complete-root profile. Decoded-plane leaves remain inspectable data; they never
enter global source-token ordering. Tests check complete text/binary replay,
source reconstruction, missing and duplicate members, foreign links, altered
source bytes and derivations, and refusal before mutation or accessor execution.
Accepted and rejected ambiguous parses retain their complete trees,
alternatives, ambiguities, rejection details and decoded source planes.
Outcomes without a concrete tree remain outside this finite experiment.
Physical extraction uses one bounded iterative traversal and one final byte
concatenation, avoiding a full source copy at each nested node. The codec's
node and byte limits also bound this traversal and physical buffers. The
reader refuses an oversized payload manifest before cloning its members;
the existing decoder still checks the complete expanded value, including
repeated strings, before mapping validation or physical extraction.
Ordinary UTF-8 outcomes keep their absent `sourcePlanes` field absent. The
reader derives a root-only checker envelope from the actual owned bytes;
mapped outcomes still require every declared source plane and derivation.

`concreteLinksWithOwnedGrammarOutcomes` is an experimental projection. The
tests explicitly retain the fact that the public default projection still
omits the Semantic payload. No catalog default, runtime export, public
projection policy or Rust runtime changes here. Production integration still
requires a shared projection policy, native transport and the public default
parser route; this experiment does not establish those capabilities or full
language coverage. Mapping verification checks source correspondence, while
the actual archived grammar parser supplies acceptance.

Run the bounded JavaScript controls through
`js/tests/grammar-outcome-network-experiment.test.js`. Rust compilation and
full language workloads remain CI-only.
