Add an opt-in standard Rust binary64 owner model for complete flat primitive
functions. Qualified `std::primitive::f64` and `core::primitive::f64` owners and
unique explicit aliases retain their actual paths, import order and constant
sites as inspectable Links facts. Source-free reload checks those facts against
the typed interface and body. Default source admission remains unchanged.
