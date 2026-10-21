//! Host facts and checked atomic publication. Generated rules alone schedule components.
use super::super::emission::check_and_emit_item;
use super::super::{Context, GroupKind, Translated, project_type_script_typing, translate_group};
use super::dependency_rules;
use super::{Binder, Scan, Visit, accept_signature};
use crate::translation::{
    check::check_program,
    frontend_rules::ml_array::number_index,
    ir::Decl,
    javascript::{ModuleBindingScope, ModuleContext, parse_javascript_bound_in},
    surface::External,
};
use std::collections::{HashMap, HashSet};

pub(super) struct Components {
    leaders: Vec<usize>,
    members: Vec<Vec<usize>>,
}

pub(super) fn source_number(value: usize) -> Option<f64> {
    let number = serde_json::Number::from(value).as_f64()?;
    (number_index(number) == Some(value) && number <= 9_007_199_254_740_991.0).then_some(number)
}

pub(super) fn read_components(
    scans: &[Option<Scan>],
    owners: &HashMap<String, usize>,
) -> Option<Components> {
    let (mut starts, mut counts, mut targets) = (Vec::new(), Vec::new(), Vec::new());
    for scan in scans {
        starts.push(source_number(targets.len())?);
        let before = targets.len();
        if let Some(scan) = scan
            && scan.imports.is_none()
        {
            for name in &scan.mentions {
                if let Some(owner) = owners.get(name) {
                    targets.push(source_number(*owner)?);
                }
            }
        }
        counts.push(source_number(targets.len() - before)?);
    }
    let actual = dependency_rules::read_components(&starts, &counts, &targets);
    if actual.len() != scans.len() {
        return None;
    }
    let leaders = actual
        .into_iter()
        .map(|number| number_index(number).filter(|leader| *leader < scans.len()))
        .collect::<Option<Vec<_>>>()?;
    let mut members = vec![Vec::new(); scans.len()];
    for (index, leader) in leaders.iter().copied().enumerate() {
        if leader > index || leaders[leader] != leader {
            return None;
        }
        members[leader].push(index);
    }
    Some(Components { leaders, members })
}

impl Binder<'_, '_> {
    pub(super) fn module_binding_scope(state: &Visit) -> ModuleBindingScope {
        ModuleBindingScope {
            names: state.module_binding_names.clone(),
            known: state.module_scope_known,
        }
    }

    pub(super) fn module_scope_known(&self, state: &Visit) -> bool {
        if state
            .owner_counts
            .values()
            .any(|count| !super::accept_signature_binding(true, *count))
        {
            return false;
        }
        // Conditional candidate classification includes every actual known
        // outer owner. Its standalone standard-owner assumption is discharged
        // only by the final unique safe-form or real whole-module proof below.
        let mut candidate_scope = Self::module_binding_scope(state);
        candidate_scope.known = true;
        let (_, literal_groups) = super::super::scope::collect_literal_bindings(
            self.groups,
            true,
            self.from,
            self.to,
            self.decorators,
            self.utf16_strings,
            Some(&candidate_scope),
        );
        let forms = self
            .groups
            .iter()
            .enumerate()
            .map(|(index, group)| match &group.kind {
                GroupKind::Comment { .. } => "comment",
                GroupKind::Item { term, .. } if term == "ERROR" => "unknown module form",
                GroupKind::Item { .. }
                    if state.scans[index]
                        .as_ref()
                        .is_some_and(|scan| scan.function_declaration) =>
                {
                    "named function declaration"
                }
                GroupKind::Item { .. }
                    if state.scans[index]
                        .as_ref()
                        .is_some_and(|scan| scan.imports.is_some()) =>
                {
                    "named import"
                }
                GroupKind::Item { .. }
                    if state.scans[index]
                        .as_ref()
                        .is_some_and(|scan| scan.empty_class_declaration.is_some()) =>
                {
                    "empty class declaration"
                }
                GroupKind::Item { .. } if literal_groups.contains(&index) => {
                    "primitive literal binding"
                }
                _ => "unknown module form",
            })
            .collect::<Vec<_>>();
        if dependency_rules::accept_module_scope(&forms, false) {
            return true;
        }
        let mut externals = Vec::new();
        for (index, scan) in state.scans.iter().enumerate() {
            if scan.as_ref().is_none_or(|scan| scan.imports.is_none()) {
                continue;
            }
            if let Some(out) = &state.outs[index]
                && out.status == "translated"
            {
                externals.extend(out.signatures.iter().cloned());
            }
        }
        let source = if self.from == "TypeScript" {
            match project_type_script_typing(self.source) {
                Ok(source) => source,
                Err(_) => return false,
            }
        } else {
            self.source.to_owned()
        };
        let context = ModuleContext {
            externals,
            module_directory: Some(self.module_directory.to_vec()),
        };
        let complete_checked = parse_javascript_bound_in(&source, &context, &HashMap::new(), true)
            .and_then(|surface| check_program(&surface))
            .is_ok();
        dependency_rules::accept_module_scope(&forms, complete_checked)
    }

