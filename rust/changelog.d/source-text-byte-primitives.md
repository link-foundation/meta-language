Add paired source-text primitives for reading UTF-16 units, exact UTF-8 length
and bytes, and unchanged slices at validated UTF-8 byte boundaries. Their shared
numeric algorithms are translated automatically from JavaScript and use the
linked Unicode scalar model, bounded to 4,294,967,295 UTF-16 units.

JavaScript rejects nonprimitive strings and unpaired surrogates. Rust accepts
`&str` and reports model-bound errors through `Result`. Invalid numeric offsets
produce `null` or `None`; slicing additionally refuses endpoints inside a
scalar's UTF-8 encoding.
This component does not admit the complete standard-library number formatter
or add Rust slice-syntax translation.
