Rust source admission now requires an actual root `extern crate alloc;`
declaration before projecting allocation-crate Box values. Standard-library
Box imports and renamed Box imports resolve their actual owner while retaining
source ownership checks. The shared source fixtures cover declared and absent
allocation crates, aliases, duplicate declarations and shadowed owners.
