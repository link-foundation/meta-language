Draft immediate Rust copied/cloned scalar `all`, `any` and `fold` translation
through shared array operations. Preserve callback source signatures, short
circuiting, fold initialization order and actual vector element owners.
Controls remain deferred until the complete requirements draft is ready.

Also draft immediate borrowed scalar predicates and folds, preserving actual
immutable reference parameter signatures and explicit dereferences. Keep
mutable, nested and unproved borrowing outside this admission.
