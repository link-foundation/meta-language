Draft ordinary standard scalar-vector `clone`, `to_owned` and `to_vec`, with
actual receiver and Copy-element checks. Copies use shared full-slice meanings
across Rust, JavaScript, Lean and Rocq. Rust copies without floating-point length
conversion; JavaScript keeps independent arrays. General reference-clone
resolution, non-Copy ownership and aggregate ABI support remain unfinished.
The public typed-emission declaration now lists all four supported targets.
