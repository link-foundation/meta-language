//! Actual checked-program/emitter fact transport; all normative admission is
//! normally generated from the shared JavaScript decision source.
use super::super::emission::CheckedItem;
use super::{dependency_rules, recursive_groups::source_number};
use crate::translation::{ir::Decl, surface::External};
use serde_json::Value;
use std::collections::HashSet;

pub(super) struct OwnedPublication {
    pub(super) indices: Vec<Vec<usize>>,
}

struct Capture<'a> {
    item: &'a CheckedItem,
    roots: Vec<usize>,
    owners: Vec<usize>,
    roles: Vec<f64>,
    references: Vec<Vec<String>>,
}

/// Inspect actual checked declaration ownership using the publication validator.
/// Mutated or incomplete metadata is refused; this does not certify execution.
#[must_use]
pub fn accept_declaration_owned_emission(item: &CheckedItem, names: &[String]) -> bool {
    read_capture(item, names).is_some()
}

fn append_text(values: &mut Vec<f64>, text: &str) -> Option<()> {
    values.push(source_number(text.encode_utf16().count())?);
    values.extend(text.encode_utf16().map(f64::from));
    Some(())
}

fn append_type(
    values: &mut Vec<f64>,
    value: &Value,
    declarations: &[Decl],
    local: &[usize],
    depth: usize,
) -> Option<()> {
    if depth > 128 {
        return None;
    }
    match value {
        Value::Null => values.push(0.0),
        Value::Bool(value) => values.push(if *value { 2.0 } else { 1.0 }),
        Value::Number(value) => {
            let number = value.as_f64()?;
            if !number.is_finite() {
                return None;
            }
            values.extend([3.0, number]);
        }
        Value::String(value) => {
            values.push(4.0);
            append_text(values, value)?;
        }
        Value::Array(value) => {
            values.extend([5.0, source_number(value.len())?]);
            for item in value {
                append_type(values, item, declarations, local, depth + 1)?;
            }
        }
        Value::Object(value) => {
            let mut fields = value.keys().collect::<Vec<_>>();
            fields.sort();
            values.extend([6.0, source_number(fields.len())?]);
            for field in fields {
                append_text(values, field)?;
                if value.get("kind").and_then(Value::as_str) == Some("data") && field == "name" {
                    let name = value.get(field)?.as_str()?;
                    let actual = declarations
                        .iter()
                        .position(|entry| entry.full_name() == name)?;
                    values.extend([
                        7.0,
                        source_number(local.iter().position(|index| *index == actual)?)?,
                    ]);
                } else {
                    append_type(values, value.get(field)?, declarations, local, depth + 1)?;
                }
            }
        }
    }
    Some(())
}

// Same actual checked reference fields as typed-program-links. Literal text
// is never a declaration edge. Transport limits refuse unknown exhaustion.
fn declaration_references(entry: &Decl) -> Option<Vec<String>> {
    fn visit(
        value: &Value,
        references: &mut Vec<String>,
        visits: &mut usize,
        depth: usize,
    ) -> Option<()> {
        if !value.is_array() && !value.is_object() {
            return Some(());
        }
        if depth > 256 || *visits >= 100_000 {
            return None;
        }
        *visits += 1;
        match value {
            Value::Array(values) => {
                for value in values {
                    visit(value, references, visits, depth + 1)?;
                }
            }
            Value::Object(fields) => {
                let kind = fields.get("k").and_then(Value::as_str);
                let reference = if matches!(kind, Some("call" | "functionRef")) {
                    fields.get("fn")
                } else if kind == Some("ctor") {
                    fields.get("data")
                } else if fields.get("kind").and_then(Value::as_str) == Some("data") {
                    fields.get("name")
                } else {
                    None
                };
                if let Some(reference) = reference {
                    let name = reference.as_str()?;
                    if !references.iter().any(|previous| previous == name) {
                        references.push(name.to_owned());
                    }
                }
                for (field, child) in fields {
                    if field != "span" {
                        visit(child, references, visits, depth + 1)?;
                    }
                }
            }
            _ => {}
        }
        Some(())
    }
    let fields = match entry {
        Decl::Fn(function) => serde_json::json!([
            function
                .params
                .iter()
                .map(|parameter| &parameter.ty)
                .collect::<Vec<_>>(),
            function.ret,
            function.body
        ]),
        Decl::Data(data) => serde_json::json!([data
            .ctors
            .iter()
            .map(|constructor| constructor
                .fields
                .iter()
                .map(|field| &field.ty)
                .collect::<Vec<_>>())
            .collect::<Vec<_>>()]),
        Decl::Theorem(_) => return None,
    };
    let mut references = Vec::new();
    let mut visits = 0;
    visit(&fields, &mut references, &mut visits, 0)?;
    Some(references)
}

