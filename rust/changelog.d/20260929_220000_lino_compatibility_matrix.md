---
bump: minor
---

### Added

- A Links Notation compatibility matrix, `parity/fixtures/lino-compatibility-matrix.json`, in both runtimes. It covers named and anonymous links, arity, shared, recursive and forward references, identity, ordering, indentation, nested multiline groups, quoting, escaping, comments and trivia, Unicode and source mappings. Every feature is:
  - decoded by the official parser and by meta-language's own reading alike;
  - reconstructed byte for byte;
  - edited through the network;
  - encoded back into text that both parsers read the same way.
- `LinkNetwork::links_notation_reading` / `linksNotationReading` returns the official reading of one parsed link, and `links_notation_text` / `linksNotationText` writes links back as Links Notation text.
- `docs/downstream-consumers.md` maps what relative-meta-logic and link-assistant/formal-ai use and require to meta-language capabilities, ledger rows and tests. `npm run check:downstream-consumers` fails when a mapped row or test does not exist.

### Changed

- The link-semantics reader in both runtimes now follows the LiNo CST. `#` comments, blank lines and nested groups are read the way the official 0.22 parser reads them.
- The Links Notation scaling guards read their regression inputs, including the RML upstream regressions, from the compatibility matrix.
