//! Reconstruct declared field writes from the actual lexical storage and source term.
use super::{
    Decl, Environment, Expr, Node, Position, Regions, Result, SourceOwner, StorageValue, Value,
    fail, rules,
};
use crate::translation::{
    source_place_transforms::{
        RustSourcePlaceTransform, SourcePlaceOperation, SourcePlaceSlot, check_place_transform,
        plain,
    },
    types::Type,
};

impl Regions<'_> {
    pub(super) fn state_expression(
        &mut self,
        node: &Expr,
        env: &Environment,
        bindings: &[String],
        expected: Option<&Position>,
        depth: usize,
    ) -> Result<Value> {
        if plain(node)
            && let Some(name) = node.var_name()
            && let Some(value) = env.get(name).filter(|value| {
                value.storage.is_some()
                    && value
                        .position
                        .as_ref()
                        .is_some_and(|position| position.ty == node.ty)
            })
        {
            self.visits += 1;
            self.budget(depth)?;
            return self.compatible(self.read_lifetime(value.clone())?, expected);
        }
        self.expression(node, env, bindings, expected, depth)
    }

    pub(super) fn place_transform(
        &mut self,
        node: &Expr,
        facet: &RustSourcePlaceTransform,
        env: &Environment,
        bindings: &[String],
        depth: usize,
    ) -> Result<Value> {
        let checked = check_place_transform(
            facet,
            node,
            &self.program.source_language,
            |ty| Ok(ty.clone()),
            |ty| match ty {
                Type::Data { name } => self.data(name),
                _ => None,
            },
            |name| {
                self.program
                    .declarations
                    .iter()
                    .find_map(|declaration| match declaration {
                        Decl::Fn(function) if function.full_name == name => Some(function),
                        _ => None,
                    })
            },
        )?;
        if !self.composed || &checked != facet {
            return Err(fail(
                "a field write needs its actual complete checked source place",
            ));
        }
        if let RustSourcePlaceTransform::Call { slots, places, .. } = &checked {
            return self.projected_vector_mutation(
                node,
                &slots[0],
                &places[0].fields,
                env,
                bindings,
                depth,
            );
        }
        let RustSourcePlaceTransform::Replacement {
            operation,
            slots,
            places,
            operands,
            ..
        } = checked
        else {
            return Err(fail("a field write needs its actual source operation"));
        };
        let slot = &slots[0];
        self.scope(&slot.position, bindings)?;
        self.closed_output(&slot.position)?;
        let path = &places[0].fields;
        let assignment = operation == SourcePlaceOperation::AssignField;
        let (source, prior) = self.place_input(slot, env)?;
        if !assignment {
            self.check_selected_record_field(prior, path)?;
        }
        let Node::Let { value, .. } = &node.node else {
            return Err(fail(
                "a field write needs its checked source operand binding",
            ));
        };
        let shared_operand = match (operation, &value.node, operands.as_slice()) {
            (
                SourcePlaceOperation::AssignField | SourcePlaceOperation::AppendVectorElement,
                _,
                [operand],
            ) => {
                let input = if operation == SourcePlaceOperation::AppendVectorElement {
                    match &value.node {
                        Node::Append { right, .. } => match &right.node {
                            Node::Array { items } => &items[0],
                            _ => return Err(fail("a Vec replacement retains its checked element")),
                        },
                        _ => return Err(fail("a Vec replacement retains its checked append")),
                    }
                } else {
                    value.as_ref()
                };
                self.scope(&operand.position, bindings)?;
                let actual =
                    self.expression(input, env, bindings, Some(&operand.position), depth + 1)?;
                let actual = self.read(actual, "move")?;
                if !actual.regions.is_empty() {
                    return Err(fail(
                        "a closed source field write cannot retain a borrowed operand",
                    ));
                }
                None
            }
            (SourcePlaceOperation::AppendStringText, Node::Binary { right, .. }, [operand]) => {
                self.scope(&operand.position, bindings)?;
                self.closed_target(&operand.position)?;
                let actual =
                    self.expression(right, env, bindings, Some(&operand.position), depth + 1)?;
                Some(self.read(actual, "read")?)
            }
            (SourcePlaceOperation::ClearString, _, []) => None,
            _ => return Err(fail("a field replacement retains its checked operand")),
        };
        self.read_lifetime(source.clone())?;
        if !assignment {
            self.check_selected_record_field(prior, path)?;
        }
        let updated = self.update_place(slot, path, source.root, prior)?;
        if let Some(mut actual) = shared_operand {
            actual.root = None;
            actual.producer = true;
            self.read(actual, "read")?;
        }

        let unit = self.position(&Position {
            ty: Type::Unit,
            wrappers: Vec::new(),
            lifetimes: Vec::new(),
            owner: SourceOwner {
                kind: "unit".into(),
                path: Vec::new(),
                arguments: Vec::new(),
            },
            array_element_position: None,
        })?;
        let result = Value::new(
            Some(unit),
            Some(self.root(String::new(), false)?),
            Vec::new(),
            true,
            None,
        );
        self.create_packet(node.ty.clone(), vec![result, updated], None)
    }

    fn place_input(&mut self, slot: &SourcePlaceSlot, env: &Environment) -> Result<(Value, usize)> {
        let source = env
            .get(&slot.input)
            .cloned()
            .ok_or_else(|| fail("a field write needs its actual enclosing lexical binder"))?;
        let source = self.read_lifetime(source)?;
        let source = self.compatible(source, Some(&slot.position))?;
        let identity = &slot.binder.identity;
        let current = self.storages.get(identity).filter(|storage| {
            storage.binder == slot.binder && storage.position.as_ref() == &slot.position
                && source.regions.is_empty()
                && source.storage.as_ref().is_some_and(|actual| {
                    &actual.identity == identity && actual.revision == storage.revision
                })
        }).ok_or_else(|| fail("a field write retains its actual initialized binder and current closed storage revision"))?;
        Ok((source, current.revision))
    }

    fn update_place(
        &mut self,
        slot: &SourcePlaceSlot,
        path: &[usize],
        lexical: Option<usize>,
        prior: usize,
    ) -> Result<Value> {
        let revision = self.revise_record_field(prior, &slot.position, path, 0)?;
        if let Some(root) = lexical.filter(|root| *root != prior) {
            self.states[root] = rules::consume_rust_borrowed_source_region(&self.states[root]);
        }
        let identity = &slot.binder.identity;
        self.storages
            .get_mut(identity)
            .ok_or_else(|| fail("a field write retains its original source storage"))?
            .revision = revision;
        let mut updated = Value::new(
            Some(self.position(&slot.position)?),
            None,
            Vec::new(),
            true,
            None,
        );
        updated.storage = Some(StorageValue {
            identity: identity.clone(),
            revision,
            owns_scope: false,
            transport: true,
        });
        Ok(updated)
    }

    fn projected_vector_mutation(
        &mut self,
        node: &Expr,
        slot: &SourcePlaceSlot,
        path: &[usize],
        environment: &Environment,
        bindings: &[String],
        depth: usize,
    ) -> Result<Value> {
        let Node::Let { value, .. } = &node.node else {
            return Err(fail("a projected Vec helper needs its checked bound call"));
        };
        let Node::Call { func, args } = &value.node else {
            return Err(fail("a projected Vec helper needs its checked call"));
        };
        let helper = self
            .program
            .declarations
            .iter()
            .find_map(|declaration| match declaration {
                Decl::Fn(function) if &function.full_name == func => {
                    function.source_vector_mutation.as_ref()
                }
                _ => None,
            })
            .ok_or_else(|| fail("a projected Vec helper needs its actual selected declaration"))?;
        self.scope(&slot.position, bindings)?;
        self.closed_output(&slot.position)?;
        let (source, prior) = self.place_input(slot, environment)?;
        self.check_selected_record_field(prior, path)?;
        for (argument, parameter) in args.iter().zip(&helper.parameters).skip(1) {
            let actual =
                self.expression(argument, environment, bindings, Some(parameter), depth + 1)?;
            let actual = self.compatible(actual, Some(parameter))?;
            self.read(actual, "move")?;
        }
        self.read_lifetime(source.clone())?;
        self.check_selected_record_field(prior, path)?;
        let updated = self.update_place(slot, path, source.root, prior)?;
        self.closed_output(&helper.result)?;
        let result = Value::new(
            Some(self.position(&helper.result)?),
            None,
            Vec::new(),
            true,
            None,
        );
        self.create_packet(node.ty.clone(), vec![result, updated], None)
    }
}