fn target_id(targets: &mut Vec<String>, value: &str) -> Option<f64> {
    if value.is_empty() {
        return None;
    }
    let index = targets
        .iter()
        .position(|target| target == value)
        .unwrap_or_else(|| {
            targets.push(value.to_owned());
            targets.len() - 1
        });
    source_number(index)
}

fn read_capture<'a>(item: &'a CheckedItem, names: &[String]) -> Option<Capture<'a>> {
    let program = &item.program;
    let emitted = &item.emitted;
    let metadata = item.definition_ownership.as_ref()?;
    if item.source_language != "JavaScript"
        || program.source_language != crate::translation::Language::JavaScript
        || program
            .main
            .as_ref()
            .is_some_and(|main| !main.effects.is_empty())
        || !program.imports.is_empty()
        || program.output_threaded
        || program.aborts_threaded
        || program.source_runtime_shell.is_some()
    {
        return None;
    }
    let declarations = &program.declarations;
    if declarations
        .iter()
        .map(Decl::full_name)
        .collect::<HashSet<_>>()
        .len()
        != declarations.len()
        || names.iter().collect::<HashSet<_>>().len() != names.len()
    {
        return None;
    }
    let roots=names.iter().map(|name|declarations.iter().position(|entry|
        matches!(entry,Decl::Fn(function) if function.name==*name && function.full_name==*name
            && function.module_path.is_empty() && !function.generated
            && function.params.iter().all(|parameter|parameter.guard.is_none()&&!parameter.optional&&parameter.default_value.is_none()))))
        .collect::<Option<Vec<_>>>()?;
    let root_starts = roots
        .iter()
        .map(|index| source_number(declarations[*index].span()?.start))
        .collect::<Option<Vec<_>>>()?;
    let root_ends = roots
        .iter()
        .map(|index| source_number(declarations[*index].span()?.end))
        .collect::<Option<Vec<_>>>()?;
    let starts = declarations
        .iter()
        .map(|entry| source_number(entry.span()?.start))
        .collect::<Option<Vec<_>>>()?;
    let ends = declarations
        .iter()
        .map(|entry| source_number(entry.span()?.end))
        .collect::<Option<Vec<_>>>()?;
    let roles = declarations
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            if !entry.module_path().is_empty() {
                -1.0
            } else if roots.contains(&index) {
                0.0
            } else {
                match entry {
                    Decl::Fn(function) if function.generated => 1.0,
                    Decl::Data(data) if data.generated && !data.output => 2.0,
                    _ => -1.0,
                }
            }
        })
        .collect::<Vec<_>>();
    let owners =
        dependency_rules::read_source_owners(&root_starts, &root_ends, &starts, &ends, &roles);
    if owners.len() != declarations.len() {
        return None;
    }
    let owners = owners
        .into_iter()
        .map(|owner| {
            crate::translation::frontend_rules::ml_array::number_index(owner)
                .filter(|owner| *owner < roots.len())
        })
        .collect::<Option<Vec<_>>>()?;
    let references = declarations
        .iter()
        .map(declaration_references)
        .collect::<Option<Vec<_>>>()?;
    let mut edge_starts = Vec::new();
    let mut edge_counts = Vec::new();
    let mut edge_targets = Vec::new();
    for row in &references {
        edge_starts.push(source_number(edge_targets.len())?);
        let before = edge_targets.len();
        for name in row {
            if let Some(index) = declarations
                .iter()
                .position(|entry| entry.full_name() == name)
            {
                edge_targets.push(source_number(index)?);
            } else if !program
                .externals
                .iter()
                .any(|external| external.name() == name)
            {
                return None;
            }
        }
        edge_counts.push(source_number(edge_targets.len() - before)?);
    }
    let owner_numbers = owners
        .iter()
        .copied()
        .map(source_number)
        .collect::<Option<Vec<_>>>()?;
    if !dependency_rules::accept_auxiliary_closure(
        &owner_numbers,
        &roles,
        &edge_starts,
        &edge_counts,
        &edge_targets,
    ) {
        return None;
    }
    let externals = crate::translation::emit_rust::read_rust_external_targets(program);
    if metadata.external_targets != externals {
        return None;
    }
    let mut targets = Vec::new();
    let mut forbidden = Vec::new();
    for row in &externals {
        forbidden.push(target_id(&mut targets, &row.target)?);
    }
    if metadata.blocks.len() != emitted.definitions.len() {
        return None;
    }
    let constructors = declarations
        .iter()
        .map(|entry| {
            source_number(match entry {
                Decl::Data(data) => data.ctors.len(),
                _ => 0,
            })
        })
        .collect::<Option<Vec<_>>>()?;
    let mut blocks = Vec::new();
    let mut mappings = Vec::new();
    for row in &metadata.blocks {
        let index = declarations
            .iter()
            .position(|entry| entry.full_name() == row.source)?;
        let declaration = &declarations[index];
        if row.kind != declaration.kind()
            || row.source_span != declaration.span()
            || row.mapping_start > row.mapping_end
            || row.mapping_end > emitted.mappings.len()
            || emitted.definitions.get(row.definition_index) != Some(&row.definition)
            || row.definition.trim().is_empty()
            || row.targets.len() != row.mapping_end - row.mapping_start
        {
            return None;
        }
        blocks.extend([
            source_number(index)?,
            source_number(row.mapping_start)?,
            source_number(row.mapping_end - row.mapping_start)?,
            source_number(row.definition_index)?,
        ]);
        for (position, mapping) in emitted.mappings[row.mapping_start..row.mapping_end]
            .iter()
            .enumerate()
        {
            let expected = if position == 0 {
                declaration.full_name().to_owned()
            } else {
                let Decl::Data(data) = declaration else {
                    return None;
                };
                format!("{}.{}", data.full_name, data.ctors.get(position - 1)?.name)
            };
            let kind = if position > 0 {
                "constructor"
            } else if matches!(declaration, Decl::Fn(_)) {
                "function"
            } else {
                "data"
            };
            if mapping.source != expected
                || mapping.kind != kind
                || mapping.source_span != declaration.span()
                || row.targets.get(position) != Some(&mapping.target)
            {
                return None;
            }
            mappings.extend([
                source_number(index)?,
                source_number(position)?,
                target_id(&mut targets, &mapping.target)?,
            ]);
        }
    }
    if mappings.len() != emitted.mappings.len() * 3
        || !dependency_rules::accept_owned_emission(
            &roles,
            &constructors,
            &blocks,
            &mappings,
            &forbidden,
        )
    {
        return None;
    }
    Some(Capture {
        item,
        roots,
        owners,
        roles,
        references,
    })
}

