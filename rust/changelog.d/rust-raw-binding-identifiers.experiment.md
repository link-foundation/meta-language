Added an explicit JavaScript experiment for raw Rust function and parameter
identifiers in complete primitive modules. It retains original source bytes,
actual binding identities, checked invocation premises, saved Links edits and
automatically translated pure selectors. An explicit invocation checker requires
the actual declaration, its exact parameter array and Boolean or signed i32
arguments. A CI-only source oracle compares seven original modules and three
coherent edits using the same compiler/byte-receipt runner as the text oracle.
Production selection and native runtime integration remain separate work.
