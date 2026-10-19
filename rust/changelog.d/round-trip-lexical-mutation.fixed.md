Extend source-scoped lexer declarations when round-trip mutation adds a parser
literal. Preserve original explicit-token order, reuse existing literal aliases,
and renumber later implicit tokens to match re-import of the edited source.
Both adapters use the shared JavaScript extension plan; native checks run in CI.
Lossless renamed-source comparison checks a bijection from original parser-owner
aliases to their actual public references. Actual renamed Links retain those
aliases; token, literal, action and every other clause remain exact.