impl Capture<'_> {
    fn inventory(&self, owner: usize, original_names: &[String]) -> Option<Vec<f64>> {
        let declarations = &self.item.program.declarations;
        let actual = (0..declarations.len())
            .filter(|index| self.owners[*index] == owner)
            .collect::<Vec<_>>();
        let origin = declarations[*self.roots.get(owner)?].span()?.start;
        let mut values = vec![1.0, source_number(actual.len())?];
        for index in &actual {
            let declaration = &declarations[*index];
            let span = declaration.span()?;
            values.extend([
                self.roles[*index],
                source_number(span.start.checked_sub(origin)?)?,
                source_number(span.end.checked_sub(origin)?)?,
            ]);
            match declaration {
                Decl::Fn(function) => {
                    // Original external interfaces cannot carry a private
                    // generated datatype identity at any nested position.
                    let type_owners = if self.roles[*index] == 0.0 {
                        &[][..]
                    } else {
                        actual.as_slice()
                    };
                    values.push(source_number(function.params.len())?);
                    for parameter in &function.params {
                        append_type(
                            &mut values,
                            &serde_json::to_value(&parameter.ty).ok()?,
                            declarations,
                            type_owners,
                            0,
                        )?;
                    }
                    append_type(
                        &mut values,
                        &serde_json::to_value(&function.ret).ok()?,
                        declarations,
                        type_owners,
                        0,
                    )?;
                }
                Decl::Data(data) => {
                    values.push(source_number(data.ctors.len())?);
                    for constructor in &data.ctors {
                        values.push(source_number(constructor.fields.len())?);
                        for field in &constructor.fields {
                            append_type(
                                &mut values,
                                &serde_json::to_value(&field.ty).ok()?,
                                declarations,
                                &actual,
                                0,
                            )?;
                        }
                    }
                }
                Decl::Theorem(_) => return None,
            }
            values.push(source_number(self.references[*index].len())?);
            for name in &self.references[*index] {
                let local = declarations
                    .iter()
                    .position(|entry| entry.full_name() == name)
                    .and_then(|target| actual.iter().position(|index| *index == target));
                if let Some(local) = local {
                    values.extend([1.0, source_number(local)?]);
                } else if let Some(root) = original_names.iter().position(|root| root == name) {
                    values.extend([2.0, source_number(root)?]);
                } else {
                    values.push(3.0);
                    append_text(&mut values, name)?;
                }
            }
        }
        Some(values)
    }
}

