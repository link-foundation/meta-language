Add shared Rust cooked-string continuation and CRLF value normalization decisions
to the JavaScript and native Rust frontends, retaining original source spans and
bytes. Rust and JavaScript keep their distinct continuation whitespace rules.
Automatically converted Links Notation fixtures cover semantic reload, executable
JavaScript output and canonical target literal encodings; native parity runs in
CI. Full emitted Rust runtime-wrapper reimport remains a separate frontend gap.
