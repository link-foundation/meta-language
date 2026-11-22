//! Source ownership evidence retained while Rust indirection projects to values.
//!
//! Mirrors `js/src/translation/rust-source-origins.js`. Declarations and lexical
//! scopes come from the existing parser; value typing remains in the checker.

use std::collections::{HashMap, HashSet, VecDeque};

use super::array_rules::{
    accept_rust_fresh_array_result_tail, accept_rust_source_array_coercion,
    accept_rust_source_array_element, accept_rust_source_array_owner,
};
use super::indirection_rules::{
    accept_rust_immutable_indirection_layer, accept_rust_source_box_availability,
    accept_rust_source_box_coercion_layer, accept_rust_source_box_owner,
    accept_rust_source_indirection_argument, accept_rust_source_text_equality,
    read_rust_source_expression_origin,
};
use super::product_rules::{accept_rust_immutable_product_type, accept_rust_product_origin};
use super::{Result, SExpr, SNode, Span, Type, unsupported};
use crate::translation::rust_array_index_rules::accept_rust_scalar_array_index_element;
use crate::translation::rust_byte_string_expression_rules::ml_string::Text as ByteStringText;
use crate::translation::rust_fresh_vector_argument_rules::{
    accept_rust_fresh_owned_vector_argument, read_rust_fresh_vector_argument_form,
};
use crate::translation::rust_fresh_vector_result_rules::accept_rust_known_fresh_vector_call;
use crate::translation::rust_pointer_integer_owner_rules::{
    accept_rust_pointer_integer_owner_child, read_rust_pointer_integer_owner_error,
    read_rust_pointer_integer_owner_join_error, select_rust_pointer_integer_result_owner,
};

mod callable_owners;
mod mutable_record_loans;
mod nominal_copies;
mod owned_bindings;
mod patterns;
mod primitive_expressions;
mod type_owners;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SourceOrigin {
    pub(super) array_element_position:
        Option<Box<crate::translation::source_signatures::SourceTypeSignature>>,
    pub(super) source_function_pointer_header: bool,
    pub(super) source_fn_mut_parameter: bool,
    /// Actual concrete standard sum source children, retained independently of
    /// the generated datatype spelling and re-resolved against the catalog.
    pub(super) standard_sum_source: Option<Box<super::standard_sum_source::StandardSumSource>>,
    pub(super) source_binding_identity: Option<String>,
    pub(super) source_lexical_binding_identity: Option<String>,
    pub(super) source_record_state_collecting_site: Option<Span>,
    pub(super) source_value_borrow_bindings: Vec<String>,
    pub(super) source_value_field_borrow_bindings: Option<Vec<Vec<String>>>,
    pub(super) source_box_layers: Vec<super::owned_containers::SourceBoxLayer>,
    pub(super) source_place_binding_identity: Option<String>,
    pub(super) source_loan_binding_identities: Vec<String>,
    pub(super) source_place_loan_binding_identities: Vec<String>,
    pub(super) source_owned_result_proof: bool,
    pub(super) source_standard_sum_pattern_move: bool,
    pub(super) source_standard_sum_borrowed_field: bool,
    pub(super) source_nominal_pattern_move: bool,
    pub(super) source_nominal_borrowed_field: bool,
    pub(super) source_byte_iterator: bool,
    pub(super) source_borrowed_place: bool,
    pub(super) source_extending_temporary_bindings: Vec<String>,
    pub(super) ty: Option<Type>,
    /// Explicit primitive f64 annotation or literal evidence, never fallback inference.
    pub(super) float_source_model: Option<&'static str>,
    pub(super) path: Vec<String>,
    pub(super) wrappers: Vec<String>,
    pub(super) proven: bool,
    pub(super) diverging: bool,
    pub(super) uncertain: bool,
    /// Source Copy evidence does not choose the checker's numeric width.
    pub(super) copy_evidence: Option<()>,
    pub(super) fields: Option<Vec<Self>>,
    pub(super) text_source_model: Option<&'static str>,
    pub(super) array_source_model: Option<&'static str>,
    /// Element Copy facts are independent of vector ownership or freshness.
    pub(super) array_element_copy_evidence: bool,
    /// Constructor or actual call source evidence; cleared on local binding.
    pub(super) fresh_array_value: Option<Span>,
    /// Closed source result postcondition; all declaration bodies must discharge it.
    pub(super) fresh_vector_result_contract: bool,
    pub(super) copy_closure_identity: Option<Span>,
    pub(super) copy_closure_capture_count: Option<usize>,
    pub(super) callable_parameters: Option<Vec<Self>>,
    pub(super) callable_result: Option<Box<Self>>,
    pub(super) callable_source_owner_error: Option<String>,
    /// Spelled source lifetimes, in reference-layer order; None is elision.
    pub(super) source_lifetimes: Vec<Option<String>>,
    pub(super) source_owner: Option<super::super::source_signatures::SourceOwner>,
    /// Checked transparent alias owner, before retaining the source spelling.
    pub(super) expanded_source_owner: Option<super::super::source_signatures::SourceOwner>,
    pub(super) integer_source_owner_conflict: bool,
    pub(super) integer_source_owner_constraint: Option<String>,
    pub(super) primitive_owner_position: Option<serde_json::Value>,
    pub(super) namespace_spelling: Vec<String>,
}

