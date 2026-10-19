Add source-local ANTLR lexer declarations for ordered maximal-munch election,
mode commands, retained channel trivia and explicit unresolved action obligations.
Preserve original token expectations when raw grammar concepts share a canonical
name. Native adapters use the complete self-translated JavaScript policies and
the existing parser limits; native compilation and parity checks run in CI.

Correct the former case-insensitive fixture's assumption that an unavailable
`superClass = Base` behaves like the standard parser. The exact original fixture,
rule shapes and documentation remain checked, with its original option bytes
retained and execution explicitly refused. A case-insensitive-only copy keeps
both original case acceptance/rejection checks. ANTLR 4.13.2
[defines this option as the generated parser superclass in combined grammars](https://github.com/antlr/antlr4/blob/4.13.2/doc/options.md#superclass);
equal lexical bodies do not establish the missing superclass's behavior.
