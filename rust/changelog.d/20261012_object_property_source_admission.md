The JavaScript translation frontend preserves literal computed and shorthand
`__proto__` data properties. It reports a precise unsupported diagnostic for
uncomputed `__proto__:` prototype definitions, whose prototype behavior is not
represented by the portable core.