impl SourceOrigin {
    /// Actual immutable scalar-reference parameter facts; source wrappers survive.
    pub(super) fn copy_closure_parameter(&self) -> bool {
        let owner = self
            .expanded_source_owner
            .as_ref()
            .or(self.source_owner.as_ref());
        let primitive = if let (Some(owner), Some(ty)) = (owner, self.ty.as_ref()) {
            let (bits, signed) = match ty {
                Type::Fixed { bits, signed } => (bits.to_string(), *signed),
                _ => (String::new(), false),
            };
            owner.arguments.is_empty() && (owner.kind == "unit" && owner.path.is_empty() && ty.kind() == "unit" || owner.kind == "primitive" && crate::translation::source_signature_rules::accept_rust_source_signature_primitive(ty.kind(), &owner.path.join("::"), &bits, signed))
        } else {
            false
        };
        let mut base = self.clone();
        base.wrappers.clear();
        crate::translation::rust_copy_closure_parameter_rules::accept_rust_copy_closure_parameter_source(self.proven,base.direct_copy_value(),self.wrappers.len() as f64,self.wrappers.iter().all(|value| value == "reference"),primitive)
    }

    /// Standard readonly view from actual static u8 array facts; no owner mutation.
    pub(super) fn static_byte_array_read_view(&self) -> Option<Self> {
        let element = match &self.ty {
            Some(Type::Array { element, .. }) => Some(element.as_ref()),
            _ => None,
        };
        let (bits, signed) = match element {
            Some(Type::Fixed { bits, signed }) => (*bits, *signed),
            _ => (0, false),
        };
        let owner = self.source_owner.as_ref();
        let child = owner.and_then(|owner| owner.arguments.first());
        if !crate::translation::rust_byte_string_expression_rules::accept_rust_static_byte_array_read_view(&ByteStringText::from_text(self.array_source_model.unwrap_or("")),&ByteStringText::from_text(self.ty.as_ref().map_or("",Type::kind)),&ByteStringText::from_text(element.map_or("",Type::kind)),f64::from(bits),signed,self.proven,self.wrappers.len() as f64,self.wrappers.iter().all(|layer|layer=="reference"),&ByteStringText::from_text(owner.map_or("",|owner|owner.kind.as_str())),owner.map_or(0,|owner|owner.arguments.len()) as f64,&ByteStringText::from_text(child.map_or("",|child|child.kind.as_str())),&ByteStringText::from_text(&child.map(|child|child.path.join("::")).unwrap_or_default())){return None;}
        let mut view = self.clone();
        view.wrappers = vec!["reference".to_owned()];
        view.source_lifetimes = vec![Some("static".to_owned())];
        view.array_source_model = Some("immutable slice");
        view.source_owner = Some(super::super::source_signatures::SourceOwner {
            kind: "slice".to_owned(),
            path: Vec::new(),
            arguments: vec![child?.clone()],
        });
        view.expanded_source_owner = None;
        Some(view)
    }

