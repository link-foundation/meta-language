//! Reconstruct scoped storage revisions from actual checked source transitions.
use super::{Position, Regions, Result, Value, accept, fail, rules};
use crate::translation::source_loop_folds::{RustSourceStateTransition, SourceStateBinder};
use std::rc::Rc;

#[derive(Clone)]
pub(super) struct Storage {
    pub(super) binder: SourceStateBinder,
    pub(super) position: Rc<Position>,
    pub(super) revision: usize,
}

#[derive(Clone)]
pub(super) struct StorageValue {
    pub(super) identity: String,
    pub(super) revision: usize,
    pub(super) owns_scope: bool,
    pub(super) transport: bool,
}

impl Regions<'_> {
    pub(super) fn apply_storage_transition(
        &mut self,
        value: Value,
        facet: &RustSourceStateTransition,
    ) -> Result<Value> {
        let position = &facet.position;
        self.closed_output(position)?;
        let value = self.compatible(value, Some(position))?;
        if !value.regions.is_empty() {
            return Err(fail(
                "closed mutable roots cannot retain source reference regions",
            ));
        }
        let id = &facet.binder.identity;
        let prior = self.storages.get(id);
        let initializer = facet.operation == "initialize";
        let revision = match (initializer, prior) {
            (true, None) => None,
            (true, Some(_)) => {
                return Err(fail(
                    "a source mutable binder is initialized only by its actual declaration",
                ));
            }
            (false, Some(prior))
                if prior.binder == facet.binder && prior.position.as_ref() == position =>
            {
                accept(rules::read_rust_borrowed_source_storage_write_error(
                    &self.states[prior.revision],
                ))?;
                Some(prior.revision)
            }
            (false, _) => {
                return Err(fail(
                    "a source write retains its actual initialized binder and complete referent position",
                ));
            }
        };
        let mut value = self.read(value, "move")?;
        if let Some(revision) = revision {
            self.states[revision] =
                rules::consume_rust_borrowed_source_region(&self.states[revision]);
            self.retire_field_lifetime(revision, "move");
        }
        let revision = self.root(String::new(), false)?;
        let position = self.position(position)?;
        self.prepare_record_image(revision, &position)?;
        self.storages.insert(
            id.clone(),
            Storage {
                binder: facet.binder.clone(),
                position,
                revision,
            },
        );
        value.root = None;
        value.producer = true;
        value.storage = Some(StorageValue {
            identity: id.clone(),
            revision,
            owns_scope: initializer,
            transport: true,
        });
        Ok(value)
    }

    pub(super) fn check_storage_revision(&self, value: &Value) -> Result<()> {
        if let Some(storage) = &value.storage
            && (self.states[storage.revision] != "available"
                || !self
                    .storages
                    .get(&storage.identity)
                    .is_some_and(|current| current.revision == storage.revision))
        {
            return Err(fail(
                "a source state read retains its actual live selected storage revision",
            ));
        }
        Ok(())
    }

    pub(super) fn expire_binding(&mut self, id: usize, value: &Value) {
        self.states[id] = rules::expire_rust_borrowed_source_region(&self.states[id]);
        self.retire_field_lifetime(id, "expire");
        if let Some(storage) = &value.storage
            && storage.owns_scope
            && let Some(current) = self.storages.remove(&storage.identity)
        {
            self.states[current.revision] =
                rules::expire_rust_borrowed_source_region(&self.states[current.revision]);
            self.retire_field_lifetime(current.revision, "expire");
        }
    }
}
