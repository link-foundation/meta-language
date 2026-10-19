Rust source translation now admits standard primitive `f64` inclusive range
membership and immutable standard `str`/`String` emptiness queries. Range
operands evaluate once in source order; NaN and signed zero retain primitive
comparison behavior. The source owner and immutable pointer layers are checked
before projection, including standard String aliases and `String::new()`.