    pub(super) const fn unknown() -> Self {
        Self {
            array_element_position: None,
            source_function_pointer_header: false,
            source_fn_mut_parameter: false,
            standard_sum_source: None,
            ty: None,
            float_source_model: None,
            path: Vec::new(),
            wrappers: Vec::new(),
            proven: false,
            diverging: false,
            uncertain: false,
            copy_evidence: None,
            fields: None,
            text_source_model: None,
            array_source_model: None,
            array_element_copy_evidence: false,
            source_lifetimes: Vec::new(),
            source_binding_identity: None,
            source_lexical_binding_identity: None,
            source_record_state_collecting_site: None,
            source_value_borrow_bindings: Vec::new(),
            source_value_field_borrow_bindings: None,
            source_box_layers: Vec::new(),
            source_place_binding_identity: None,
            source_loan_binding_identities: Vec::new(),
            source_place_loan_binding_identities: Vec::new(),
            source_owned_result_proof: false,
            source_standard_sum_pattern_move: false,
            source_standard_sum_borrowed_field: false,
            source_nominal_pattern_move: false,
            source_nominal_borrowed_field: false,
            source_byte_iterator: false,
            source_borrowed_place: false,
            source_extending_temporary_bindings: Vec::new(),
            source_owner: None,
            expanded_source_owner: None,
            integer_source_owner_conflict: false,
            integer_source_owner_constraint: None,
            primitive_owner_position: None,
            namespace_spelling: Vec::new(),
            fresh_array_value: None,
            fresh_vector_result_contract: false,
            copy_closure_identity: None,
            copy_closure_capture_count: None,
            callable_parameters: None,
            callable_result: None,
            callable_source_owner_error: None,
        }
    }

