Added shared structural type identities and Rust signature emission for
optional values, tuples, records, read-only maps and sets, and callable values.
The portable identity, absence and signature decisions are translated
automatically from JavaScript into the Rust runtime. Shared fixtures compare
canonical type identities and complete emitted identity functions. This
change does not declare support for constructing or operating on those values
in frontends whose checked expression variants are still unsupported.
