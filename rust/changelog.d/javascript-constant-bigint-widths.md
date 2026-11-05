Draft constant JavaScript BigInt.asIntN/asUintN widths from0 through4096 after
ToIndex, using the existing modular construction plan in both frontends.
Retain value evaluation once, zero width, fractional truncation, negative zero
and NaN behavior. Rust primitive integer types remain independently bounded.
Capture ignored arguments in order and retain their abrupt completion.
Independent source and native frontend controls await execution.
