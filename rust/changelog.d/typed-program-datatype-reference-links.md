Typed program Links retain independent datatype and constructor declaration
references through source-free serialization and reload. Constructor and type
renames follow owned references; lexical pattern binders retain their scoped
identities. Reload checks declaration order, constructor ownership and argument
types, duplicate field aliases, conflicting semantic roles, and traversal bounds.
