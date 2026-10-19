Ruby's pinned native grammar now preserves newline-separated strings,
adjacent range and minus operators, block keyword boundaries, namespace
constants, and explicitly invoked constant methods through recorded source
transformations. Shared scanner policies preserve Unicode and operator symbols,
escaped line continuations, and whitespace-sensitive splat decisions within
the existing parser budgets. Both runtimes consume the generated Links data.
