//! Replay the original ordinary module ownership carried by a typed archive.
use super::{Data, Decoder};
use crate::source_definition_catalog_rules as rules;
use crate::translation::Span;
use crate::translation::javascript::has_source_library_typedef;
use crate::translation::lexer::Source;
use crate::translation::source_library_closure::{Closure, Module, compose_with_re_exports};
use crate::translation::typed_program_link_rules::read_program_reference_source_field;
use crate::typed_program_links::record_shapes::qualified_field;
use crate::typed_program_links::source_span_rules::accept_source_span_offset;
use crate::typed_program_links::{Result, error};
use crate::{LinkId, LinkType};
use serde::Deserialize;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct Directory {
    entry_path: String,
    re_exports: String,
    modules: Vec<ModuleSource>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModuleSource {
    path: String,
    source: String,
}

fn failure(message: impl std::fmt::Display) -> crate::typed_program_links::TypedProgramLinksError {
    error(format!("ordinary source archive: {message}"))
}

pub(super) fn node_limit(archive: bool) -> usize {
    crate::translation::frontend_rules::ml_array::number_index(
        rules::read_source_definition_catalog_node_limit(archive),
    )
    .expect("registered source archive limits are exact host indices")
}

impl Directory {
    pub(super) fn compose(&self) -> Result<Closure<'_>> {
        if self.re_exports != "resolved named module providers 1" {
            return Err(failure("the actual named module provider profile is required"));
        }
        let modules = self
            .modules
            .iter()
            .map(|module| (module.path.as_str(), module.source.as_str()))
            .collect::<Vec<_>>();
        compose_with_re_exports(&self.entry_path, &modules, &self.re_exports).map_err(failure)
    }
}