pub(super) fn plan_owned_publication(
    names: &[String],
    sources: &[&str],
    whole: &CheckedItem,
    members: &[CheckedItem],
    module_scope_known: bool,
) -> Option<OwnedPublication> {
    if names.len() != sources.len()
        || names.len() != members.len()
        || whole.source != sources.join("\n\n")
        || whole.signatures.len() != names.len()
        || whole.signatures.iter().any(
            |signature| !matches!(signature,External::Function{name,..}if names.contains(name)),
        )
        || whole
            .signatures
            .iter()
            .map(External::name)
            .collect::<HashSet<_>>()
            .len()
            != names.len()
    {
        return None;
    }
    let actual = read_capture(whole, names)?;
    let mut indices = Vec::new();
    for (owner, name) in names.iter().enumerate() {
        let member = members.get(owner)?;
        if member.source != sources[owner] {
            return None;
        }
        let one = read_capture(member, std::slice::from_ref(name))?;
        if !dependency_rules::accept_auxiliary_correspondence(
            &actual.inventory(owner, names)?,
            &one.inventory(0, names)?,
        ) {
            return None;
        }
        let expected = whole
            .signatures
            .iter()
            .filter(|signature| signature.name() == name)
            .cloned()
            .collect::<Vec<_>>();
        if member.signatures != expected {
            return None;
        }
        let whole_root = whole
            .emitted
            .mappings
            .iter()
            .filter(|mapping| mapping.kind == "function" && mapping.source == *name)
            .collect::<Vec<_>>();
        let member_root = member
            .emitted
            .mappings
            .iter()
            .filter(|mapping| mapping.kind == "function" && mapping.source == *name)
            .collect::<Vec<_>>();
        if whole_root.len() != 1
            || member_root.len() != 1
            || whole_root[0].target != member_root[0].target
        {
            return None;
        }
        let owned = whole
            .definition_ownership
            .as_ref()?
            .blocks
            .iter()
            .filter_map(|row| {
                let declaration = whole
                    .program
                    .declarations
                    .iter()
                    .position(|entry| entry.full_name() == row.source)?;
                (actual.owners[declaration] == owner).then_some(row.definition_index)
            })
            .collect::<Vec<_>>();
        if owned.is_empty() {
            return None;
        }
        indices.push(owned);
    }
    let proofs = [
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        if module_scope_known { 1.0 } else { 0.0 },
    ];
    dependency_rules::accept_owned_publication(
        source_number(names.len())?,
        source_number(whole.program.declarations.len())?,
        source_number(whole.signatures.len())?,
        source_number(members.len())?,
        &proofs,
    )
    .then_some(OwnedPublication { indices })
}

