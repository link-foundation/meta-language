Draft scalar Vec indexing alongside readonly Rust source regions. Scalar copies
and shared element references retain distinct source ownership and lifetime
behavior, with existing usize and bounds checks. Five additional originals and
four refusal controls are authored; these paths remain unverified.
