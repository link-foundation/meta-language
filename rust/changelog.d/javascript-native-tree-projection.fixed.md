Project native JavaScript parse trees after one trivia-normalization pass and
cache derived node spans within the current projection. Bounded controls match
the complete frozen projected values, and tiny Rust, JavaScript and JSON parses
retain their full projected trees. Full-source resource fit and native behavior
remain CI checks.