    pub(super) fn with_type(ty: Type, path: &[String], wrappers: Vec<String>) -> Self {
        let fields = match &ty {
            Type::Tuple { elements } => Some(
                elements
                    .iter()
                    .map(|element| Self::with_type(element.clone(), path, Vec::new()))
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        };
        let children_accepted = fields
            .as_ref()
            .is_none_or(|fields| fields.iter().all(Self::direct_copy_value));
        let copy_evidence = accept_rust_immutable_product_type(ty.kind(), children_accepted, false);
        let text_source_model = (ty == super::STRING).then_some("owned string");
        let array_source_model = matches!(ty, Type::Array { .. }).then_some("owned vector");
        let array_element_copy_evidence = matches!(&ty, Type::Array { element, .. } if accept_rust_scalar_array_index_element(element.kind()));
        Self {
            source_function_pointer_header: false,
            source_fn_mut_parameter: false,
            standard_sum_source: None,
            float_source_model: (ty == super::FLOAT).then_some("binary64 scalar"),
            ty: Some(ty),
            path: path.to_vec(),
            wrappers,
            proven: true,
            diverging: false,
            uncertain: false,
            copy_evidence: copy_evidence.then_some(()),
            fields,
            text_source_model,
            array_source_model,
            array_element_copy_evidence,
            source_lifetimes: Vec::new(),
            source_binding_identity: None,
            source_lexical_binding_identity: None,
            source_record_state_collecting_site: None,
            source_value_borrow_bindings: Vec::new(),
            source_value_field_borrow_bindings: None,
            source_box_layers: Vec::new(),
            source_place_binding_identity: None,
            source_loan_binding_identities: Vec::new(),
            source_place_loan_binding_identities: Vec::new(),
            source_owned_result_proof: false,
            source_standard_sum_pattern_move: false,
            source_standard_sum_borrowed_field: false,
            source_nominal_pattern_move: false,
            source_nominal_borrowed_field: false,
            source_byte_iterator: false,
            source_borrowed_place: false,
            source_extending_temporary_bindings: Vec::new(),
            source_owner: None,
            expanded_source_owner: None,
            integer_source_owner_conflict: false,
            integer_source_owner_constraint: None,
            primitive_owner_position: None,
            namespace_spelling: Vec::new(),
            fresh_array_value: None,
            fresh_vector_result_contract: false,
            copy_closure_identity: None,
            copy_closure_capture_count: None,
            callable_parameters: None,
            callable_result: None,
            callable_source_owner_error: None,
        }
    }

    pub(super) fn value() -> Self {
        Self {
            proven: true,
            ..Self::unknown()
        }
    }
    pub(super) fn pointer_integer_owner(&self) -> String {
        let mut owner = self.source_owner.as_ref();
        for _ in 0..128 {
            let Some(current) = owner else {
                return self
                    .integer_source_owner_constraint
                    .clone()
                    .unwrap_or_default();
            };
            if current.kind == "primitive" {
                return current.path.join("::");
            }
            if !accept_rust_pointer_integer_owner_child(
                &current.kind,
                &current.path.join("::"),
                serde_json::Number::from(current.arguments.len())
                    .as_f64()
                    .expect("actual source owner argument count"),
            ) && !crate::translation::rust_owned_recursive_datatype_rules::accept_rust_owned_box_pointer_owner_child(
                &current.kind,
                &current.path.join("::"),
                current.arguments.len() as f64,
            ) {
                return String::new();
            }
            owner = current.arguments.first();
        }
        String::new()
    }
    fn integer_conflict() -> Self {
        Self {
            integer_source_owner_conflict: true,
            ..Self::uncertain()
        }
    }

    pub(super) fn direct_copy_value(&self) -> bool {
        self.proven
            && self.copy_evidence.is_some()
            && self.wrappers.is_empty()
            && self
                .fields
                .as_ref()
                .is_none_or(|fields| fields.iter().all(Self::direct_copy_value))
    }

    fn scalar_copy_value(ty: Option<Type>) -> Self {
        Self {
            ty,
            copy_evidence: Some(()),
            ..Self::value()
        }
    }

    fn text_value() -> Self {
        Self {
            ty: Some(super::STRING),
            text_source_model: Some("owned string"),
            ..Self::value()
        }
    }

    fn direct_string_value(&self) -> bool {
        self.text_source_model == Some("owned string") && self.wrappers.is_empty()
    }

    pub(super) fn borrowed_string_slice(&self) -> bool {
        self.text_source_model == Some("string slice")
            && self.wrappers.len() == 1
            && self.wrappers[0] == "reference"
    }

    fn uncertain() -> Self {
        Self {
            uncertain: true,
            ..Self::unknown()
        }
    }
}

#[derive(Clone, Debug)]
pub(super) enum SourceDeclaration {
    ExternalCrate,
    Module,
    Data,
    Constructor {
        owner: String,
        fields: Vec<SourceOrigin>,
        result: SourceOrigin,
    },
    Function {
        parameters: Vec<SourceOrigin>,
        result: SourceOrigin,
    },
    FnMutTemplate(Box<super::fn_mut_source_declarations::FnMutSourceDeclaration>),
    Constant {
        result: SourceOrigin,
    },
}

pub(super) struct SourceLookup {
    pub(super) declaration: SourceDeclaration,
    pub(super) full_name: String,
}

pub(super) struct SourceOrigins {
    pub(super) type_catalog: HashMap<String, SourceDeclaration>,
    pub(super) value_catalog: HashMap<String, SourceDeclaration>,
    pub(super) standard_sum_metadata_budget: super::standard_sum_metadata::MetadataBudget,
    pub(super) standard_sum_source_origins: HashMap<String, SourceOrigin>,
    pub(super) standard_sum_source_needs_final_admission: bool,
    pub(super) shared_array_handles: bool,
    pub(super) array_result_model: String,
    pub(super) owned_local_values: bool,
    fresh_calls: HashSet<Span>,
    fresh_vector_constructors: HashSet<Span>,
    pub(super) owned_binding_excluded: bool,
    pub(super) owned_container_source_enabled: bool,
    pub(super) borrowed_vector_source_enabled: bool,
    pub(super) borrowed_vector_needs_final_admission: bool,
    pub(super) source_position_materialized_nodes: usize,
    pub(super) source_position_materialized_units: usize,
    pub(super) owned_container_metadata_budget: super::standard_sum_metadata::MetadataBudget,
    source_binding_states: HashMap<String, String>,
    next_source_binding_identity: usize,
    owned_argument_transfers: HashSet<Span>,
    source_temporary_bindings: HashSet<String>,
    mutable_record_loans: super::mutable_record_loans::MutableRecordLoans,
    pub(super) collecting: bool,
    pub(super) catalog: HashMap<String, SourceDeclaration>,
    pub(super) nominal_copy_derivations: HashMap<String, nominal_copies::NominalCopyDerivation>,
    pub(super) nominal_copy_owners: HashSet<String>,
    pub(super) nominal_ownership_declarations:
        HashMap<String, super::nominal_ownership::NominalOwnershipDeclaration>,
    pub(super) standard_collection_owners:
        HashMap<String, crate::translation::source_signatures::SourceOwner>,
    pub(super) nominal_ownership_scope_complete: bool,
    pub(super) unit_clone_facts: HashMap<String, (Option<serde_json::Value>, bool)>,
    pub(super) scope: HashMap<String, SourceOrigin>,
    pub(super) current_type: SourceOrigin,
    // Source ranges survive the copies used to construct let chains and rows.
    // Erased wrapper expressions deliberately retain their operand's range, so
    // successive wraps and dereferences update the same source evidence.
    expressions: HashMap<Span, SourceOrigin>,
    pub(super) expression_type_cache: super::source_expression_types::ExpressionTypeCache,
    pattern_references: HashMap<Span, usize>,
    literal_pattern_aliases: HashMap<Span, Vec<SExpr>>,
    wildcard_references: VecDeque<usize>,
}

impl SourceOrigins {
    pub(super) fn new(catalog: Option<HashMap<String, SourceDeclaration>>, model: &str) -> Self {
        let mut standard_sum_source_origins = HashMap::new();
        let mut standard_sum_metadata_budget =
            super::standard_sum_metadata::MetadataBudget::default();
        if let Some(entries) = catalog.as_ref() {
            let mut unique = std::collections::BTreeMap::new();
            for entry in entries.values() {
                if let SourceDeclaration::Constructor { owner, result, .. } = entry {
                    if result.standard_sum_source.is_some() {
                        unique.entry(owner).or_insert(result);
                    }
                }
            }
            for (owner, result) in unique {
                let reserved = super::standard_sum_metadata::measure_origins(&[result])
                    .and_then(|cost| standard_sum_metadata_budget.reserve(cost));
                if let Err(error) = reserved {
                    standard_sum_metadata_budget.error = Some(error);
                    break;
                }
                standard_sum_source_origins.insert(owner.clone(), result.clone());
            }
        }
        Self {
            type_catalog: catalog
                .as_ref()
                .filter(|_| standard_sum_metadata_budget.error.is_none())
                .map(|entries| {
                    entries
                        .iter()
                        .filter(|(_, entry)| {
                            super::source_origins::type_owners::namespace_role(entry) == "type"
                        })
                        .map(|(name, entry)| (name.clone(), entry.clone()))
                        .collect()
                })
                .unwrap_or_default(),
            value_catalog: catalog
                .as_ref()
                .filter(|_| standard_sum_metadata_budget.error.is_none())
                .map(|entries| {
                    entries
                        .iter()
                        .filter(|(_, entry)| {
                            super::source_origins::type_owners::namespace_role(entry) == "value"
                        })
                        .map(|(name, entry)| (name.clone(), entry.clone()))
                        .collect()
                })
                .unwrap_or_default(),
            standard_sum_metadata_budget,
            standard_sum_source_origins,
            standard_sum_source_needs_final_admission: false,
            shared_array_handles: false,
            array_result_model: model.to_owned(),
            owned_local_values: false,
            fresh_calls: HashSet::new(),
            fresh_vector_constructors: HashSet::new(),
            collecting: catalog.is_none(),
            owned_binding_excluded: false,
            owned_container_source_enabled: false,
            borrowed_vector_source_enabled: false,
            borrowed_vector_needs_final_admission: false,
            source_position_materialized_nodes: 0,
            source_position_materialized_units: 0,
            owned_container_metadata_budget: super::standard_sum_metadata::MetadataBudget::default(
            ),
            source_binding_states: HashMap::new(),
            next_source_binding_identity: 0,
            owned_argument_transfers: HashSet::new(),
            source_temporary_bindings: HashSet::new(),
            mutable_record_loans: super::mutable_record_loans::MutableRecordLoans::default(),
            catalog: catalog.unwrap_or_default(),
            unit_clone_facts: HashMap::new(),
            nominal_copy_derivations: HashMap::new(),
            nominal_copy_owners: HashSet::new(),
            nominal_ownership_declarations: HashMap::new(),
            standard_collection_owners: HashMap::new(),
            nominal_ownership_scope_complete: false,
            scope: HashMap::new(),
            current_type: SourceOrigin::unknown(),
            expressions: HashMap::new(),
            expression_type_cache: super::source_expression_types::ExpressionTypeCache::default(),
            pattern_references: HashMap::new(),
            literal_pattern_aliases: HashMap::new(),
            wildcard_references: VecDeque::new(),
        }
    }

