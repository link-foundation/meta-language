//! Parser-owned facts transported to the normally generated shared decisions.
use super::{
    BOOL, Options, Result, RustParser, SExpr, SItem, SMain, SNode, SourceDeclaration, SourceOrigin,
    Span, Token, Type, unsupported,
};
use crate::translation::rust_shared_array_handle_rules as rules;
use std::collections::HashSet;
const SOURCE_MODEL: &str = "standard shared array handle";

#[derive(Default)]
pub(super) struct SharedArrayState {
    pub(super) payload_depth: usize,
    // Each range belongs to an occurrence in this parser's owned lexer stream,
    // independent of variable spelling; erased borrows retain that occurrence.
    names: HashSet<Span>,
    borrows: HashSet<Span>,
    accepted_names: HashSet<Span>,
    accepted_borrows: HashSet<Span>,
}
fn failure(reason: &str, span: Option<Span>) -> super::TranslationError {
    unsupported("Rust shared array handle source", reason, span)
}
impl RustParser {
    pub(super) fn shared_array_origin(&self, origin: &SourceOrigin) -> bool {
        !self.shared_array_handle_source_model.is_empty()
            && origin.proven
            && origin.array_source_model == Some(SOURCE_MODEL)
            && origin.ty.as_ref().is_some_and(|ty| {
                matches!(
                    ty,
                    Type::Array {
                        reference: true,
                        ..
                    }
                )
            })
            && origin.wrappers.is_empty()
    }
    fn shared_array_owner(&self, segments: &[String], role: &str, path: &[String]) -> bool {
        let resolved = self.resolve(segments, path);
        let shadowed = segments.first().is_some_and(|first| {
            self.origins
                .lookup(std::slice::from_ref(first), path)
                .is_some()
                || self.origins.scope.contains_key(first)
        }) || resolved.first().is_some_and(|first| {
            self.origins
                .lookup(std::slice::from_ref(first), path)
                .is_some()
        });
        rules::accept_rust_shared_array_handle_layer(role, &resolved.join("::"), shadowed)
    }
    pub(super) fn shared_array_type(
        &mut self,
        segments: &[String],
        token: &Token,
    ) -> Result<Option<Type>> {
        if self.shared_array_handle_source_model.is_empty() || segments.join("::") != "std::rc::Rc"
        {
            return Ok(None);
        }
        let path = self.module_path.clone();
        if !self.shared_array_owner(segments, "handle", &path) {
            return Err(failure(
                "the outer type requires the actual unshadowed standard Rc owner",
                Some(token.span()),
            ));
        }
        self.cursor.expect("<", Some("shared array handle"))?;
        let cell_token = self.peek();
        let cell = self.path_segments("shared array cell type")?;
        if !self.shared_array_owner(&cell, "cell", &path) {
            return Err(failure(
                "the payload requires the actual standard RefCell owner",
                Some(cell_token.span()),
            ));
        }
        self.cursor.expect("<", Some("shared array cell"))?;
        let vector_token = self.peek();
        let vector = self.path_segments("shared array vector type")?;
        let resolved = self.resolve(&vector, &path);
        let shadowed = vector.first().is_some_and(|first| {
            self.origins
                .lookup(std::slice::from_ref(first), &path)
                .is_some()
        }) || resolved.first().is_some_and(|first| {
            self.origins
                .lookup(std::slice::from_ref(first), &path)
                .is_some()
        });
        if !rules::accept_rust_source_array_owner(&resolved.join("::"), shadowed) {
            return Err(failure(
                "the cell requires its actual standard Vec payload",
                Some(vector_token.span()),
            ));
        }
        self.cursor.expect("<", Some("shared array vector"))?;
        let element = self.parse_type()?;
        if !rules::accept_rust_shared_array_handle_payload(
            element.kind(),
            !self.origins.current_type.wrappers.is_empty(),
        ) || element.is_reference()
        {
            return Err(failure(
                "the vector needs direct admitted scalar elements",
                Some(vector_token.span()),
            ));
        }
        self.cursor.expect(">", Some("shared array vector"))?;
        self.cursor.expect(">", Some("shared array cell"))?;
        self.cursor.expect(">", Some("shared array handle"))?;
        let ty = self.origins.typed(
            Type::Array {
                element: Box::new(element),
                reference: true,
            },
            &path,
            Vec::new(),
        );
        self.origins.current_type.array_source_model = Some(SOURCE_MODEL);
        Ok(Some(ty))
    }
    pub(super) fn refuse_shared_bare_vector(&self, span: Option<Span>) -> Result<()> {
        if !self.shared_array_handle_source_model.is_empty() {
            return Err(failure(
                "a bare value Vec or slice requires an independent composition proof",
                span,
            ));
        }
        Ok(())
    }
    pub(super) fn shared_array_name(&mut self, node: &SExpr, path: &[String]) -> Result<()> {
        if self.shared_array_handle_source_model.is_empty() {
            return Ok(());
        }
        let SNode::Name { path: segments } = &node.node else {
            return Ok(());
        };
        let origin = self.origins.origin(node, path);
        if segments.len() == 1
            && self.origins.scope.contains_key(&segments[0])
            && self.shared_array_origin(&origin)
        {
            self.shared_array_state
                .names
                .insert(node.span.ok_or_else(|| {
                    failure("a handle occurrence needs its actual source span", None)
                })?);
        }
        if self
            .origins
            .lookup(segments, path)
            .is_some_and(|entry| matches!(entry.declaration, SourceDeclaration::Function { .. }))
            && !segments
                .first()
                .is_some_and(|name| self.origins.scope.contains_key(name))
        {
            return Err(failure(
                "a function value needs an actual call, not implicit invocation",
                node.span,
            ));
        }
        Ok(())
    }
    pub(super) fn shared_array_borrow(&mut self, node: &SExpr, path: &[String]) -> Result<()> {
        if self.shared_array_handle_source_model.is_empty() {
            return Ok(());
        }
        let origin = self.origins.origin(node, path);
        if !self.origins.collecting
            && !(origin.array_source_model == Some(SOURCE_MODEL)
                && origin.ty.as_ref().is_some_and(Type::is_reference)
                && origin.wrappers == ["reference"])
        {
            return Err(failure(
                "only one immutable handle borrow for actual Rc::ptr_eq is admitted",
                node.span,
            ));
        }
        self.shared_array_state.borrows.insert(
            node.span
                .ok_or_else(|| failure("a borrow needs its actual source occurrence", None))?,
        );
        Ok(())
    }
    fn accept_shared_name(&mut self, node: &SExpr) {
        if let Some(span) = node.span
            && self.shared_array_state.names.contains(&span)
        {
            self.shared_array_state.accepted_names.insert(span);
        }
    }
    pub(super) fn shared_array_method(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        args: &[SExpr],
        range: Option<Span>,
    ) -> Result<Option<SExpr>> {
        if self.shared_array_handle_source_model.is_empty() {
            return Ok(None);
        }
        let origin = self.origins.origin(receiver, &self.module_path);
        let actual = self.shared_array_origin(&origin);
        if method.value != "clone" {
            if !self.origins.collecting && origin.array_source_model == Some(SOURCE_MODEL) {
                return Err(failure(
                    "payload methods are outside the actual shared handle source profile",
                    range,
                ));
            }
            return Ok(None);
        }
        let form = rules::read_rust_shared_array_handle_source_form(
            "clone",
            f64::from(u32::try_from(args.len()).unwrap_or(u32::MAX)),
            true,
            actual,
            false,
            !origin.wrappers.is_empty(),
            false,
            false,
        );
        if form.is_empty() {
            if self.origins.collecting {
                return Ok(None);
            }
            return Err(failure(
                "only the actual owned shared handle can clone as an alias",
                range,
            ));
        }
        self.accept_shared_name(receiver);
        let mut result = receiver.clone();
        result.span = Some(Span::new(
            receiver.span.map_or(method.start, |span| span.start),
            self.cursor.tokens[self.cursor.index - 1].end,
        ));
        self.origins.mark(&result, origin);
        Ok(Some(result))
    }
    pub(super) fn shared_array_constructor(
        &mut self,
        segments: &[String],
        start: &Token,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        if self.shared_array_handle_source_model.is_empty()
            || segments.join("::") != "std::rc::Rc::new"
        {
            return Ok(None);
        }
        if !self.shared_array_owner(&segments[..segments.len() - 1], "handle", path) {
            return Err(failure(
                "the constructor needs the actual unshadowed Rc owner",
                Some(start.span()),
            ));
        }
        self.cursor.expect("(", Some("shared handle constructor"))?;
        let cell_token = self.peek();
        let cell = self.path_segments("shared cell constructor")?;
        if cell.last().is_none_or(|name| name != "new")
            || !self.shared_array_owner(&cell[..cell.len().saturating_sub(1)], "cell", path)
        {
            return Err(failure(
                "one actual RefCell::new layer is required",
                Some(cell_token.span()),
            ));
        }
        self.cursor.expect("(", Some("shared cell constructor"))?;
        if self.cursor.peek().kind != super::TokenKind::Macro || self.cursor.peek().value != "vec" {
            return Err(failure(
                "the one cell argument must be a fresh nonempty vec literal",
                Some(self.cursor.peek().span()),
            ));
        }
        self.shared_array_state.payload_depth += 1;
        let payload = self.expr(path, Options::default());
        self.shared_array_state.payload_depth -= 1;
        let mut payload = payload?;
        self.cursor.expect(")", Some("shared cell constructor"))?;
        self.cursor.expect(")", Some("shared handle constructor"))?;
        let origin = self.origins.origin(&payload, path);
        let fresh = matches!(&payload.node,SNode::Array {items,..} if !items.is_empty()&&items.iter().all(|item|!item.spread));
        let element = origin.ty.as_ref().and_then(Type::element).cloned();
        let accepted = element.as_ref().is_some_and(|element| {
            rules::accept_rust_shared_array_handle_payload(element.kind(), false)
        });
        if rules::read_rust_shared_array_handle_source_form(
            "allocate",
            1.0,
            true,
            false,
            false,
            false,
            false,
            fresh && accepted,
        )
        .is_empty()
        {
            return Err(failure(
                "only one complete fresh scalar vec literal constructs a shared array handle",
                Some(start.span()),
            ));
        }
        payload.span = Some(Span::new(
            start.start,
            self.cursor.tokens[self.cursor.index - 1].end,
        ));
        let mut origin = SourceOrigin::with_type(
            Type::Array {
                element: Box::new(element.expect("accepted payload element")),
                reference: true,
            },
            path,
            Vec::new(),
        );
        origin.array_source_model = Some(SOURCE_MODEL);
        self.origins.mark(&payload, origin);
        Ok(Some(payload))
    }
    pub(super) fn shared_array_literal(&self, node: &SExpr) -> Result<()> {
        if !self.shared_array_handle_source_model.is_empty()
            && self.shared_array_state.payload_depth != 1
        {
            return Err(failure(
                "value array literals are admitted only as the one owned shared-handle payload",
                node.span,
            ));
        }
        Ok(())
    }
    fn shared_array_associated_clone(
        &mut self,
        segments: &[String],
        args: &[SExpr],
        path: &[String],
        range: Option<Span>,
    ) -> Result<Option<SExpr>> {
        if args.len() != 1 {
            return Err(failure(
                "associated clone needs exactly one argument",
                range,
            ));
        }
        let arg = &args[0];
        let mut origin = self.origins.origin(arg, path);
        let handle = origin.proven
            && origin.array_source_model == Some(SOURCE_MODEL)
            && origin.ty.as_ref().is_some_and(Type::is_reference);
        let borrowed = origin.wrappers == ["reference"]
            && arg
                .span
                .is_some_and(|span| self.shared_array_state.borrows.contains(&span));
        let admitted = rules::accept_rust_shared_array_handle_borrowed_clone(
            &self.shared_array_handle_source_model,
            f64::from(u32::try_from(args.len()).unwrap_or(u32::MAX)),
            self.shared_array_owner(&segments[..segments.len() - 1], "handle", path),
            handle,
            borrowed,
        );
        if !admitted && !self.origins.collecting {
            return Err(failure(
                "associated clone requires source model 2 and one actual immutable borrowed standard shared handle",
                range,
            ));
        }
        self.accept_shared_name(arg);
        let span = arg
            .span
            .ok_or_else(|| failure("a clone argument needs its actual source occurrence", range))?;
        self.shared_array_state.accepted_borrows.insert(span);
        if origin
            .wrappers
            .first()
            .is_some_and(|wrapper| wrapper == "reference")
        {
            origin.wrappers.remove(0);
        }
        self.origins.mark(arg, origin.clone());
        let mut node = arg.clone();
        node.span =
            range.map(|span| Span::new(span.start, self.cursor.tokens[self.cursor.index - 1].end));
        self.origins.mark(&node, origin);
        Ok(Some(node))
    }
    pub(super) fn shared_array_call(
        &mut self,
        segments: &[String],
        args: &[SExpr],
        path: &[String],
        range: Option<Span>,
    ) -> Result<Option<SExpr>> {
        if !self.shared_array_handle_source_model.is_empty()
            && segments.join("::") == "std::rc::Rc::clone"
        {
            return self.shared_array_associated_clone(segments, args, path, range);
        }
        if self.shared_array_handle_source_model.is_empty()
            || segments.join("::") != "std::rc::Rc::ptr_eq"
        {
            return Ok(None);
        }
        let origins = args
            .iter()
            .map(|arg| self.origins.origin(arg, path))
            .collect::<Vec<_>>();
        let handles = origins
            .iter()
            .map(|origin| {
                origin.proven
                    && origin.array_source_model == Some(SOURCE_MODEL)
                    && origin.ty.as_ref().is_some_and(Type::is_reference)
            })
            .collect::<Vec<_>>();
        let borrowed = args.len() == 2
            && origins.iter().zip(args).all(|(origin, arg)| {
                origin.wrappers == ["reference"]
                    && arg
                        .span
                        .is_some_and(|span| self.shared_array_state.borrows.contains(&span))
            });
        let same = origins.len() == 2 && origins[0].ty.is_some() && origins[0].ty == origins[1].ty;
        let form = rules::read_rust_shared_array_handle_source_form(
            "pointer equality",
            f64::from(u32::try_from(args.len()).unwrap_or(u32::MAX)),
            self.shared_array_owner(&segments[..segments.len() - 1], "handle", path),
            handles.first() == Some(&true),
            handles.get(1) == Some(&true),
            borrowed,
            same,
            false,
        );
        if form.is_empty() && !self.origins.collecting {
            return Err(failure(
                "ptr_eq needs two actual immutable borrowed shared handles in the same complete domain",
                range,
            ));
        }
        if args.len() != 2 {
            return Err(failure("ptr_eq needs exactly two arguments", range));
        }
        for (arg, origin) in args.iter().zip(origins) {
            self.accept_shared_name(arg);
            let span = arg.span.ok_or_else(|| {
                failure(
                    "a pointer argument needs its actual source occurrence",
                    range,
                )
            })?;
            self.shared_array_state.accepted_borrows.insert(span);
            let mut origin = origin;
            if origin
                .wrappers
                .first()
                .is_some_and(|wrapper| wrapper == "reference")
            {
                origin.wrappers.remove(0);
                self.origins.mark(arg, origin);
            }
        }
        let span =
            range.map(|span| Span::new(span.start, self.cursor.tokens[self.cursor.index - 1].end));
        let node = SExpr::new(
            SNode::ReferenceIdentity {
                op: super::BinaryOp::Eq,
                left: Box::new(args[0].clone()),
                right: Box::new(args[1].clone()),
                extra: std::collections::BTreeMap::new(),
            },
            span,
        );
        self.origins
            .mark(&node, SourceOrigin::with_type(BOOL, path, Vec::new()));
        Ok(Some(node))
    }
    pub(super) fn refuse_shared_value_observation(
        &self,
        left: &SExpr,
        right: Option<&SExpr>,
        path: &[String],
        span: Option<Span>,
    ) -> Result<()> {
        if self.shared_array_handle_source_model.is_empty() || self.origins.collecting {
            return Ok(());
        }
        let left = self.origins.origin(left, path);
        let right = right.map(|value| self.origins.origin(value, path));
        if !rules::accept_rust_shared_array_handle_value_observation(
            left.array_source_model == Some(SOURCE_MODEL),
            right
                .as_ref()
                .is_some_and(|origin| origin.array_source_model == Some(SOURCE_MODEL)),
        ) {
            return Err(failure(
                if right.is_some() {
                    "ordinary Rust comparison observes payloads; only actual Rc::ptr_eq is admitted"
                } else {
                    "Display of a shared handle needs an independent payload observation model"
                },
                span,
            ));
        }
        Ok(())
    }
    pub(super) fn finish_shared_array_source(
        &self,
        items: &[SItem],
        main: Option<&SMain>,
    ) -> Result<()> {
        if self.shared_array_handle_source_model.is_empty() {
            return Ok(());
        }
        if items.is_empty() && main.is_none()
            || items.iter().any(|item| !matches!(item, SItem::Fn(_)))
            || main.is_some_and(|main| {
                main.effects
                    .iter()
                    .any(|effect| matches!(effect, super::SEffect::Assert { .. }))
            })
            || !self.aliases.is_empty()
            || !self.crate_declarations.is_empty()
        {
            return Err(failure(
                "this first profile requires closed flat functions with standard fully qualified owners",
                None,
            ));
        }
        for span in &self.shared_array_state.names {
            if !self.shared_array_state.accepted_names.contains(span) {
                return Err(failure(
                    "a shared handle source use needs its actual explicit clone or ptr_eq borrow",
                    Some(*span),
                ));
            }
        }
        for span in &self.shared_array_state.borrows {
            if !self.shared_array_state.accepted_borrows.contains(span) {
                return Err(failure(
                    "a handle borrow cannot escape the actual ptr_eq call",
                    Some(*span),
                ));
            }
        }
        Ok(())
    }
}
