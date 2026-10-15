//! Compile the actual generated pure decisions inside the integration test crate.
//! The module path preserves their existing self-qualified references without
//! making the runtime's private frontend module public.

#[path = "../../src/translation/frontend_rules.rs"]
pub mod frontend_rules;

#[path = "../../src/translation/rust/owned_local_state_rules.rs"]
pub mod owned_local_state_rules;

#[path = "../../src/translation/rust/owned_tuple_call_rules.rs"]
pub mod owned_tuple_call_rules;

#[path = "../../src/translation/source_text_byte_rules.rs"]
pub mod source_text_byte_rules;

pub mod rust {
    pub use super::owned_local_state_rules;
    pub use super::owned_tuple_call_rules;
}
