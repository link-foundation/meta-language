Preserve JavaScript `Array.from` string code points and mapper callback types,
argument order and observations through the generated shared iterable and map
rules, including source-free program rechecking. Array and string receivers reuse
the existing value model; this does not add native Set or Map constructors.
