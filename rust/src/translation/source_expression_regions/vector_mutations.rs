//! Prove Vec helper calls against their actual initialized storage revisions.
use super::{Environment, Expr, FnDecl, Regions, Result, StorageValue, Value, accept, fail, rules};
use crate::translation::{source_vector_mutations, types::Type};

impl Regions<'_> {
    pub(super) fn vector_mutation_body(
        &mut self,
        function: &FnDecl,
        facet: &source_vector_mutations::RustVectorMutation,
    ) -> Result<()> {
        source_vector_mutations::validate_checked(facet, function, "Rust", |ty| match ty {
            Type::Data { name } => self.data(name),
            _ => None,
        })?;
        for position in facet
            .parameters
            .iter()
            .chain([&facet.element, &facet.result])
        {
            self.closed_output(position)?;
        }
        Ok(())
    }

    pub(super) fn vector_mutation_call(
        &mut self,
        node: &Expr,
        function: &FnDecl,
        facet: &source_vector_mutations::RustVectorMutation,
        arguments: &[Expr],
        environment: &Environment,
        bindings: &[String],
        depth: usize,
    ) -> Result<Value> {
        if !self.composed
            || arguments.len() != facet.parameters.len()
            || node.ty != function.ret
            || node.source_position.is_some()
            || node.source_state_transition.is_some()
        {
            return Err(fail(
                "a Vec mutation call retains its actual complete result and arguments",
            ));
        }
        let (receiver, operands) = arguments
            .split_first()
            .ok_or_else(|| fail("a Vec helper needs its actual receiver"))?;
        let position = &facet.parameters[0];
        let selected =
            self.expression(receiver, environment, bindings, Some(position), depth + 1)?;
        let selected = self.compatible(selected, Some(position))?;
        let actual = selected
            .storage
            .as_ref()
            .filter(|_| selected.regions.is_empty() && selected.place.is_none())
            .ok_or_else(|| fail("a Vec helper retains its actual initialized whole source root"))?;
        let identity = actual.identity.clone();
        let mut storage = self
            .storages
            .get(&identity)
            .filter(|storage| {
                storage.revision == actual.revision && storage.position.as_ref() == position
            })
            .cloned()
            .ok_or_else(|| fail("a Vec helper retains its actual live source storage revision"))?;
        let lexical = selected.root;
        self.read(selected, "read")?;
        accept(rules::read_rust_borrowed_source_storage_write_error(
            &self.states[storage.revision],
        ))?;
        self.retire_field_lifetime(storage.revision, "move");
        self.states[storage.revision] =
            rules::consume_rust_borrowed_source_region(&self.states[storage.revision]);
        if let Some(root) = lexical
            && root != storage.revision
        {
            self.states[root] = rules::consume_rust_borrowed_source_region(&self.states[root]);
        }
        // The source adapter binds any effectful operands before the helper.
        for (argument, position) in operands.iter().zip(&facet.parameters[1..]) {
            let actual =
                self.expression(argument, environment, bindings, Some(position), depth + 1)?;
            let actual = self.compatible(actual, Some(position))?;
            self.read(actual, "move")?;
        }

        let result = Value::new(
            Some(self.position(&facet.result)?),
            None,
            Vec::new(),
            true,
            None,
        );
        let position = self.position(position)?;
        let revision = self.root(String::new(), false)?;
        storage.revision = revision;
        storage.position = position.clone();
        self.storages.insert(identity.clone(), storage);
        let mut vector = Value::new(Some(position), None, Vec::new(), true, None);
        vector.storage = Some(StorageValue {
            identity,
            revision,
            owns_scope: false,
            transport: true,
        });
        self.create_packet(node.ty.clone(), vec![result, vector], None)
    }
}
