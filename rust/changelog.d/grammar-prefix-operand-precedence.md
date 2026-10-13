Native grammar precedence now requires the actual first child's source rule or
reduction before using an operand suffix's precedence. Anonymous source prefixes
retain their own precedence, preserving the pinned Rust CST fields and aliases
for omitted range endpoints followed by dereference or negation syntax.