    pub(super) fn enter(
        &mut self,
        scope: Option<HashMap<String, SourceOrigin>>,
    ) -> HashMap<String, SourceOrigin> {
        let next = scope.unwrap_or_else(|| self.scope.clone());
        let next = next
            .into_iter()
            .map(|(name, origin)| {
                let origin = if origin.source_binding_identity.is_none() {
                    self.register_owned_binding(origin)
                } else {
                    origin
                };
                let origin = if origin.source_lexical_binding_identity.is_none() {
                    self.assign_copy_closure_lexical_binding(origin)
                } else {
                    origin
                };
                (name, origin)
            })
            .collect();
        std::mem::replace(&mut self.scope, next)
    }

    pub(super) fn typed(&mut self, ty: Type, path: &[String], wrappers: Vec<String>) -> Type {
        let stored = match &ty {
            Type::Named { path: names, .. } => self
                .lookup_type_owner(names, path)
                .filter(|owner| matches!(owner.declaration, SourceDeclaration::Data))
                .and_then(|owner| self.standard_sum_source_origins.get(&owner.full_name))
                .cloned(),
            _ => None,
        };
        if let Some(mut origin) = stored {
            origin.path = path.to_vec();
            origin.wrappers = wrappers;
            self.current_type = origin;
            return ty;
        }
        let mut origin = SourceOrigin::with_type(ty.clone(), path, wrappers);
        self.retain_nominal_copy_origin(&mut origin);
        self.current_type = origin;
        ty
    }

