Draft actual Rust constant values independently of function items, preserving
checked constant reads, callable pointer values, source owners and local shadows.
Report unrepresented constant patterns instead of treating them as bindings.
Keep ordinary module values in their actual namespace and retain zero-argument
applications independently of function-value expectations.
