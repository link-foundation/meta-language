Rust decimal integer tokens now accept separators immediately before a complete
supported integer suffix, preserving exact integer text, raw source and spans.
One Links policy generates the JavaScript and native lexer decision. Existing
unsuffixed trailing-separator refusals and JavaScript numeric rules are unchanged.
Non-ASCII suffix boundaries, including identifier continuations and Unicode
whitespace, retain their original refusal pending Unicode boundary support.
