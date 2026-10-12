Preserve valid Rust `String` equality and inequality against `&str` during
translation and edited self-translation. Source origins retain the distinction
between an owned string, a string slice and a reference to an owned string, so
the comparison rule does not authorize unrelated pointer projections. Shared
JavaScript admission rules generate the native decision through Links Notation
decorators; existing self-translation fixture expectations remain unchanged.