impl Decoder<'_> {
    pub(super) fn read_source_archive(&mut self, root: LinkId) -> Result<Option<Directory>> {
        let label = rules::read_ordinary_source_archive_field().to_string();
        let mut directories = self
            .fields
            .get(&root)
            .into_iter()
            .flatten()
            .filter(|(field, _)| field == &label);
        let Some((_, id)) = directories.next() else {
            return Ok(None);
        };
        let id = *id;
        if directories.next().is_some() || !self.record_shapes || self.network.len() > node_limit(true) {
            return Err(failure("one bounded ordinary source module directory is required"));
        }
        let qualified = self.network.links().any(|relation| {
            let metadata = relation.metadata();
            metadata.link_type() == Some(LinkType::Field)
                && !metadata.is_named()
                && metadata.language().is_none()
                && metadata.term().is_none()
                && metadata.definition().is_none()
                && matches!(relation.references(), [parent, field, child]
                    if *parent == root && *child == id && !self.fields.contains_key(field)
                        && self.network.link(*field).is_some_and(|point|
                            qualified_field(point) && point.metadata().term() == Some(label.as_str())))
        });
        if !qualified {
            return Err(failure("the original source module directory relation is invalid"));
        }
        let previous = self.limits;
        self.limits.node_limit = node_limit(false).min(previous.node_limit);
        self.read(id, 0)?;
        if !self.references.is_empty() {
            return Err(failure("a source module directory contains a semantic reference"));
        }
        let mut remaining = self.limits.node_limit;
        let mut bytes = 16 * 1024 * 1024;
        let value = self.render(id, &mut remaining, &mut bytes)?;
        let directory = Directory::deserialize(&value).map_err(failure)?;
        self.values.clear();
        self.visited = 0;
        self.limits = previous;
        Ok(Some(directory))
    }

    fn source_offset(&self, id: LinkId, field: &str) -> Option<usize> {
        let Data::Scalar(value) = self.values.get(&self.field(id, field)?)? else {
            return None;
        };
        let number = value.as_f64()?;
        if !accept_source_span_offset(number, number == 0.0 && number.is_sign_negative()) {
            return None;
        }
        crate::translation::frontend_rules::ml_array::number_index(number)
    }

    fn archive_owner<'closure, 'source>(
        &self,
        id: LinkId,
        name: &str,
        archive: &'closure Closure<'source>,
        source: &Source,
    ) -> Result<&'closure Module<'source>> {
        let span = self.field(id, "span").and_then(|span| {
            Some(Span::new(self.source_offset(span, "start")?, self.source_offset(span, "end")?))
        });
        let Some(span) = span.filter(|span| span.start <= span.end) else {
            return Err(failure(format!("declaration {name} needs its actual source interval")));
        };
        let owner = archive.modules.iter().find(|module| {
            module.range.start <= span.start && span.start < module.range.end && span.end <= module.range.end
        }).ok_or_else(|| failure(format!("declaration {name} needs a represented module owner")))?;
        let generated = self
            .field(id, "generated")
            .and_then(|flag| self.values.get(&flag))
            .is_some_and(|flag| matches!(flag, Data::Scalar(Value::Bool(true))));
        match self.kind(id) {
            "data" if generated || has_source_library_typedef(&source.slice(span.start, span.end), span, name) => {}
            "fn" if generated || owner.declarations.contains_key(name) => {}
            _ => return Err(failure(format!("declaration {name} has no actual original module owner"))),
        }
        Ok(owner)
    }

    pub(super) fn validate_source_archive(
        &self,
        body: LinkId,
        declarations: &BTreeMap<String, LinkId>,
        archive: &Closure<'_>,
    ) -> Result<()> {
        let nonempty = |record, label| self.field(record, label).is_some_and(|id| {
            !matches!(self.values.get(&id), Some(Data::Array(values)) if values.is_empty())
        });
        if self.tag(body, "sourceLanguage") != "JavaScript"
            || nonempty(body, "imports")
            || nonempty(body, "externals")
            || self.field(body, "main").is_some_and(|main| nonempty(main, "effects"))
        {
            return Err(failure("a closed ordinary JavaScript source library is required"));
        }
        let source = Source::new(&archive.source);
        let owners = declarations.iter().map(|(name, id)| {
            Ok((*id, self.archive_owner(*id, name, archive, &source)?))
        }).collect::<Result<HashMap<_, _>>>()?;
        let mut pending = vec![(body, None)];
        let mut units: HashMap<Option<&str>, HashSet<LinkId>> = HashMap::new();
        let mut total = 0;
        let unit_limit = node_limit(false);
        let aggregate_limit = node_limit(true);
        while let Some((id, current)) = pending.pop() {
            if matches!(self.values.get(&id), Some(Data::Scalar(_))) {
                continue;
            }
            let owner = owners.get(&id).copied().or(current);
            let unit = units.entry(owner.map(|module| module.path)).or_default();
            if !unit.insert(id) {
                continue;
            }
            total += 1;
            if unit.len() > unit_limit || total > aggregate_limit {
                return Err(failure(format!("source ownership exceeds its structural resource limit in {}",
                    owner.map_or("the program", |module| module.path))));
            }
            if let Some(owner) = owner
                && matches!(self.kind(id), "call" | "functionRef")
                && let Some(references) = self.references.get(&id)
            {
                for (label, target) in references {
                    if read_program_reference_source_field(label, self.kind(id), self.reference_type_kind(id)) != "fn" {
                        continue;
                    }
                    if owners.get(target).is_none_or(|module| module.path != owner.path) {
                        let name = self.full_name(*target)?;
                        if !owner.imports.contains(&name) {
                            return Err(failure(format!("{} cannot capture {name} from another module", owner.path)));
                        }
                    }
                }
            }
            match self.values.get(&id) {
                Some(Data::Record(fields)) => pending.extend(fields.values().map(|id| (*id, owner))),
                Some(Data::Array(children)) => pending.extend(children.iter().map(|id| (*id, owner))),
                // Retain the structural pair records as well as their members.
                Some(Data::Map(_)) => {
                    let link = self.network.link(id).ok_or_else(|| failure("an actual module value is missing"))?;
                    pending.extend(link.references().iter().map(|id| (*id, owner)));
                }
                Some(Data::Scalar(_)) => {}
                None => return Err(failure("an actual module value is missing")),
            }
        }
        Ok(())
    }
}
