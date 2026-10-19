Add two explicit source models for owned standard `String` local values:
`closed owned text local state 1` captures a once-evaluated `(String, i32)`
initialization, ordered moves, replacements and conditional joins;
`canonical owned text local copies 1` retains actual standard String clones in
emitted definition replay. Owned interface and occurrence facts survive editable
Links reload and are rechecked before emission. Both models retain their
selected input-domain checks and exclude allocation failures, pointer identity,
release timing, custom destruction and ABI observations.

The JavaScript public `rustOwnedLocalStateModel` option and native
`create_typed_program_links_with_rust_owned_local_state_model` factory retain the
selection separately from caller provenance. The optional Rust surface facet is
boxed and the checked record stores its transparent value. Existing source
profiles keep their original defaults. Actual generated runtime wrappers and the
complete binary formatter still require separate source interpretations. Native
compilation, execution and runtime parity remain CI verification obligations.
