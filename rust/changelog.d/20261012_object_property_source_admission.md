The JavaScript translation frontend preserves literal computed and shorthand
`__proto__` data properties. Tagged object parsing retains prototype-definition
expressions explicitly; portable checking reports a precise unsupported
diagnostic until object prototype semantics are represented. Unmodeled runtime
`Object.freeze` operands are refused instead of erasing the call. Self-translation
continues to represent literal readonly module arrays as immutable Rust constants.
