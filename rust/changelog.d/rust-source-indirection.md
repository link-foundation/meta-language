Rust source admission now retains immutable reference and standard `Box` origin
evidence until it has checked dereferences, constructor arguments, declared
initializers and function results. Forward declarations, module imports and
shadowed lexical bindings use the existing source parser's scopes. Scalar,
unknown, mutable and shadowed Box origins produce source diagnostics instead of
being silently erased into values.

The current value model still does not implement complete Rust borrow, lifetime,
trait, `Clone` or `Drop` checking. The shared regression sources exercise actual
original Rust compilation in CI, including boxed recursion and derived cloning.
