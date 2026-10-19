CI checks complete self-translation report artifacts in a separate JavaScript
test group before Rust and delivery stages. It verifies the producing commit,
current JavaScript and Rust source hashes, and the existing coverage and
difference assertions without repeating module dependency translation in the
ordinary JavaScript test group. Standalone JavaScript workflow runs produce
and check their own report shards.

Report counterpart lookup also recognizes actual Rust grammar host files under
`grammar/feature_runtime` for JavaScript `grammar-runtime` modules.
