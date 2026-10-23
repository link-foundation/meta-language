Add a second standard Rust shared-array source model for explicit
`std::rc::Rc::clone(&value)` calls, including nested clones. Shared JavaScript
decisions generate the native rules. Preserve the first source model and all
earlier fixtures; add Boolean, numeric and Unicode-string parity cases.
