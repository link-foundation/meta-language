---
bump: patch
---
Tokenize template substitutions with the normal lexical rules so braces in
strings, comments, regex literals and nested templates preserve their boundaries.
Reuse the source encoding across substitutions, retain absolute UTF-16 offsets,
and test lexical recovery and emitted program behavior in both runtimes.