    pub(super) fn try_recursive_group(&self, state: &mut Visit, index: usize) -> bool {
        let Some(components) = &state.components else {
            return false;
        };
        let Some(&leader) = components.leaders.get(index) else {
            return false;
        };
        let members = components.members[leader].clone();
        if state.attempted.contains(&leader) {
            return false;
        }
        let Some(count) = source_number(members.len()) else {
            return false;
        };
        let ordinary = members.iter().all(|member| {
            state.scans[*member]
                .as_ref()
                .is_some_and(|scan| scan.function_declaration && !scan.is_async)
        });
        let unique = state.module_scope_known
            && members.iter().all(|member| {
                state.scans[*member]
                    .as_ref()
                    .and_then(|scan| scan.declares.as_ref())
                    .is_some_and(|name| state.owner_counts.get(name) == Some(&1))
            });
        if !dependency_rules::accept_group(count, ordinary, unique) {
            return false;
        }
        state.attempted.insert(leader);
        let membership = members.iter().copied().collect::<HashSet<_>>();
        for member in &members {
            state.visiting.insert(*member);
        }
        let mut externals: Vec<External> = Vec::new();
        for member in &members {
            let mentions = state.scans[*member]
                .as_ref()
                .map_or_else(Vec::new, |scan| scan.mentions.clone());
            for name in mentions {
                let Some(owner) = state.owners.get(&name).copied() else {
                    continue;
                };
                if membership.contains(&owner)
                    || !super::accept_signature_binding(
                        state.module_scope_known,
                        state.owner_counts.get(&name).copied().unwrap_or(0),
                    )
                {
                    continue;
                }
                self.visit(state, owner);
                let Some(out) = &state.outs[owner] else {
                    continue;
                };
                if out.status != "translated"
                    || state.scans[owner]
                        .as_ref()
                        .is_some_and(|scan| scan.is_async)
                {
                    continue;
                }
                if let Some(signature) = out.signatures.iter().find(|signature| {
                    signature.name() == name.as_str() && accept_signature(signature)
                }) {
                    if let Some(previous) = externals
                        .iter_mut()
                        .find(|previous| previous.name() == name.as_str())
                    {
                        *previous = signature.clone();
                    } else {
                        externals.push(signature.clone());
                    }
                }
            }
        }
        let completed = self.check_recursive_members(state, &members, &externals);
        for member in &members {
            state.visiting.remove(member);
        }
        if let Some(completed) = completed {
            // No staged peer signature is visible to a visit or fallback above.
            for (member, out) in completed {
                state.outs[member] = Some(out);
            }
            return true;
        }
        false
    }

