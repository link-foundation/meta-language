//! Actual source declarations into the shared nominal Copy dependency plan.
use super::{Result, SourceDeclaration, SourceOrigin, SourceOrigins, Span, Type, unsupported};
use crate::translation::rust_nominal_copy_rules as rules;
use serde_json::Value;

#[derive(Clone, Copy)]
pub(in super::super) struct NominalCopyDerivation {
    copy: bool,
    clone: bool,
    span: Option<Span>,
}
fn count(value: usize) -> f64 {
    u32::try_from(value).map_or(f64::INFINITY, f64::from)
}

impl SourceOrigins {
    pub(in super::super) fn register_nominal_copy_declaration(
        &mut self,
        name: &str,
        path: &[String],
        derivations: &[Value],
    ) {
        let traits = derivations
            .iter()
            .filter_map(|value| value["traits"].as_array())
            .flatten();
        let mut copy = false;
        let mut clone = false;
        for value in traits {
            copy |= value.as_str() == Some("Copy");
            clone |= value.as_str() == Some("Clone");
        }
        let mut owner = path.to_vec();
        owner.push(name.to_owned());
        let span = derivations
            .iter()
            .find(|value| {
                value["traits"]
                    .as_array()
                    .is_some_and(|traits| traits.iter().any(|name| name.as_str() == Some("Copy")))
            })
            .and_then(|value| serde_json::from_value::<Span>(value["span"].clone()).ok());
        self.nominal_copy_derivations
            .insert(owner.join("."), NominalCopyDerivation { copy, clone, span });
    }

    pub(in super::super) fn retain_nominal_copy_origin(&self, origin: &mut SourceOrigin) {
        if let Some(fields) = &mut origin.fields {
            for field in fields {
                self.retain_nominal_copy_origin(field);
            }
        }
        if let Some(fields) = &mut origin.callable_parameters {
            for field in fields {
                self.retain_nominal_copy_origin(field);
            }
        }
        if let Some(result) = &mut origin.callable_result {
            self.retain_nominal_copy_origin(result);
        }
        if let Some(Type::Named { path, .. }) = &origin.ty {
            origin.copy_evidence = self
                .lookup_type_owner(path, &origin.path)
                .filter(|owner| matches!(owner.declaration, SourceDeclaration::Data))
                .filter(|owner| self.nominal_copy_owners.contains(&owner.full_name))
                .map(|_| ());
        } else if matches!(origin.ty, Some(Type::Tuple { .. })) {
            origin.copy_evidence = origin
                .fields
                .as_ref()
                .is_some_and(|fields| fields.iter().all(SourceOrigin::direct_copy_value))
                .then_some(());
        }
    }

    fn nominal_copy_field(
        &self,
        origin: &SourceOrigin,
        owners: &[String],
        targets: &mut Vec<f64>,
        leaves: &mut Vec<bool>,
    ) {
        if matches!(origin.ty, Some(Type::Tuple { .. }))
            && origin.wrappers.is_empty()
            && let Some(fields) = &origin.fields
        {
            for field in fields {
                self.nominal_copy_field(field, owners, targets, leaves);
            }
            return;
        }
        let owner = match &origin.ty {
            Some(Type::Named { path, .. }) => self
                .lookup_type_owner(path, &origin.path)
                .filter(|owner| matches!(owner.declaration, SourceDeclaration::Data)),
            _ => None,
        };
        let target =
            owner.and_then(|owner| owners.iter().position(|name| name == &owner.full_name));
        targets.push(target.map_or(-1.0, count));
        leaves.push(rules::accept_rust_nominal_copy_field_leaf(
            origin.ty.as_ref().map_or("", Type::kind),
            &origin.wrappers,
            origin.copy_evidence.is_some() && origin.proven,
        ));
    }

    pub(in super::super) fn finish_nominal_copies(&mut self) -> Result<()> {
        let mut owners = self
            .nominal_copy_derivations
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        owners.sort();
        let (mut copies, mut clones, mut starts, mut counts, mut targets, mut leaves) = (
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        for owner in &owners {
            let derivation = self.nominal_copy_derivations[owner];
            copies.push(derivation.copy);
            clones.push(derivation.clone);
            starts.push(count(targets.len()));
            let before = targets.len();
            for declaration in self.catalog.values() {
                if let SourceDeclaration::Constructor {
                    owner: actual,
                    fields,
                    ..
                } = declaration
                    && actual == owner
                {
                    for field in fields {
                        self.nominal_copy_field(field, &owners, &mut targets, &mut leaves);
                    }
                }
            }
            counts.push(count(targets.len() - before));
        }
        let plan = rules::plan_rust_nominal_copy_owners(
            &owners, &copies, &clones, &starts, &counts, &targets, &leaves,
        );
        if plan.first() != Some(&0.0) {
            let reason = match plan.first() {
                Some(-2.0) => "a derived Copy owner requires its actual Clone derivation",
                Some(-3.0) => {
                    "every field of every derived Copy variant needs an actual Copy proof"
                }
                _ => {
                    "the complete nominal Copy source graph must retain valid distinct owners and fields"
                }
            };
            let span = plan
                .get(1)
                .copied()
                .and_then(crate::translation::frontend_rules::ml_array::number_index)
                .and_then(|index| owners.get(index))
                .and_then(|owner| self.nominal_copy_derivations.get(owner))
                .and_then(|derive| derive.span);
            return Err(unsupported("Rust nominal Copy derivation", reason, span));
        }
        self.nominal_copy_owners = owners
            .into_iter()
            .enumerate()
            .filter_map(|(index, name)| (plan[index + 1] == 1.0).then_some(name))
            .collect();
        let entries = self.catalog.clone();
        for (name, mut declaration) in entries {
            self.retain_nominal_copy_declaration(&mut declaration);
            self.catalog.insert(name, declaration);
        }
        Ok(())
    }

    pub(in super::super) fn retain_nominal_copy_declaration(
        &self,
        declaration: &mut SourceDeclaration,
    ) {
        match declaration {
            SourceDeclaration::Function { parameters, result } => {
                for parameter in parameters {
                    self.retain_nominal_copy_origin(parameter);
                }
                self.retain_nominal_copy_origin(result);
            }
            SourceDeclaration::Constructor { fields, result, .. } => {
                for field in fields {
                    self.retain_nominal_copy_origin(field);
                }
                self.retain_nominal_copy_origin(result);
            }
            SourceDeclaration::Constant { result } => self.retain_nominal_copy_origin(result),
            SourceDeclaration::Data
            | SourceDeclaration::Module
            | SourceDeclaration::ExternalCrate => {}
        }
    }
}
