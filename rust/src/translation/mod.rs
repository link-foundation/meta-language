//! Translation between JavaScript, Rust, Lean and Rocq through a checked
//! portable core.
//!
//! Each frontend reads a source program into the surface AST,
//! the checker resolves names, types and the exact operator semantics, and
//! each emitter writes a target program together with its mapping, the
//! encodings it chose and the assumptions it relies on.
//!
//! Mirrors `js/src/translation/`; both runtimes produce the same surface
//! trees, checked programs, target texts and contracts.

use serde::{Deserialize, Serialize};

pub mod aborts;
pub(crate) mod array_binding_rules;
pub(crate) mod array_callback_rules;
pub(crate) mod array_classification_rules;
pub(crate) mod array_from_rules;
pub(crate) mod array_value_rules;
pub(crate) mod ascii_text_predicate_rules;
pub(crate) mod boolean_declaration_value_rules;
pub(crate) mod borrowed_call_interface;
pub(crate) mod borrowed_call_interface_rules;
pub(crate) mod callable_emission_rules;
pub(crate) mod callable_source_rules;
pub mod check;
pub(crate) mod collection_access_rules;
pub(crate) mod collection_construction_rules;
pub mod decimal;
pub(crate) mod declaration_dependency_rules;
pub mod diagnostics;
pub(crate) mod embedded_region_rules;
pub mod emit_common;
pub mod emit_javascript;
pub mod emit_lean;
pub mod emit_rocq;
pub mod emit_rust;
pub(crate) mod external_emission_rules;
pub(crate) mod fixed_integer_cast_program;
mod fixed_integer_cast_rules;
pub(crate) mod frontend_rules;
pub(crate) mod grammar_attribute_order_rules;
pub(crate) mod grammar_lexer_rules;
pub(crate) mod grammar_rule_composition_rules;
pub(crate) mod grammar_selection_field_range_rules;
pub(crate) mod grammar_selection_included_range_rules;
pub(crate) mod grammar_selection_mapping_rules;
pub(crate) mod grammar_selection_rules;
pub(crate) mod guarded_private_array_write_rules;
pub(crate) mod inert_module_binding_rules;
pub mod ir;
pub mod javascript;
mod javascript_line_terminator_rules;
pub(crate) mod javascript_record_parameter_rules;
pub(crate) mod javascript_tag_context_rules;
pub mod js_number;
pub mod lean;
pub mod lean_root_names;
pub mod lexer;
pub(crate) mod link_network_binary_buffer_rules;
pub(crate) mod network_projection_rules;
pub(crate) mod nullable_type_syntax_rules;
pub(crate) mod number_bitwise_program;
mod number_bitwise_program_data;
pub(crate) mod number_bitwise_rules;
mod number_predicate_rules;
pub(crate) mod optional_inference_rules;
pub(crate) mod optional_value_rules;
pub mod output;
pub(crate) mod owned_array_append_rules;
pub(crate) mod owned_local_state;
pub(crate) mod parameter_default_rules;
pub(crate) mod partial_callable_rules;
pub(crate) mod present_type;
mod primitive_method_rules;
pub mod primitive_value_rules;
pub mod product_emission_rules;
pub(crate) mod program_number_projection_rules;
mod program_string_representation_rules;
pub mod proof;
pub(crate) mod record_emission_rules;
pub(crate) mod record_type_syntax_rules;
pub(crate) mod record_value_rules;
pub(crate) mod record_write_program;
pub(crate) mod record_write_rules;
pub(crate) mod reference_identity_rules;
pub(crate) mod reference_method_program;
pub(crate) mod reference_method_rules;
pub mod rocq;
pub(crate) mod runtime_shell;
pub(crate) mod runtime_shell_rules;
pub mod rust;
pub(crate) mod rust_character_rules;
mod rust_const_branch_rules;
pub(crate) mod rust_diagnostic_attribute_rules;
pub(crate) mod rust_literal_match_rules;
pub(crate) mod rust_shared_array_handle_rules;
pub(crate) mod scalar_loop_rules;
pub(crate) mod scoped_binding_emission;
pub(crate) mod scoped_binding_emission_rules;
pub(crate) mod self_translation_layout_rules;
pub(crate) mod semantic_input_rules;
pub(crate) mod source_const_functions;
pub(crate) mod source_diagnostic_attributes;
pub(crate) mod source_empty_options;
pub(crate) mod source_header_emission;
pub(crate) mod source_header_emission_rules;
pub mod source_loops;
pub(crate) mod source_primitive_owners;
pub(crate) mod source_pure_unit_calls;
pub(crate) mod source_signature_rules;
pub mod source_signatures;
pub(crate) mod source_text_byte_rules;
pub(crate) mod source_text_bytes;
pub(crate) mod source_type_namespaces;
pub(crate) mod source_unit_clone_derivations;
mod string_order_rules;
pub mod structured_type_rules;
pub mod surface;
pub mod surface_raw_metadata_rules;
pub mod surface_reference_analysis_rules;
pub(crate) mod surface_reference_facts;
pub(crate) mod type_namespace_rules;
pub(crate) mod type_syntax_rules;
pub(crate) mod typed_program_link_rules;
pub mod types;

/// A source range in UTF-16 code units, as JavaScript string offsets count.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// The four languages of the translation matrix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    JavaScript,
    Rust,
    Lean,
    Rocq,
}

impl Language {
    pub const ALL: [Self; 4] = [Self::JavaScript, Self::Rust, Self::Lean, Self::Rocq];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::JavaScript => "JavaScript",
            Self::Rust => "Rust",
            Self::Lean => "Lean",
            Self::Rocq => "Rocq",
        }
    }

    #[must_use]
    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|language| language.as_str() == name)
    }
}
