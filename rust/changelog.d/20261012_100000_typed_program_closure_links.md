Source-free typed program links now preserve the initializer type of lexical
bindings when their enclosing expression returns a different type. Escaping
closures retain shared mutable cells after links serialization, and invalid
cell, lambda, and application type edits are rejected before target emission.