    pub(super) fn require_product_origin(
        &self,
        origin: &SourceOrigin,
        span: Option<Span>,
    ) -> Result<()> {
        if !self.collecting
            && (origin.fields.is_none()
                || !accept_rust_product_origin(
                    origin.direct_copy_value(),
                    self.owned_local_product(origin),
                ))
        {
            return Err(unsupported(
                "immutable Rust tuple product",
                "every field requires a direct recursively immutable scalar Copy source value",
                span,
            ));
        }
        Ok(())
    }

    pub(super) fn product(
        &mut self,
        node: &SExpr,
        path: &[String],
        span: Option<Span>,
    ) -> Result<()> {
        let SNode::Tuple { items, .. } = &node.node else {
            return Ok(());
        };
        let fields: Vec<_> = items.iter().map(|item| self.origin(item, path)).collect();
        let ty = self
            .owned_local_values
            .then(|| {
                fields
                    .iter()
                    .map(|field| field.ty.clone())
                    .collect::<Option<Vec<_>>>()
            })
            .flatten()
            .map(|elements| Type::Tuple { elements });
        let origin = SourceOrigin {
            ty,
            copy_evidence: fields
                .iter()
                .all(SourceOrigin::direct_copy_value)
                .then_some(()),
            fields: Some(fields),
            ..SourceOrigin::value()
        };
        self.require_product_origin(&origin, span)?;
        self.mark(node, origin);
        Ok(())
    }

    pub(super) fn owned_local_product(&self, origin: &SourceOrigin) -> bool {
        self.owned_local_values
            && origin
                .ty
                .as_ref()
                .is_some_and(super::owned_local_state::owned_pair)
            && origin.proven
            && origin.wrappers.is_empty()
            && origin.fields.as_ref().is_some_and(|fields| {
                fields
                    .iter()
                    .all(|field| field.proven && field.wrappers.is_empty())
                    && fields
                        .first()
                        .is_some_and(SourceOrigin::direct_string_value)
            })
    }

    pub(super) fn product_projection(
        &mut self,
        node: &SExpr,
        path: &[String],
        span: Option<Span>,
    ) -> Result<()> {
        let SNode::TupleIndex { object, index } = &node.node else {
            return Ok(());
        };
        let origin = self.origin(object, path);
        self.require_product_origin(&origin, span)?;
        let field = origin
            .fields
            .as_ref()
            .and_then(|fields| fields.get(*index))
            .cloned()
            .unwrap_or_else(SourceOrigin::unknown);
        self.mark(
            node,
            super::copy_closure_lifetimes::project_value_field(&origin, field, *index),
        );
        Ok(())
    }

