Add the explicit `closed guarded function returns 1` Rust source profile for
terminal value returns and closed conditional guards. It lowers to the existing
typed let/if core, preserves enclosing continuation scopes and source spans,
and stores selected source models separately from caller provenance in links.
The default early-return refusals remain unchanged. Escaping returns, partially
returning branches, unreachable statements, mutable loops and destructor or
lifetime effects still require their own source-admission proofs.

This selected profile also reimports the emitter's exact
`panic!("{}", literalString)` form. The second argument remains literal message
data, including braces and escapes; dynamic Display formatting is still refused
and the default profile's formatted-panic diagnostics stay unchanged.
