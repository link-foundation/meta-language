Draft complete multi-language concrete source snapshots using the existing
graph archive. Both packages preserve original graph/cache/allocator state,
support source-free text/binary reload and explicit AST views, and append new
sources without reparsing old graphs. Rust semantic project packets reuse this
storage while retaining the preceding layout reader. Controls remain deferred.
