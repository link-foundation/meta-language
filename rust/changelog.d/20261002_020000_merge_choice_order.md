---
bump: patch
---

### Fixed
- `merge_grammars` (JavaScript `mergeGrammars`) now keeps the alternatives of an unordered choice in their source order. Before, the merged grammar sorted them. A parser that commits to the first matching alternative, such as the peggy parser behind JavaScript `parseWithGrammar`, then rejected input the source grammar accepted: `word ::= letter word | letter` became `letter | letter word` and stopped after one letter. The order-free form is still used to compare rules, so merge decisions and fingerprints are unchanged.
