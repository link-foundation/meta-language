//! Actual source declarations and checked expressions into the shared evaluator.
use super::{
    Result, RustParser, SExpr, SFn, SItem, SNode, SProgram, SourceDeclaration, SourceOrigin, Span,
    Token, TokenKind, Type, type_error, unsupported,
};
use crate::translation::ir::Decl;
use crate::translation::surface_reference_facts::create_surface_reference_facts;
use crate::translation::{Language, rust_source_constant_evaluation_rules as rules};
use serde_json::{Value, json};
use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Clone)]
pub(super) struct ConstantInitializer {
    pub(super) owner: String,
    pub(super) path: Vec<String>,
    pub(super) keyword: usize,
    pub(super) result: SourceOrigin,
}
fn number(value: usize) -> f64 {
    u32::try_from(value).map_or(f64::INFINITY, f64::from)
}
fn domain(origin: &SourceOrigin) -> Result<String> {
    match &origin.ty {
        Some(Type::Fixed { .. }) => {
            let owner = origin.pointer_integer_owner();
            if crate::translation::types::rust_fixed_type(&owner).is_some() {
                return Ok(owner);
            }
        }
        Some(Type::Bool) => return Ok("bool".to_owned()),
        Some(Type::String)
            if origin.text_source_model == Some("borrowed str")
                && origin
                    .wrappers
                    .first()
                    .is_some_and(|value| value == "reference") =>
        {
            return Ok("str".to_owned());
        }
        _ => {}
    }
    Err(unsupported(
        "Rust source constant pattern",
        "the declaration needs its actual structural primitive source type",
        None,
    ))
}
impl RustParser {
    pub(super) fn capture_source_constant_initializer(
        &mut self,
        name: &str,
        path: &[String],
        start: &Token,
        result: &SourceOrigin,
    ) -> Result<()> {
        let kinds = self
            .cursor
            .tokens
            .iter()
            .map(|token| {
                if token.kind == TokenKind::Punct {
                    "punct".to_owned()
                } else {
                    "literal".to_owned()
                }
            })
            .collect::<Vec<_>>();
        let values = self
            .cursor
            .tokens
            .iter()
            .map(|token| token.value.clone())
            .collect::<Vec<_>>();
        let boundary = rules::read_rust_source_constant_initializer_boundary(
            &kinds,
            &values,
            number(self.cursor.index),
        );
        let Some(boundary) = crate::translation::frontend_rules::ml_array::number_index(boundary)
        else {
            return Err(Self::fail(
                "a source constant initializer needs its complete declaration",
                self.cursor.peek(),
            ));
        };
        let mut owner = path.to_vec();
        owner.push(name.to_owned());
        let owner = owner.join(".");
        self.source_constant_initializers.insert(
            owner.clone(),
            ConstantInitializer {
                owner,
                path: path.to_vec(),
                keyword: start.start,
                result: result.clone(),
            },
        );
        if self.header_discovery {
            self.cursor.index = boundary + 1;
        }
        Ok(())
    }
    fn read_source_constant_initializer(
        &mut self,
        row: &ConstantInitializer,
    ) -> Result<(SFn, SourceOrigin, Self)> {
        if self.source_constant_read_stack.contains(&row.owner) {
            return Err(type_error(
                "Rust source constant pattern: a constant pattern dependency cannot evaluate itself recursively",
                None,
            ));
        }
        self.source_constant_read_stack.insert(row.owner.clone());
        let result = (|| {
            let origins = super::SourceOrigins::new(Some(self.origins.catalog.clone()), "");
            let mut reader = Self::new(
                self.cursor.tokens.clone(),
                origins,
                super::SourceProfiles::default(),
                false,
                self.original_source.clone(),
            );
            reader.source_version_minor = self.source_version_minor;
            reader.retain_source_type_owners = self.retain_source_type_owners;
            reader.aliases = self.aliases.clone();
            reader.named_records = self.named_records.clone();
            reader.inherent_methods = self.inherent_methods.clone();
            reader.origins.nominal_copy_owners = self.origins.nominal_copy_owners.clone();
            reader.origins.nominal_copy_derivations = self.origins.nominal_copy_derivations.clone();
            reader.source_constant_initializers = self.source_constant_initializers.clone();
            reader.source_constant_read_stack = self.source_constant_read_stack.clone();
            reader.initialize_type_aliases()?;
            reader.finish_type_aliases()?;
            reader.module_path = row.path.clone();
            reader.cursor.index = reader
                .cursor
                .tokens
                .iter()
                .position(|token| token.start == row.keyword)
                .ok_or_else(|| {
                    Self::fail(
                        "the constant initializer needs its actual source declaration",
                        self.cursor.peek(),
                    )
                })?;
            let declaration = reader.const_item(&row.path)?;
            let result = reader
                .source_constant_initializers
                .get(&row.owner)
                .expect("the actual parsed constant retains its header")
                .result
                .clone();
            Ok((declaration, result, reader))
        })();
        self.source_constant_read_stack.remove(&row.owner);
        result
    }
    pub(super) fn read_source_constant_pattern(
        &mut self,
        path: &[String],
    ) -> Result<Option<super::SPattern>> {
        if !self.ordinary_completion_context() {
            return Ok(None);
        }
        let start = self.peek();
        let mut offset = 0;
        let mut segments = Vec::new();
        while self.cursor.peek_at(offset).kind == TokenKind::Identifier {
            segments.push(self.cursor.peek_at(offset).value.clone());
            offset += 1;
            if !self.cursor.is_at("::", offset) {
                break;
            }
            if self.cursor.peek_at(offset + 1).kind != TokenKind::Identifier {
                return Ok(None);
            }
            offset += 1;
        }
        if segments.is_empty() || self.cursor.is_at("@", offset) {
            return Ok(None);
        }
        let Some(entry) = self
            .origins
            .lookup(&self.source_value_path(&segments, path), path)
        else {
            return Ok(None);
        };
        if !matches!(entry.declaration, SourceDeclaration::Constant { .. }) {
            return Ok(None);
        }
        let span = Some(Span {
            start: start.start,
            end: self.cursor.peek_at(offset - 1).end,
        });
        let pattern = self.evaluate_source_constant_pattern(&entry.full_name, span)?;
        self.cursor.index += offset;
        Ok(Some(pattern))
    }
    fn evaluate_source_constant_pattern(
        &mut self,
        owner: &str,
        span: Option<Span>,
    ) -> Result<super::SPattern> {
        let mut pending = vec![owner.to_owned()];
        let mut records = BTreeMap::<String, (String, SFn)>::new();
        let mut owners_by_span = HashMap::<Span, String>::new();
        let mut at = 0;
        while at < pending.len() {
            let name = pending[at].clone();
            at += 1;
            if records.contains_key(&name) {
                continue;
            }
            let row = self
                .source_constant_initializers
                .get(&name)
                .cloned()
                .ok_or_else(|| {
                    unsupported(
                        "Rust source constant pattern",
                        "the actual constant initializer must be retained",
                        span,
                    )
                })?;
            let (mut declaration, result, reader) = self.read_source_constant_initializer(&row)?;
            let mut body = serde_json::to_value(&declaration.body)
                .expect("actual source expression serializes");
            let mut dependencies = Vec::new();
            project(
                &mut body,
                &reader,
                &row.path,
                &HashSet::new(),
                &mut dependencies,
                &mut owners_by_span,
            );
            declaration.body = serde_json::from_value(body)
                .expect("source name qualification preserves expression structure");
            declaration.name = name.clone();
            records.insert(name, (domain(&result)?, declaration));
            for dependency in dependencies {
                if !records.contains_key(&dependency) {
                    pending.push(dependency);
                }
            }
            if records.len() > 65536 {
                return Err(unsupported(
                    "Rust source constant pattern",
                    "constant pattern evaluation exceeds its source resource budget",
                    span,
                ));
            }
        }
        let surface = SProgram {
            reference_semantics: None,
            source_runtime_shell: None,
            source_pure_unit_calls: None,
            language: Language::Rust,
            items: records
                .values()
                .map(|(_, function)| SItem::Fn(function.clone()))
                .collect(),
            main: None,
            imports: Vec::new(),
            externals: Vec::new(),
        };
        let checked = crate::translation::check::check_program(&surface)?;
        let mut constants = Vec::new();
        for declaration in &checked.declarations {
            if let Decl::Fn(function) = declaration {
                let mut body =
                    serde_json::to_value(&function.body).expect("checked expression serializes");
                retain(&mut body, &owners_by_span);
                constants.push(
                    json!({"owner":function.name,"domain":records[&function.name].0,"body":body}),
                );
            }
        }
        let packet = json!({"constants":constants});
        let facts = create_surface_reference_facts(&packet);
        let value = rules::evaluate_rust_source_constant(
            &facts.texts,
            &facts.starts,
            &facts.counts,
            &facts.labels,
            &facts.targets,
            facts.root,
            owner,
        );
        let kind = value.first().map_or("", String::as_str);
        if kind == "error" {
            return Err(type_error(
                format!("Rust source constant pattern: {}", value[1]),
                span,
            ));
        }
        let node = match kind {
            "integer" => {
                let ty =
                    crate::translation::types::rust_fixed_type(&value[1]).ok_or_else(|| {
                        type_error(
                            "a constant pattern lost its actual integer source owner",
                            span,
                        )
                    })?;
                let negative = value[2].starts_with('-');
                super::SPatternNode::TypedNumLit {
                    value: value[2].strip_prefix('-').unwrap_or(&value[2]).to_owned(),
                    ty,
                    owner: value[1].clone(),
                    negative,
                }
            }
            "boolean" => super::SPatternNode::BoolLit {
                value: value[2] == "true",
                negative: false,
            },
            "string" => super::SPatternNode::StrLit {
                value: value[2].clone(),
            },
            _ => {
                return Err(unsupported(
                    "Rust source constant pattern",
                    "the value needs its structural source pattern interpretation",
                    span,
                ));
            }
        };
        Ok(super::SPattern { node, span })
    }
}
fn project(
    value: &mut Value,
    reader: &RustParser,
    path: &[String],
    locals: &HashSet<String>,
    dependencies: &mut Vec<String>,
    owners: &mut HashMap<Span, String>,
) {
    if let Some(values) = value.as_array_mut() {
        for value in values {
            project(value, reader, path, locals, dependencies, owners);
        }
        return;
    }
    let Some(fields) = value.as_object_mut() else {
        return;
    };
    let kind = fields
        .get("k")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_owned();
    let name = fields
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_owned();
    for (key, child) in fields.iter_mut() {
        if key == "body" && kind == "let" {
            let mut scope = locals.clone();
            scope.insert(name.clone());
            project(child, reader, path, &scope, dependencies, owners);
        } else {
            project(child, reader, path, locals, dependencies, owners);
        }
    }
    if kind == "name" {
        let names = fields["path"]
            .as_array()
            .expect("actual name path")
            .iter()
            .map(|value| value.as_str().expect("source name segment").to_owned())
            .collect::<Vec<_>>();
        if !(names.len() == 1 && locals.contains(&names[0]))
            && let Some(entry) = reader
                .origins
                .lookup(&reader.source_value_path(&names, path), path)
            && matches!(entry.declaration, SourceDeclaration::Constant { .. })
        {
            let names = std::iter::once("crate".to_owned())
                .chain(entry.full_name.split('.').map(str::to_owned))
                .collect::<Vec<_>>();
            fields.insert("path".to_owned(), json!(names));
            dependencies.push(entry.full_name);
        }
    }
    if kind == "num" || kind == "cast" {
        let expression: SExpr = serde_json::from_value(Value::Object(fields.clone()))
            .expect("actual scalar expression");
        let origin = reader.origins.origin(&expression, path);
        let owner = origin.pointer_integer_owner();
        if crate::translation::types::rust_fixed_type(&owner).is_some()
            && let Some(span) = expression.span
        {
            owners.insert(span, owner);
        }
    }
}
fn retain(value: &mut Value, owners: &HashMap<Span, String>) {
    if let Some(values) = value.as_array_mut() {
        for value in values {
            retain(value, owners);
        }
        return;
    }
    let Some(fields) = value.as_object_mut() else {
        return;
    };
    if let Some(span) = fields
        .get("span")
        .and_then(|value| serde_json::from_value::<Span>(value.clone()).ok())
        && let Some(owner) = owners.get(&span)
    {
        fields.insert("sourceIntegerOwner".to_owned(), json!(owner));
    }
    for child in fields.values_mut() {
        retain(child, owners);
    }
}
