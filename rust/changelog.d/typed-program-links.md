JavaScript and Rust portable-core programs can be encoded as source-free typed
links, serialized, edited and emitted in either target. The graph shares its
schema and generated classifiers with JavaScript. Declaration and lexical binding
identities survive reload; scalar, scope and structural edits are validated before
the existing checker and emitters run. Unsupported Rust IR forms retain explicit
adapter diagnostics instead of falling back to a saved source envelope.
Repeated paths through shared graphs have a traversal budget, and the shared
schema admits only supported machine integer widths and optional absence policies
before the Rust checker or emitter reads edited types.
