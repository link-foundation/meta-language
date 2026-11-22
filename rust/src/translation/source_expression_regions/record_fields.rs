//! A selected field has a value revision and a separate containing lifetime.
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use super::{Environment, Position, Regions, Result, Value, accept, fail, rules};
use crate::translation::{
    ir::{Expr, Node},
    rust_owned_record_state_rules::read_rust_owned_record_state_storage_error as storage_budget,
    source_place_transforms::fields_for,
    source_record_places::{
        RustSourceRecordPlace, box_projection_depth, check_record_place, owner_view,
    },
    types::Type,
};

#[derive(Clone)]
pub(super) struct FieldPlace {
    pub(super) roots: Vec<usize>,
    pub(super) ancestors: Vec<usize>,
    pub(super) borrowed: bool,
}

pub(super) struct RecordImage {
    position: Rc<Position>,
    fields: Vec<usize>,
}

impl Regions<'_> {
    pub(super) fn field_lifetime(&mut self, root: usize) -> Result<usize> {
        if let Some(id) = self.field_lifetimes.get(&root) {
            return Ok(*id);
        }
        let id = self.root(self.lifetimes[root].clone(), false)?;
        self.field_lifetimes.insert(root, id);
        Ok(id)
    }

    pub(super) fn prepare_record_image(&mut self, root: usize, position: &Position) -> Result<()> {
        if !self.composed || !matches!(position.ty, Type::Data { .. }) {
            return Ok(());
        }
        let mut pending = vec![(root, self.position(position)?, Vec::new())];
        let mut visits = 0;
        while let Some((root, position, active)) = pending.pop() {
            visits += 1;
            accept(storage_budget(visits as f64, active.len() as f64).to_string())?;
            let Type::Data { name } = &position.ty else {
                continue;
            };
            let Some((owner, _)) = owner_view(&position, false) else {
                continue;
            };
            let Some(data) = self.data(name).filter(|data| data.ctors.len() == 1) else {
                continue;
            };
            let Some(fields) = data
                .source_field_signatures
                .as_ref()
                .and_then(|signature| signature.ctors.first())
                .filter(|entry| entry.fields.len() == data.ctors[0].fields.len())
            else {
                continue;
            };
            let mut view = position.as_ref().clone();
            view.owner = owner.clone();
            view.wrappers.clear();
            view.lifetimes.clear();
            if let Some(prior) = self.field_images.get(&root) {
                if prior.position.as_ref() != &view {
                    return Err(fail(
                        "a record field image retains its actual declaration and owner",
                    ));
                }
                continue;
            }
            if active.contains(&data.full_name) {
                return Err(fail(
                    "recursive record fields need a finite constructive storage image",
                ));
            }
            let mut active = active;
            active.push(data.full_name.clone());
            self.field_lifetime(root)?;
            let mut children = Vec::with_capacity(fields.fields.len());
            for field in &fields.fields {
                let child = self.root(self.lifetimes[root].clone(), false)?;
                self.field_lifetime(child)?;
                pending.push((child, self.position(field)?, active.clone()));
                children.push(child);
            }
            let position = self.position(&view)?;
            self.field_images.insert(
                root,
                RecordImage {
                    position,
                    fields: children,
                },
            );
        }
        Ok(())
    }

    pub(super) fn record_field_roots(&self, roots: Vec<usize>) -> Result<Vec<usize>> {
        let mut pending = roots;
        let mut roots = Vec::new();
        let mut seen = HashSet::new();
        while let Some(root) = pending.pop() {
            if !seen.insert(root) {
                continue;
            }
            accept(storage_budget(seen.len() as f64, 0.0).to_string())?;
            if root >= self.states.len() {
                return Err(fail(
                    "a field image needs its actual allocated source region",
                ));
            }
            roots.push(root);
            pending.extend(self.field_lifetimes.get(&root).copied());
            if let Some(image) = self.field_images.get(&root) {
                pending.extend(&image.fields);
            }
            if let Some(views) = self.borrowed_field_views.get(&root) {
                pending.extend(views.values().copied());
            }
        }
        Ok(roots)
    }

    pub(super) fn place_roots(value: &Value) -> Vec<usize> {
        value.place.as_ref().map_or_else(
            || {
                value
                    .storage
                    .as_ref()
                    .map(|slot| slot.revision)
                    .or(value.root)
                    .into_iter()
                    .collect()
            },
            |place| place.roots.clone(),
        )
    }

    pub(super) fn prepare_value_record_images(&mut self, value: &Value) -> Result<()> {
        let Some(position) = value.position.as_deref() else {
            return Ok(());
        };
        if !self.composed || !matches!(position.ty, Type::Data { .. }) {
            return Ok(());
        }
        if value.role == "reference" {
            let Some((owner, _)) = owner_view(position, true) else {
                return Ok(());
            };
            let Type::Data { name } = &position.ty else {
                return Ok(());
            };
            let mut view = position.clone();
            view.wrappers.clear();
            view.lifetimes.clear();
            view.owner = owner.clone();
            for parent in &value.borrow_roots {
                let root = self.borrowed_record_view(*parent, name, &view)?;
                self.prepare_record_image(root, &view)?;
            }
        } else {
            for root in Self::place_roots(value) {
                self.prepare_record_image(root, position)?;
            }
        }
        Ok(())
    }

    fn borrowed_record_view(
        &mut self,
        parent: usize,
        name: &str,
        view: &Position,
    ) -> Result<usize> {
        if self
            .field_images
            .get(&parent)
            .is_some_and(|image| image.position.as_ref() == view)
        {
            return Ok(parent);
        }
        if let Some(root) = self
            .borrowed_field_views
            .get(&parent)
            .and_then(|views| views.get(name))
        {
            return Ok(*root);
        }
        let root = self.root(self.lifetimes[parent].clone(), false)?;
        self.borrowed_field_views
            .entry(parent)
            .or_default()
            .insert(name.into(), root);
        Ok(root)
    }

    pub(super) fn check_place_image(&self, value: &Value) -> Result<()> {
        self.check_record_roots(Self::place_roots(value))
    }

    pub(super) fn check_record_roots(&self, roots: Vec<usize>) -> Result<()> {
        for root in self.record_field_roots(roots)? {
            accept(rules::read_rust_borrowed_source_read_error(
                &self.states[root],
                &[],
            ))?;
        }
        Ok(())
    }

    pub(super) fn revise_record_field(
        &mut self,
        prior: usize,
        position: &Position,
        path: &[usize],
        depth: usize,
    ) -> Result<usize> {
        accept(storage_budget((depth + 1) as f64, depth as f64).to_string())?;
        let root = self.root(self.lifetimes[prior].clone(), false)?;
        if let Some((&index, rest)) = path.split_first() {
            accept(rules::read_rust_borrowed_source_read_error(
                &self.states[prior],
                &[],
            ))?;
            let (view, mut children) = self
                .field_images
                .get(&prior)
                .map(|image| (image.position.clone(), image.fields.clone()))
                .ok_or_else(|| {
                    fail("a changed source ancestor retains its actual declaration image")
                })?;
            let (_, positions) = fields_for(&view, &mut |ty| match ty {
                Type::Data { name } => self.data(name),
                _ => None,
            })
            .ok_or_else(|| fail("a changed source ancestor retains its actual typed fields"))?;
            if children.len() != positions.len() || index >= children.len() {
                return Err(fail(
                    "a field write retains its actual declaration-order coordinate",
                ));
            }
            let lifetime = self.field_lifetime(prior)?;
            self.field_lifetimes.insert(root, lifetime);
            children[index] =
                self.revise_record_field(children[index], &positions[index], rest, depth + 1)?;
            self.field_images.insert(
                root,
                RecordImage {
                    position: view,
                    fields: children,
                },
            );
        } else {
            self.prepare_record_image(root, position)?;
            self.retire_field_lifetime(prior, "move");
        }
        self.states[prior] = rules::consume_rust_borrowed_source_region(&self.states[prior]);
        Ok(root)
    }

    pub(super) fn join_record_images(&mut self, sources: [usize; 2]) -> Result<usize> {
        self.join_record_image_pair(sources, 0, &mut 0)
    }

    fn join_record_image_pair(
        &mut self,
        [left, right]: [usize; 2],
        depth: usize,
        visits: &mut usize,
    ) -> Result<usize> {
        *visits += 3;
        accept(storage_budget(*visits as f64, depth as f64).to_string())?;
        if left == right {
            return Ok(left);
        }
        let root = self.root(self.lifetimes[left].clone(), false)?;
        match (
            self.field_lifetimes.get(&left).copied(),
            self.field_lifetimes.get(&right).copied(),
        ) {
            (Some(a), Some(b)) if a == b => {
                self.field_lifetimes.insert(root, a);
            }
            _ => {
                self.field_lifetime(root)?;
            }
        }
        match (self.field_images.get(&left), self.field_images.get(&right)) {
            (None, None) => {}
            (Some(a), Some(b)) if a.position == b.position && a.fields.len() == b.fields.len() => {
                let position = a.position.clone();
                let coordinates: Vec<_> = a
                    .fields
                    .iter()
                    .copied()
                    .zip(b.fields.iter().copied())
                    .collect();
                let fields = coordinates
                    .into_iter()
                    .map(|(a, b)| self.join_record_image_pair([a, b], depth + 1, visits))
                    .collect::<Result<_>>()?;
                self.field_images
                    .insert(root, RecordImage { position, fields });
            }
            _ => {
                return Err(fail(
                    "joined source fields retain their actual declaration and order",
                ));
            }
        }
        Ok(root)
    }

    pub(super) fn map_record_image(
        &mut self,
        source: usize,
        target: usize,
        mapping: &mut HashMap<usize, usize>,
    ) -> Result<()> {
        let mut pending = vec![(source, target)];
        let mut visits = 0;
        while let Some((source, target)) = pending.pop() {
            visits += 1;
            accept(storage_budget(visits as f64, 0.0).to_string())?;
            if let Some(prior) = mapping.insert(source, target) {
                if prior != target {
                    return Err(fail(
                        "a joined field keeps its exact corresponding storage component",
                    ));
                }
                continue;
            }
            if let Some(lifetime) = self.field_lifetimes.get(&source).copied() {
                let destination = self.field_lifetime(target)?;
                mapping.insert(lifetime, destination);
            }
            match (
                self.field_images.get(&source),
                self.field_images.get(&target),
            ) {
                (None, None) => {}
                (Some(left), Some(right))
                    if left.position == right.position
                        && left.fields.len() == right.fields.len() =>
                {
                    pending.extend(
                        left.fields
                            .iter()
                            .copied()
                            .zip(right.fields.iter().copied()),
                    );
                }
                _ => {
                    return Err(fail(
                        "a joined field image retains its complete declaration order",
                    ));
                }
            }
            for (name, root) in self
                .borrowed_field_views
                .get(&source)
                .cloned()
                .unwrap_or_default()
            {
                let position = self
                    .field_images
                    .get(&root)
                    .map(|image| image.position.clone())
                    .ok_or_else(|| fail("a borrowed field view retains its actual declaration"))?;
                let destination = self.borrowed_record_view(target, &name, &position)?;
                self.prepare_record_image(destination, &position)?;
                pending.push((root, destination));
            }
        }
        Ok(())
    }

    pub(super) fn retire_field_lifetime(&mut self, root: usize, mode: &str) {
        if let Some(id) = self.field_lifetimes.get(&root) {
            self.states[*id] = if mode == "move" {
                rules::consume_rust_borrowed_source_region(&self.states[*id])
            } else {
                rules::expire_rust_borrowed_source_region(&self.states[*id])
            };
        }
    }

    pub(super) fn project_record_place(
        &mut self,
        node: &Expr,
        facet: &RustSourceRecordPlace,
        env: &Environment,
        bindings: &[String],
        depth: usize,
    ) -> Result<Value> {
        let (facet, base) = check_record_place(
            facet,
            node,
            &self.program.source_language,
            |ty| Ok(ty.clone()),
            |ty| match ty {
                Type::Data { name } => self.data(name),
                _ => None,
            },
        )?;
        self.scope(&facet.root, bindings)?;
        self.closed_target(&facet.root)?;
        self.closed_output(&facet.value)?;
        let direct = match &base.node {
            Node::Var { name }
                if base.source_state_transition.is_none()
                    && base.source_loop_fold.is_none()
                    && base.source_record_place.is_none() =>
            {
                env.get(name)
                    .filter(|value| {
                        value.position.as_deref().is_some_and(|position| {
                            position == &facet.root
                                || position.ty == facet.root.ty
                                    && box_projection_depth(position, &facet.root)
                                        .is_some_and(|depth| depth > 0)
                        }) && base
                            .source_position
                            .as_ref()
                            .is_none_or(|fact| fact.position == facet.root)
                    })
                    .cloned()
            }
            _ => None,
        };
        let owner = if let Some(mut value) = direct {
            if let Some(position) = base.source_position.as_ref() {
                self.scope(&position.position, bindings)?;
            }
            if value.position.as_deref() != Some(&facet.root) {
                value.role = super::role(&facet.root);
                value.position = Some(self.position(&facet.root)?);
            }
            self.read_lifetime(value)?
        } else {
            let value = self.expression(base, env, bindings, Some(&facet.root), depth + 1)?;
            if base.source_record_place.is_some() {
                self.read_lifetime(value)?
            } else {
                self.read(value, "read")?
            }
        };
        let owner = self.compatible(owner, Some(&facet.root))?;
        let borrowed =
            owner.role == "reference" || owner.place.as_ref().is_some_and(|place| place.borrowed);
        let parents = if owner.role == "reference" {
            owner.borrow_roots.clone()
        } else {
            Self::place_roots(&owner)
        };
        let (leaf, _) = owner_view(&facet.root, true)
            .ok_or_else(|| fail("a source field needs its actual containing owner"))?;
        let mut view = facet.root.clone();
        view.owner = leaf.clone();
        view.wrappers.clear();
        view.lifetimes.clear();
        let mut containing = Vec::new();
        let mut roots = Vec::new();
        let Type::Data { name } = &view.ty else {
            return Err(fail("a source field needs its actual nominal record"));
        };
        for parent in parents {
            let container = if owner.role == "reference" {
                self.borrowed_record_view(parent, name, &view)?
            } else {
                parent
            };
            self.prepare_record_image(container, &view)?;
            let field = self
                .field_images
                .get(&container)
                .and_then(|image| image.fields.get(facet.field))
                .copied()
                .ok_or_else(|| {
                    fail("a field image selects its actual declaration-order storage component")
                })?;
            containing.push(self.field_lifetime(container)?);
            roots.push(field);
        }
        let root = roots
            .first()
            .copied()
            .ok_or_else(|| fail("a field needs its actual containing storage"))?;
        let ancestors = self.union([
            if owner.role == "reference" {
                owner.regions
            } else {
                owner.place.map_or_else(Vec::new, |place| place.ancestors)
            },
            containing,
        ])?;
        let mut value = Value::new(
            Some(self.position(&facet.value)?),
            Some(root),
            Vec::new(),
            owner.producer,
            None,
        );
        value.place = Some(FieldPlace {
            roots,
            ancestors,
            borrowed,
        });
        self.read_lifetime(value)
    }
}