    fn check_recursive_members(
        &self,
        state: &Visit,
        members: &[usize],
        externals: &[External],
    ) -> Option<Vec<(usize, Translated)>> {
        let names = members
            .iter()
            .map(|member| state.scans[*member].as_ref()?.declares.clone())
            .collect::<Option<Vec<_>>>()?;
        let texts = members
            .iter()
            .map(|member| match &self.groups[*member].kind {
                GroupKind::Item { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()?;
        let source = texts.join("\n\n");
        let context = Context {
            module: ModuleContext {
                externals: externals.to_vec(),
                module_directory: Some(self.module_directory.to_vec()),
            },
            utf16_strings: self.utf16_strings,
            module_binding_scope: Some(Self::module_binding_scope(state)),
        };
        let whole =
            check_and_emit_item(&source, self.from, self.to, &context, &HashMap::new()).ok()??;
        let declarations = &whole.program.declarations;
        let signatures = &whole.signatures;
        let flat=whole.program.main.as_ref().is_none_or(|main|main.effects.is_empty())&&whole.program.imports.is_empty()
            &&declarations.iter().all(|declaration|matches!(declaration,Decl::Fn(function)if function.module_path.is_empty()&&!function.generated&&names.contains(&function.name)))
            &&declarations.iter().map(|declaration|declaration.name()).collect::<HashSet<_>>().len()==names.len()
            &&signatures.iter().all(|signature|matches!(signature,External::Function{name,..}if names.contains(name)))
            &&signatures.iter().map(External::name).collect::<HashSet<_>>().len()==names.len();
        let count = source_number(names.len())?;
        let declaration_count = source_number(declarations.len())?;
        let signature_count = source_number(signatures.len())?;
        let emitted = !whole.emitted.definitions.is_empty();
        if !dependency_rules::accept_publication(
            count,
            declaration_count,
            signature_count,
            flat,
            emitted,
            true,
        ) {
            return None;
        }
        let targets = whole
            .emitted
            .mappings
            .iter()
            .filter(|mapping| mapping.kind == "function" && names.contains(&mapping.source))
            .collect::<Vec<_>>();
        if targets.len() != names.len()
            || targets
                .iter()
                .map(|mapping| mapping.target.as_str())
                .collect::<HashSet<_>>()
                .len()
                != names.len()
        {
            return None;
        }
        let mut completed = Vec::new();
        for (&member, name) in members.iter().zip(&names) {
            let mut actual_externals = externals.to_vec();
            actual_externals.extend(
                signatures
                    .iter()
                    .filter(|signature| signature.name() != name.as_str())
                    .cloned(),
            );
            let context = Context {
                module: ModuleContext {
                    externals: actual_externals,
                    module_directory: Some(self.module_directory.to_vec()),
                },
                utf16_strings: self.utf16_strings,
                module_binding_scope: Some(Self::module_binding_scope(state)),
            };
            let text = match &self.groups[member].kind {
                GroupKind::Item { text, .. } => text,
                _ => return None,
            };
            let capture =
                check_and_emit_item(text, self.from, self.to, &context, &HashMap::new()).ok()??;
            let out = translate_group(
                &self.groups[member],
                self.from,
                self.to,
                self.decorators,
                &context,
            );
            if out.status != "translated" {
                return None;
            }
            let own = out
                .signatures
                .iter()
                .filter(|signature| signature.name() == name.as_str())
                .collect::<Vec<_>>();
            let expected = signatures
                .iter()
                .filter(|signature| signature.name() == name.as_str())
                .collect::<Vec<_>>();
            let actual_targets = capture
                .emitted
                .mappings
                .iter()
                .filter(|mapping| mapping.kind == "function")
                .collect::<Vec<_>>();
            let target = targets.iter().find(|mapping| mapping.source == *name)?;
            let matching = own == expected
                && capture.program.declarations.len() == 1
                && actual_targets.len() == 1
                && actual_targets[0].source == *name
                && actual_targets[0].target == target.target;
            if !dependency_rules::accept_publication(
                count,
                declaration_count,
                signature_count,
                flat,
                !capture.emitted.definitions.is_empty(),
                matching,
            ) {
                return None;
            }
            completed.push((member, out));
        }
        (completed.len() == members.len()).then_some(completed)
    }
}