    pub(super) fn declaration(
        &mut self,
        name: &str,
        path: &[String],
        mut entry: SourceDeclaration,
    ) -> Result<()> {
        if self.collecting {
            self.retain_nominal_copy_declaration(&mut entry);
            let mut full_path = path.to_vec();
            full_path.push(name.to_owned());
            let key = full_path.join(".");
            if let Some(existing) = self.namespace_entry(&key, "value") {
                let compatible = match (existing, &entry) {
                    (
                        SourceDeclaration::FnMutTemplate(left),
                        SourceDeclaration::FnMutTemplate(right),
                    ) => left.declaration == right.declaration,
                    (SourceDeclaration::FnMutTemplate(_), _)
                    | (_, SourceDeclaration::FnMutTemplate(_)) => {
                        type_owners::namespace_role(&entry) != "value"
                    }
                    _ => true,
                };
                if !compatible {
                    return Err(super::type_error(
                        format!("duplicate source declaration {key}"),
                        None,
                    ));
                }
            }
            if let Some(existing) = self.namespace_entry(&key, "type") {
                if !crate::translation::rust_source_namespace_rules::accept_rust_source_namespace_declaration_pair(type_owners::namespace_kind(existing),type_owners::namespace_kind(&entry)) {
                    return Err(unsupported(
                        "extern crate alloc",
                        "a source crate declaration cannot share its module name with another item",
                        None,
                    ));
                }
            }
            self.register_catalog_entry(key, entry);
        }
        Ok(())
    }

    /// Resolve forward aliases with the completed parser's existing resolver.
    pub(super) fn resolve_catalog_types<F>(&mut self, mut resolve: F)
    where
        F: FnMut(&[String], &[String]) -> Vec<String>,
    {
        fn resolve_origin<F>(origin: &mut SourceOrigin, resolve: &mut F)
        where
            F: FnMut(&[String], &[String]) -> Vec<String>,
        {
            if let Some(Type::Named { path, .. }) = &mut origin.ty {
                *path = resolve(path, &origin.path);
            }
            if let Some(fields) = &mut origin.fields {
                for field in fields {
                    resolve_origin(field, resolve);
                }
            }
            if let Some(fields) = &mut origin.callable_parameters {
                for field in fields {
                    resolve_origin(field, resolve);
                }
            }
            if let Some(result) = &mut origin.callable_result {
                resolve_origin(result, resolve);
            }
        }
        for declaration in self
            .catalog
            .values_mut()
            .chain(self.type_catalog.values_mut())
            .chain(self.value_catalog.values_mut())
        {
            match declaration {
                SourceDeclaration::Function { parameters, result } => {
                    parameters
                        .iter_mut()
                        .for_each(|parameter| resolve_origin(parameter, &mut resolve));
                    resolve_origin(result, &mut resolve);
                }
                SourceDeclaration::Constant { result } => resolve_origin(result, &mut resolve),
                SourceDeclaration::Constructor { fields, result, .. } => {
                    fields
                        .iter_mut()
                        .for_each(|field| resolve_origin(field, &mut resolve));
                    resolve_origin(result, &mut resolve);
                }
                SourceDeclaration::ExternalCrate
                | SourceDeclaration::Module
                | SourceDeclaration::FnMutTemplate(_)
                | SourceDeclaration::Data => {}
            }
        }
    }

    pub(super) fn box_owner(
        &self,
        segments: &[String],
        resolved: &[String],
        path: &[String],
        span: Option<Span>,
    ) -> Result<()> {
        if self.collecting {
            return Ok(());
        }
        let owner = resolved.join("::");
        let shadowed = [segments.first(), resolved.first()]
            .into_iter()
            .flatten()
            .any(|name| {
                self.lookup_type_owner(std::slice::from_ref(name), path)
                    .is_some_and(|entry| {
                        !matches!(entry.declaration, SourceDeclaration::ExternalCrate)
                    })
            });
        if !accept_rust_source_box_owner(&owner, shadowed) {
            return Err(unsupported(
                "Box owner",
                "only the unshadowed standard immutable Box is portable",
                span,
            ));
        }
        let declared = resolved
            .first()
            .and_then(|name| self.lookup_type_owner(std::slice::from_ref(name), path))
            .is_some_and(|entry| matches!(entry.declaration, SourceDeclaration::ExternalCrate));
        if !accept_rust_source_box_availability(&owner, declared) {
            return Err(unsupported(
                "Box source crate",
                "alloc requires an actual extern crate alloc declaration",
                span,
            ));
        }
        Ok(())
    }

