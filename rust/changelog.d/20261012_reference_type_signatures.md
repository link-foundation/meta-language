Both runtimes preserve shared reference identities in array, record, map and set
type keys, and represent mutable binding cells as a separate recursive type.
Rust target signatures use the automatically translated shared type rules to
render these identities with `Rc<RefCell<...>>`. Nullable data retains conditional
total equality; nested floating fields omit `Eq`, and callable data fields report
an explicit unsupported diagnostic. This does not add checked state operations
to the Rust runtime frontend.
