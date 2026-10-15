Refresh only the parse diagnostic for the unchanged
`frontend-rust/frontend-rust-stringEscapes` source. Its invalid `\{` escape
still produces a syntax error, now at the original escape span `84..86`
before format-string validation. The [Rust Reference's cooked-string
grammar](https://doc.rust-lang.org/reference/tokens.html#string-literals)
excludes this escape. All program descriptors and other generated stage
records remain unchanged; focused tests retain the original source hash,
lexical refusal, valid neighboring escapes and refresh conservation guards.
