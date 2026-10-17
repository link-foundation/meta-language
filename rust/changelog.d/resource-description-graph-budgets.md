### Fixed

- Count imported terms, ordered occurrences, archived merge sources, equality
  witnesses and final graph roots against the existing 100,000-node output
  budget. Imports and merges refuse oversized results before allocating beyond
  that cap, so accepted graphs fit the default source-free decoder and witness
  verifier. Existing triple, source-count, lexical and traversal caps remain.
