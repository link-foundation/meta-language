---
bump: patch
---

### Added

- JavaScript async functions (declared by name, exported, or bound to a top-level constant as an arrow or function expression) and `await`, in functions and at the top level of a module, translate in both runtimes when every call of an async function is awaited where it is made, or returned from another async function. Nothing then runs concurrently, so each await is the ordinary call of its function; `@returns {Promise<T>}` declares a `T` result, and every target records the `sequential-async` encoding. A call whose Promise the program could observe, and `await` outside an async function, are rejected with a diagnostic at the call. A corpus program of async functions runs natively in Rust, Lean, Rocq and JavaScript with Node's output.
