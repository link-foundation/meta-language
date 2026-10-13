//! Compile the actual generated pure decisions inside the integration test crate.
//! The module path preserves their existing self-qualified references without
//! making the runtime's private frontend module public.

#[path = "../../src/translation/frontend_rules.rs"]
pub mod frontend_rules;
