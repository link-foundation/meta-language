//! Header-owner capture around the existing type parser, never a second parser.
use super::super::{Result, RustParser, SourceOrigin, TokenKind, Type, unsupported};
use crate::translation::source_signatures::{SourceOwner, SourceSignature, SourceTypeSignature};
use serde_json::Value;
use std::sync::LazyLock;

static POLICY: LazyLock<Value> = LazyLock::new(|| {
    crate::read_structured_data_lino(include_str!("../../../data/rust-source-signatures.lino"))
        .expect("the generated Rust source signature policy is valid")
});

fn owner(kind: &str, path: Vec<String>, arguments: Vec<SourceOwner>) -> SourceOwner {
    SourceOwner {
        kind: kind.to_owned(),
        path,
        arguments,
    }
}

impl RustParser {
    fn source_type_head(&self) -> Vec<String> {
        let first = self.cursor.peek().value.clone();
        if first == "&" || first == "(" {
            let mut head = vec![first.clone()];
            if first == "&" {
                let offset = if self.cursor.is_at("'", 1) { 3 } else { 1 };
                if self.cursor.is_at("[", offset) {
                    head.push("[".to_owned());
                }
            }
            return head;
        }
        let mut path = vec![first];
        let mut offset = 1;
        while self.cursor.is_at("::", offset)
            && self.cursor.is_kind_at(TokenKind::Identifier, offset + 1)
        {
            path.push(self.cursor.peek_at(offset + 1).value.clone());
            offset += 2;
        }
        path
    }

    fn source_type_owner(
        &self,
        head: &[String],
        children: Vec<SourceOwner>,
        ty: &Type,
    ) -> SourceOwner {
        if head[0] == "&" {
            if head.iter().any(|part| part == "[") {
                return owner("slice", Vec::new(), children);
            }
            if let Some(child) = children.into_iter().next() {
                return child;
            }
            return self.named_source_type_owner(&["str".to_owned()], Vec::new());
        }
        if head[0] == "(" {
            return match ty {
                Type::Unit => owner("unit", Vec::new(), Vec::new()),
                Type::Tuple { .. } => owner("tuple", Vec::new(), children),
                _ => children
                    .into_iter()
                    .next()
                    .expect("a grouped type has a parsed child"),
            };
        }
        self.named_source_type_owner(head, children)
    }

    pub(in super::super) fn named_source_type_owner(
        &self,
        head: &[String],
        children: Vec<SourceOwner>,
    ) -> SourceOwner {
        let resolved = self.resolve(head, &self.module_path);
        let declared = self
            .origins
            .lookup_type_owner(head, &self.module_path)
            .or_else(|| self.origins.lookup_type_owner(&resolved, &self.module_path));
        if let Some(declared) = declared {
            return owner(
                "declaration",
                declared.full_name.split('.').map(str::to_owned).collect(),
                children,
            );
        }
        let libraries = POLICY["libraries"]
            .as_object()
            .expect("library owner policy is a mapping");
        let library = libraries.get(&head.join("::")).or_else(|| {
            libraries.values().find(|path| {
                path.as_array().is_some_and(|segments| {
                    segments
                        .iter()
                        .filter_map(Value::as_str)
                        .eq(resolved.iter().map(String::as_str))
                })
            })
        });
        if let Some(library) = library {
            return owner(
                "library",
                library
                    .as_array()
                    .expect("library path is ordered")
                    .iter()
                    .map(|part| part.as_str().expect("a library segment is text").to_owned())
                    .collect(),
                children,
            );
        }
        if head.len() == 1 && POLICY["primitives"].get(&head[0]).is_some() {
            return owner("primitive", head.to_vec(), children);
        }
        owner("declaration", resolved, children)
    }

    pub(in super::super) fn parse_type(&mut self) -> Result<Type> {
        // Readonly dispatch also needs actual owners; publishing the captured
        // header facet remains independently opt-in.
        let head = self.source_type_head();
        self.source_owner_frames.push(Vec::new());
        self.primitive_owner_frames.push(Vec::new());
        self.type_namespace_frames.push(Vec::new());
        let result = self.parse_type_value();
        let children = self
            .source_owner_frames
            .pop()
            .expect("the source type frame is owned");
        let namespace_children = self
            .type_namespace_frames
            .pop()
            .expect("the type namespace frame is owned");
        let ty = result?;
        if !self.shared_array_handle_source_model.is_empty()
            && !self.origins.current_type.wrappers.is_empty()
        {
            return Err(unsupported(
                "Rust shared array handle source",
                "borrowed or boxed source type wrappers need an independent composition proof",
                None,
            ));
        }
        let primitive_children = self
            .primitive_owner_frames
            .pop()
            .expect("the primitive owner frame is owned");
        let primitive = self
            .read_standard_primitive_position(&head, &self.module_path)?
            .or_else(|| {
                ((head[0] == "&" || head[0] == "(") && primitive_children.len() == 1)
                    .then(|| primitive_children[0].clone())
                    .flatten()
            });
        self.origins
            .current_type
            .primitive_owner_position
            .clone_from(&primitive);
        let owner = if primitive.is_some() {
            SourceOwner {
                kind: "primitive".to_owned(),
                path: vec!["f64".to_owned()],
                arguments: Vec::new(),
            }
        } else {
            self.source_type_owner(&head, children, &ty)
        };
        self.origins.current_type.source_owner = Some(owner.clone());
        let namespace_spelling = if head[0] != "&" && head[0] != "(" {
            head
        } else if namespace_children.len() == 1 && !head.iter().any(|part| part == "[") {
            namespace_children[0].clone()
        } else if head[0] == "&" && !head.iter().any(|part| part == "[") {
            vec!["str".to_owned()]
        } else {
            Vec::new()
        };
        self.origins
            .current_type
            .namespace_spelling
            .clone_from(&namespace_spelling);
        if let Some(parent) = self.type_namespace_frames.last_mut() {
            parent.push(namespace_spelling);
        }
        if let Some(parent) = self.source_owner_frames.last_mut() {
            parent.push(owner);
        }
        if let Some(parent) = self.primitive_owner_frames.last_mut() {
            parent.push(primitive);
        }
        Ok(ty)
    }

    pub(super) fn capture_source_signature(
        &self,
        parameters: &[SourceOrigin],
        result: &SourceOrigin,
        lifetime_bindings: Vec<String>,
        scalar_loop: bool,
    ) -> Result<Option<SourceSignature>> {
        if self.source_signature_model.is_empty() && !scalar_loop {
            return Ok(None);
        }
        let position = |origin: &SourceOrigin| -> Result<SourceTypeSignature> {
            Ok(SourceTypeSignature {
                ty: origin.ty.clone().ok_or_else(|| {
                    unsupported(
                        "Rust source signature",
                        "a parsed header needs its value type",
                        None,
                    )
                })?,
                wrappers: origin.wrappers.clone(),
                lifetimes: origin.source_lifetimes.clone(),
                owner: origin.source_owner.clone().ok_or_else(|| {
                    unsupported(
                        "Rust source signature",
                        "a parsed header needs its actual type owner",
                        None,
                    )
                })?,
            })
        };
        Ok(Some(SourceSignature {
            model: if self.source_signature_model.is_empty() {
                "Rust source function signature 1".into()
            } else {
                self.source_signature_model.clone()
            },
            language: "Rust".to_owned(),
            parameters: parameters.iter().map(position).collect::<Result<_>>()?,
            result: position(result)?,
            lifetime_bindings,
        }))
    }
}
