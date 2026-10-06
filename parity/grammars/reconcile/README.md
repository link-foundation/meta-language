# Real grammar pairs for merge reconciliation

[`pairs.json`](pairs.json) lists the language grammar pairs the merge's
`reconcile` mode is checked on. Each pair is a shipped native grammar from
[`../native`](../native) and a grammars-v4 ANTLR grammar vendored here
unchanged from
[antlr/grammars-v4](https://github.com/antlr/grammars-v4) at revision
`7df52be94698550d219d299d04105c6bafadd9c3`, the revision
`parity/grammars-v4-sources.json` pins for the bulk pipeline:

- `grammars-v4-7df52be-JSON.g4` is `json/JSON.g4`. Its header says it is taken
  from "The Definitive ANTLR 4 Reference" by Terence Parr and derived from
  https://json.org.
- `grammars-v4-7df52be-CSV.g4` is `csv/CSV.g4`, under the BSD licence its
  header states (Copyright (c) 2013 Terence Parr).