impl super::Binder<'_, '_> {
    pub(super) fn check_owned_recursive_members(
        &self,
        state: &super::Visit,
        members: &[usize],
        externals: &[External],
        whole: &CheckedItem,
    ) -> Option<Vec<(usize, super::super::Translated)>> {
        use super::super::{Context, GroupKind, translate_group};
        use crate::translation::javascript::ModuleContext;
        if self.from != "JavaScript" || self.to != "Rust" {
            return None;
        }
        let whole_generic = whole.emitted.definitions.join("\n\n");
        if crate::grammar::decorate_emitted(self.to, &whole_generic, self.decorators)
            != whole_generic
        {
            return None;
        }
        let names = members
            .iter()
            .map(|member| state.scans[*member].as_ref()?.declares.clone())
            .collect::<Option<Vec<_>>>()?;
        let sources = members
            .iter()
            .map(|member| match &self.groups[*member].kind {
                GroupKind::Item { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()?;
        let mut captures = Vec::new();
        for (owner, member) in members.iter().copied().enumerate() {
            let mut actual_externals = externals.to_vec();
            actual_externals.extend(
                whole
                    .signatures
                    .iter()
                    .filter(|signature| signature.name() != names[owner].as_str())
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
            let capture = super::super::emission::check_and_emit_item_with_ownership(
                sources[owner],
                self.from,
                self.to,
                &context,
                &std::collections::HashMap::new(),
            )
            .ok()??;
            let member_generic = capture.emitted.definitions.join("\n\n");
            if crate::grammar::decorate_emitted(self.to, &member_generic, self.decorators)
                != member_generic
            {
                return None;
            }
            // Retain the original independent complete member translation.
            let original = translate_group(
                &self.groups[member],
                self.from,
                self.to,
                self.decorators,
                &context,
            );
            if original.status != "translated" {
                return None;
            }
            captures.push(capture);
        }
        let plan =
            plan_owned_publication(&names, &sources, whole, &captures, state.module_scope_known)?;
        let mut staged = Vec::new();
        for (owner, member) in members.iter().copied().enumerate() {
            let GroupKind::Item { text, term } = &self.groups[member].kind else {
                return None;
            };
            let definitions = plan
                .indices
                .get(owner)?
                .iter()
                .map(|index| whole.emitted.definitions.get(*index))
                .collect::<Option<Vec<_>>>()?;
            if definitions.is_empty() {
                return None;
            }
            let generic = definitions
                .into_iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
                .join("\n\n");
            let code = crate::grammar::decorate_emitted(self.to, &generic, self.decorators);
            // The closed initial profile cannot change a coherent whole
            // namespace with a member-specific textual decorator.
            if code != generic || code.trim().is_empty() {
                return None;
            }
            let count = plan.indices[owner].len();
            let mut lines = vec![format!(
                "{}{} {} items={} sha256={}",
                super::super::TRANSLATED,
                self.from,
                term,
                count,
                super::super::sha256(&code)
            )];
            lines.extend(
                super::super::LINE_BREAK
                    .split(text)
                    .map(super::super::source_line),
            );
            lines.push(code);
            staged.push((
                member,
                super::super::Translated {
                    code: Some(lines.join("\n")),
                    preludes: whole.emitted.preludes.clone(),
                    status: "translated",
                    reason: None,
                    signatures: whole
                        .signatures
                        .iter()
                        .filter(|signature| signature.name() == names[owner].as_str())
                        .cloned()
                        .collect(),
                },
            ));
        }
        (staged.len() == members.len()).then_some(staged)
    }
}
