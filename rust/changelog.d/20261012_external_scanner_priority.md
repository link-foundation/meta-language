Requested external scanner tokens keep their content ahead of competing
internal extras, including comment-shaped interpolation text and explicit
scanner padding. Zero-width layout tokens retain following extras. Composite
extra productions now parse their internal separators as syntax, and native
oracle projections retain the actual extra flag carried by syntax nodes.
