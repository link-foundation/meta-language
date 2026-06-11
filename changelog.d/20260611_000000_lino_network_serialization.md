---
bump: minor
---

### Added
- `LinkNetwork::to_lino` serializes an entire links network to canonical
  links-notation text, keyed by each link's numeric id (doublets-style id
  discipline), covering references, names, types, terms, definitions,
  languages, source spans, parse flags, and term registration.
- `LinkNetwork::from_lino` reconstructs the exact network from that text,
  forming a lossless round-trip (`from_lino(to_lino(n))` is isomorphic to `n`).
- `LinoSerializationError` reports parse and schema failures from `from_lino`.
- A round-trip property test over every language fixture plus synthetic
  networks, and a test that `to_lino` output is accepted by the
  `links-notation` 0.13 crate parser.
- `ParityCapability::LinoSerialization` and an output-side serialization
  parity fixture for the `links-notation` target.

### Changed
- Added the `links-notation` 0.13 crate as a dependency so serialized output
  aligns with the wider links-notation ecosystem.