    pub(super) fn vector_owner(
        &self,
        segments: &[String],
        resolved: &[String],
        path: &[String],
        span: Option<Span>,
    ) -> Result<()> {
        if self.collecting {
            return Ok(());
        }
        let shadowed = [segments.first(), resolved.first()]
            .into_iter()
            .flatten()
            .any(|name| {
                self.lookup_type_owner(std::slice::from_ref(name), path)
                    .is_some()
            });
        if !accept_rust_source_array_owner(&resolved.join("::"), shadowed) {
            return Err(unsupported(
                "Vec owner",
                "only the unshadowed standard vector is admitted by the immutable array model",
                span,
            ));
        }
        Ok(())
    }

    /// Compare retained source types by their existing declaration identities.
    pub(super) fn same_origin(&self, left: &SourceOrigin, right: &SourceOrigin) -> bool {
        if !crate::translation::rust_copy_closure_rules::read_rust_copy_closure_join_error(
            left.copy_closure_identity.is_some(),
            right.copy_closure_identity.is_some(),
            left.copy_closure_identity == right.copy_closure_identity,
        )
        .is_empty()
        {
            return false;
        }
        if !crate::translation::rust_callable_owner_rules::accept_rust_callable_owner_descriptors_equal(&callable_owners::descriptor(left),&callable_owners::descriptor(right)){return false;}
        self.compare_origin_structure(left, right)
    }
    fn compare_origin_structure(&self, left: &SourceOrigin, right: &SourceOrigin) -> bool {
        if !crate::translation::rust_byte_string_expression_rules::accept_rust_static_byte_array_owner_equality(&ByteStringText::from_text(left.array_source_model.unwrap_or("")),&ByteStringText::from_text(left.source_owner.as_ref().map_or("",|owner|owner.kind.as_str())),&ByteStringText::from_text(&left.source_owner.as_ref().map(|owner|owner.path.join("::")).unwrap_or_default()),&ByteStringText::from_text(right.source_owner.as_ref().map_or("",|owner|owner.kind.as_str())),&ByteStringText::from_text(&right.source_owner.as_ref().map(|owner|owner.path.join("::")).unwrap_or_default())){return false;}

        if left.proven != right.proven
            || left.diverging != right.diverging
            || left.uncertain != right.uncertain
            || left.wrappers != right.wrappers
            || left.copy_evidence != right.copy_evidence
            || left.copy_closure_identity != right.copy_closure_identity
            || left.float_source_model != right.float_source_model
            || left.text_source_model != right.text_source_model
            || left.array_source_model != right.array_source_model
        {
            return false;
        }
        match (&left.fields, &right.fields) {
            (Some(left), Some(right)) => {
                if left.len() != right.len()
                    || !left
                        .iter()
                        .zip(right)
                        .all(|(left, right)| self.compare_origin_structure(left, right))
                {
                    return false;
                }
            }
            (None, None) => {}
            _ => return false,
        }
        match (&left.callable_parameters, &right.callable_parameters) {
            (Some(left), Some(right)) => {
                if left.len() != right.len()
                    || !left
                        .iter()
                        .zip(right)
                        .all(|(left, right)| self.compare_origin_structure(left, right))
                {
                    return false;
                }
            }
            (None, None) => {}
            _ => return false,
        }
        match (&left.callable_result, &right.callable_result) {
            (Some(left), Some(right)) => {
                if !self.compare_origin_structure(left, right) {
                    return false;
                }
            }
            (None, None) => {}
            _ => return false,
        }
        match (&left.ty, &right.ty) {
            (
                Some(Type::Named {
                    path: left_path, ..
                }),
                Some(Type::Named {
                    path: right_path, ..
                }),
            ) => {
                match (
                    self.lookup_type_owner(left_path, &left.path),
                    self.lookup_type_owner(right_path, &right.path),
                ) {
                    (Some(left), Some(right)) => left.full_name == right.full_name,
                    _ => left_path == right_path && left.path == right.path,
                }
            }
            _ => left.ty == right.ty,
        }
    }
}

include!("source_origin_expressions.rs");

include!("source_origin_transfers.rs");
