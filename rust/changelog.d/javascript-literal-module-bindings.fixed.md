Native JavaScript parsing retains admitted immutable module literals, including
`const NaN = 3`, nulls and nested literal records, when sibling functions read
them. Both frontends use the same generated literal-admission decision; spreads,
effectful initializers and mutable module captures keep their existing refusals.
