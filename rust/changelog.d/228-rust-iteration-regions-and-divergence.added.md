Draft Rust source-region handling for iteration-local storage, conditional
panic paths and Boolean short-circuiting. Finite bodies close their local scopes
before the next iteration, and aborting alternatives supply no normal ownership
or return state. A wholly aborting owned fold retains its zero-iteration path.
JavaScript source and independent controls are drafted; automatic Rust policy
translation and all execution checks remain deferred to the complete draft.
