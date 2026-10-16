Keep source lexer bindings when independently replaying Peggy's optimized
single-character alternatives. Check exact finite-set coverage before lowering
the optimized class, preserving Unicode byte spans and rejection offsets while
refusing unbound members and ambiguous source ownership.

Extend the same shared literal pool when a target AST changes a parser literal,
retaining original source clauses. Independent mutation witnesses require actual
syntax rejection; operation failures retain their diagnostic causes separately
from syntax and resource rejections.
