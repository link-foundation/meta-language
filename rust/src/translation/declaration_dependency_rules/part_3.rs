// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::contains_declaration_dependency;
use super::ml;
use super::ml_array;
use super::ml_read_declaration_dependency_components_loop14;
use super::read_required_number_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_dependency_components_loop13(
    starts: &[f64],
    vertices: &[f64],
    leaders: &[f64],
    mut result: Vec<f64>,
    mut node: f64,
) -> Vec<f64> {
    loop {
        if node < (starts.len() as f64) {
            {
                let at = 0f64;
                {
                    let result_2 = self::ml_read_declaration_dependency_components_loop14(
                        vertices, leaders, result, node, at,
                    );
                    {
                        let node_2 = node + 1f64;
                        {
                            (result, node) = (result_2, node_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadDeclarationDependencyComponentsLoop12Result {
    MlReadDeclarationDependencyComponentsLoop12Done(Vec<f64>, Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadDeclarationDependencyComponentsLoop16Result {
    MlReadDeclarationDependencyComponentsLoop16Done(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_dependency_components_loop16(
    mut vertices: Vec<f64>,
    mut leaders: Vec<f64>,
    mut leader: f64,
    ml_values6: &[f64],
    mut ml_index6: self::ml::Big,
) -> self::MlReadDeclarationDependencyComponentsLoop16Result {
    loop {
        if ml_index6 < (self::ml::Big::from_u128(ml_values6.len() as u128)) {
            {
                let member = self::ml_array::at(ml_values6, ml_index6.to_index());
                {
                    let vertices_2 = {
                        let (ml_append_right, ml_append_left) = (vec![member], vertices);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let leaders_2 = {
                            let (ml_append_right, ml_append_left) = (vec![leader], leaders);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let ml_index6_2 = ml_index6.add(&self::ml::Big::from_i128(1));
                            {
                                (vertices, leaders, ml_index6) =
                                    (vertices_2, leaders_2, ml_index6_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlReadDeclarationDependencyComponentsLoop16Result::MlReadDeclarationDependencyComponentsLoop16Done(vertices, leaders)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadDeclarationDependencyComponentsLoop15Result {
    MlReadDeclarationDependencyComponentsLoop15Done(Vec<f64>, Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_dependency_components_loop18(
    targets: &[f64],
    mut pending: Vec<f64>,
    mut current: f64,
    mut source: f64,
    mut end: f64,
    mut edge: f64,
) -> Vec<f64> {
    loop {
        if edge < end {
            if ((self::read_required_number_fact(targets, edge)) == current)
                && !(self::contains_declaration_dependency(&(pending), source))
            {
                {
                    let pending_2 = {
                        let (ml_append_right, ml_append_left) = (vec![source], pending);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let edge_2 = edge + 1f64;
                        {
                            (pending, edge) = (pending_2, edge_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let edge_3 = edge + 1f64;
                    {
                        edge = edge_3;
                        continue;
                    }
                }
            }
        }
        return { pending };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_dependency_components_loop17(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    assigned: &[f64],
    mut pending: Vec<f64>,
    mut current: f64,
    mut source: f64,
) -> Vec<f64> {
    loop {
        if source < (starts.len() as f64) {
            if (self::read_required_number_fact(assigned, source)) == 1f64 {
                {
                    let source_2 = source + 1f64;
                    {
                        source = source_2;
                        continue;
                    }
                }
            }
            {
                {
                    let start = self::read_required_number_fact(starts, source);
                    {
                        let end = start + self::read_required_number_fact(counts, source);
                        {
                            let edge = start;
                            {
                                let pending_2 =
                                    self::ml_read_declaration_dependency_components_loop18(
                                        targets, pending, current, source, end, edge,
                                    );
                                {
                                    let source_3 = source + 1f64;
                                    {
                                        (pending, source) = (pending_2, source_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { pending };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_declaration_dependency_components_loop15(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut assigned: Vec<f64>,
    mut pending: Vec<f64>,
    mut members: Vec<f64>,
    mut leader: f64,
    mut at: f64,
) -> self::MlReadDeclarationDependencyComponentsLoop15Result {
    loop {
        if at < (pending.len() as f64) {
            {
                let current = self::read_required_number_fact(&(pending), at);
                {
                    let at_2 = at + 1f64;
                    if (self::read_required_number_fact(&(assigned), current)) == 1f64 {
                        {
                            at = at_2;
                            continue;
                        }
                    }
                    {
                        {
                            let assigned_2 = self::ml_array::append(
                                self::ml_array::append(
                                    self::ml_array::slice(&assigned, 0f64, current),
                                    vec![1f64],
                                ),
                                self::ml_array::slice(&assigned, (current + 1f64), f64::INFINITY),
                            );
                            {
                                let members_2 = {
                                    let (ml_append_right, ml_append_left) =
                                        (vec![current], members);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let leader_3 = if current < leader {
                                        { current }
                                    } else {
                                        leader
                                    };
                                    {
                                        let source = 0f64;
                                        {
                                            let pending_2 = self::ml_read_declaration_dependency_components_loop17(starts, counts, targets, &(assigned_2), pending, current, source);
                                            {
                                                (assigned, pending, members, leader, at) = (
                                                    assigned_2, pending_2, members_2, leader_3,
                                                    at_2,
                                                );
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlReadDeclarationDependencyComponentsLoop15Result::MlReadDeclarationDependencyComponentsLoop15Done(assigned, members, leader)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_declaration_dependency_components_loop12(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    finished: &[f64],
    mut assigned: Vec<f64>,
    mut vertices: Vec<f64>,
    mut leaders: Vec<f64>,
    mut position: f64,
) -> self::MlReadDeclarationDependencyComponentsLoop12Result {
    loop {
        return if position > 0f64 {
            {
                let position_2 = position - 1f64;
                {
                    let root = self::read_required_number_fact(finished, position_2);
                    if (self::read_required_number_fact(&(assigned), root)) == 1f64 {
                        {
                            position = position_2;
                            continue;
                        }
                    }
                    {
                        {
                            let pending = vec![root];
                            {
                                let members = Vec::<f64>::new();
                                {
                                    let leader = root;
                                    {
                                        let at = 0f64;
                                        {
                                            let ml_s3344 = self::ml_read_declaration_dependency_components_loop15(starts, counts, targets, assigned, pending, members, leader, at);
                                            match ml_s3344 {
                                                self::MlReadDeclarationDependencyComponentsLoop15Result::MlReadDeclarationDependencyComponentsLoop15Done(assigned_2, members_2, leader_2) => {
                                                    {
                                                        let ml_values6 = members_2;
                                                        {
                                                            let ml_index6 = self::ml::Big::from_i128(0);
                                                            {
                                                                let ml_s3355 = self::ml_read_declaration_dependency_components_loop16(vertices, leaders, leader_2, &(ml_values6), ml_index6);
                                                                match ml_s3355 {
                                                                    self::MlReadDeclarationDependencyComponentsLoop16Result::MlReadDeclarationDependencyComponentsLoop16Done(vertices_2, component_representatives) => {
                                                                        {
                                                                            (assigned, vertices, leaders, position) = (assigned_2, vertices_2, component_representatives, position_2);
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadDeclarationDependencyComponentsLoop12Result::MlReadDeclarationDependencyComponentsLoop12Done(vertices, leaders)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptDeclarationModuleScopeLoop20Result {
    MlAcceptDeclarationModuleScopeLoop20Done,
    MlAcceptDeclarationModuleScopeLoop20Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_declaration_module_scope_loop20(
    ml_values19: &[String],
    mut ml_index19: self::ml::Big,
) -> self::MlAcceptDeclarationModuleScopeLoop20Result {
    loop {
        return if ml_index19 < (self::ml::Big::from_u128(ml_values19.len() as u128)) {
            {
                let form = self::ml_array::at(ml_values19, ml_index19.to_index());
                if ((((form != "comment") && (form != "named function declaration"))
                    && (form != "named import"))
                    && (form != "primitive literal binding"))
                    && (form != "empty class declaration")
                {
                    self::MlAcceptDeclarationModuleScopeLoop20Result::MlAcceptDeclarationModuleScopeLoop20Return(false)
                } else {
                    {
                        let ml_index19_2 = ml_index19.add(&self::ml::Big::from_i128(1));
                        {
                            ml_index19 = ml_index19_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlAcceptDeclarationModuleScopeLoop20Result::MlAcceptDeclarationModuleScopeLoop20Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadRecursiveDeclarationSourceOwnersLoop22Result {
    MlReadRecursiveDeclarationSourceOwnersLoop22Done(Vec<f64>, Vec<f64>),
    MlReadRecursiveDeclarationSourceOwnersLoop22Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadRecursiveDeclarationSourceOwnersJoin24 {
    MlReadRecursiveDeclarationSourceOwnersJoin24Next(Vec<f64>),
    MlReadRecursiveDeclarationSourceOwnersJoin24Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadRecursiveDeclarationSourceOwnersLoop23Result {
    MlReadRecursiveDeclarationSourceOwnersLoop23Done(f64),
    MlReadRecursiveDeclarationSourceOwnersLoop23Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_recursive_declaration_source_owners_loop23(
    root_starts: &[f64],
    root_ends: &[f64],
    mut start: f64,
    mut end: f64,
    mut role: f64,
    mut owner: f64,
    mut root: f64,
) -> self::MlReadRecursiveDeclarationSourceOwnersLoop23Result {
    loop {
        return if root < (root_starts.len() as f64) {
            {
                let root_start = self::read_required_number_fact(root_starts, root);
                {
                    let root_end = self::read_required_number_fact(root_ends, root);
                    if (start >= root_start) && (end <= root_end) {
                        if owner >= 0f64 {
                            self::MlReadRecursiveDeclarationSourceOwnersLoop23Result::MlReadRecursiveDeclarationSourceOwnersLoop23Return(Vec::<f64>::new())
                        } else {
                            {
                                let owner_2 = root;
                                if ((role == 0f64) && ((start != root_start) || (end != root_end)))
                                    || ((role != 0f64)
                                        && ((start <= root_start) || (end >= root_end)))
                                {
                                    self::MlReadRecursiveDeclarationSourceOwnersLoop23Result::MlReadRecursiveDeclarationSourceOwnersLoop23Return(Vec::<f64>::new())
                                } else {
                                    {
                                        let root_2 = root + 1f64;
                                        {
                                            (owner, root) = (owner_2, root_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let root_3 = root + 1f64;
                            {
                                root = root_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadRecursiveDeclarationSourceOwnersLoop23Result::MlReadRecursiveDeclarationSourceOwnersLoop23Done(owner)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn ml_read_recursive_declaration_source_owners_loop22(
    root_starts: &[f64],
    root_ends: &[f64],
    declaration_starts: &[f64],
    declaration_ends: &[f64],
    roles: &[f64],
    mut owners: Vec<f64>,
    mut roots: Vec<f64>,
    mut declaration: f64,
) -> self::MlReadRecursiveDeclarationSourceOwnersLoop22Result {
    loop {
        return if declaration < (roles.len() as f64) {
            {
                let start = self::read_required_number_fact(declaration_starts, declaration);
                {
                    let end = self::read_required_number_fact(declaration_ends, declaration);
                    {
                        let role = self::read_required_number_fact(roles, declaration);
                        if !((((start >= 0f64) && ((start % 1f64) == 0f64)) && (end > start))
                            && ((end % 1f64) == 0f64))
                            || (((role != 0f64) && (role != 1f64)) && (role != 2f64))
                        {
                            self::MlReadRecursiveDeclarationSourceOwnersLoop22Result::MlReadRecursiveDeclarationSourceOwnersLoop22Return(Vec::<f64>::new())
                        } else {
                            {
                                let owner = -1f64;
                                {
                                    let root = 0f64;
                                    {
                                        let ml_s3366 = self::ml_read_recursive_declaration_source_owners_loop23(root_starts, root_ends, start, end, role, owner, root);
                                        match ml_s3366 {
                                            self::MlReadRecursiveDeclarationSourceOwnersLoop23Result::MlReadRecursiveDeclarationSourceOwnersLoop23Done(owner_2) => {
                                                if owner_2 < 0f64 {
                                                    self::MlReadRecursiveDeclarationSourceOwnersLoop22Result::MlReadRecursiveDeclarationSourceOwnersLoop22Return(Vec::<f64>::new())
                                                } else {
                                                    {
                                                        let ml_s3377 = if role == 0f64 {
                                                            if self::contains_declaration_dependency(&(roots), owner_2) {
                                                                self::MlReadRecursiveDeclarationSourceOwnersJoin24::MlReadRecursiveDeclarationSourceOwnersJoin24Return(Vec::<f64>::new())
                                                            } else {
                                                                {
                                                                    let roots_2 = { let (ml_append_right, ml_append_left) = (vec![owner_2], roots); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                    self::MlReadRecursiveDeclarationSourceOwnersJoin24::MlReadRecursiveDeclarationSourceOwnersJoin24Next(roots_2)
                                                                }
                                                            }
                                                        } else {
                                                            self::MlReadRecursiveDeclarationSourceOwnersJoin24::MlReadRecursiveDeclarationSourceOwnersJoin24Next(roots)
                                                        };
                                                        match ml_s3377 {
                                                            self::MlReadRecursiveDeclarationSourceOwnersJoin24::MlReadRecursiveDeclarationSourceOwnersJoin24Next(roots_3) => {
                                                                {
                                                                    let collected_source_owners = { let (ml_append_right, ml_append_left) = (vec![owner_2], owners); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                    {
                                                                        let declaration_2 = declaration + 1f64;
                                                                        {
                                                                            (owners, roots, declaration) = (collected_source_owners, roots_3, declaration_2);
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlReadRecursiveDeclarationSourceOwnersJoin24::MlReadRecursiveDeclarationSourceOwnersJoin24Return(ml_result24) => {
                                                                self::MlReadRecursiveDeclarationSourceOwnersLoop22Result::MlReadRecursiveDeclarationSourceOwnersLoop22Return(ml_result24)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlReadRecursiveDeclarationSourceOwnersLoop23Result::MlReadRecursiveDeclarationSourceOwnersLoop23Return(ml_result24_2) => {
                                                self::MlReadRecursiveDeclarationSourceOwnersLoop22Result::MlReadRecursiveDeclarationSourceOwnersLoop22Return(ml_result24_2)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadRecursiveDeclarationSourceOwnersLoop22Result::MlReadRecursiveDeclarationSourceOwnersLoop22Done(owners, roots)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadRecursiveDeclarationSourceOwnersLoop21Result {
    MlReadRecursiveDeclarationSourceOwnersLoop21Done,
    MlReadRecursiveDeclarationSourceOwnersLoop21Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_recursive_declaration_source_owners_loop21(
    root_starts: &[f64],
    root_ends: &[f64],
    mut previous_end: f64,
    mut root: f64,
) -> self::MlReadRecursiveDeclarationSourceOwnersLoop21Result {
    loop {
        return if root < (root_starts.len() as f64) {
            {
                let start = self::read_required_number_fact(root_starts, root);
                {
                    let end = self::read_required_number_fact(root_ends, root);
                    if ((((((start >= 0f64) && ((start % 1f64) == 0f64)) && (end > start))
                        && ((end % 1f64) == 0f64))
                        && (end <= 9_007_199_254_740_991f64))
                        && (start >= previous_end))
                    {
                        {
                            let previous_end_2 = end;
                            {
                                let root_2 = root + 1f64;
                                {
                                    (previous_end, root) = (previous_end_2, root_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        self::MlReadRecursiveDeclarationSourceOwnersLoop21Result::MlReadRecursiveDeclarationSourceOwnersLoop21Return(Vec::<f64>::new())
                    }
                }
            }
        } else {
            self::MlReadRecursiveDeclarationSourceOwnersLoop21Result::MlReadRecursiveDeclarationSourceOwnersLoop21Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptRecursiveDeclarationAuxiliaryClosureLoop29Result {
    MlAcceptRecursiveDeclarationAuxiliaryClosureLoop29Done,
    MlAcceptRecursiveDeclarationAuxiliaryClosureLoop29Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_recursive_declaration_auxiliary_closure_loop31(
    owners: &[f64],
    targets: &[f64],
    mut owner: f64,
    mut pending: Vec<f64>,
    mut last: f64,
    mut edge: f64,
) -> Vec<f64> {
    loop {
        if edge < last {
            {
                let target = self::read_required_number_fact(targets, edge);
                if ((self::read_required_number_fact(owners, target)) == owner)
                    && !(self::contains_declaration_dependency(&(pending), target))
                {
                    {
                        let pending_2 = {
                            let (ml_append_right, ml_append_left) = (vec![target], pending);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let edge_2 = edge + 1f64;
                            {
                                (pending, edge) = (pending_2, edge_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let edge_3 = edge + 1f64;
                        {
                            edge = edge_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { pending };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_recursive_declaration_auxiliary_closure_loop30(
    owners: &[f64],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut owner: f64,
    mut pending: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (pending.len() as f64) {
            {
                let current = self::read_required_number_fact(&(pending), at);
                {
                    let at_2 = at + 1f64;
                    {
                        let first = self::read_required_number_fact(starts, current);
                        {
                            let last = first + self::read_required_number_fact(counts, current);
                            {
                                let edge = first;
                                {
                                    let pending_2 = self::ml_accept_recursive_declaration_auxiliary_closure_loop31(owners, targets, owner, pending, last, edge);
                                    {
                                        (pending, at) = (pending_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { pending };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptRecursiveDeclarationAuxiliaryClosureLoop32Result {
    MlAcceptRecursiveDeclarationAuxiliaryClosureLoop32Done,
    MlAcceptRecursiveDeclarationAuxiliaryClosureLoop32Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_recursive_declaration_auxiliary_closure_loop32(
    owners: &[f64],
    roles: &[f64],
    mut owner: f64,
    pending: &[f64],
    mut declaration: f64,
) -> self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop32Result {
    loop {
        return if declaration < (roles.len() as f64) {
            if ((self::read_required_number_fact(owners, declaration)) == owner)
                && !(self::contains_declaration_dependency(pending, declaration))
            {
                self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop32Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop32Return(false)
            } else {
                {
                    let declaration_2 = declaration + 1f64;
                    {
                        declaration = declaration_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop32Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop32Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_recursive_declaration_auxiliary_closure_loop29(
    owners: &[f64],
    roles: &[f64],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut root: f64,
) -> self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop29Result {
    loop {
        return if root < (roles.len() as f64) {
            if (self::read_required_number_fact(roles, root)) != 0f64 {
                {
                    let root_2 = root + 1f64;
                    {
                        root = root_2;
                        continue;
                    }
                }
            }
            {
                {
                    let owner = self::read_required_number_fact(owners, root);
                    {
                        let pending = vec![root];
                        {
                            let at = 0f64;
                            {
                                let pending_2 =
                                    self::ml_accept_recursive_declaration_auxiliary_closure_loop30(
                                        owners, starts, counts, targets, owner, pending, at,
                                    );
                                {
                                    let declaration = 0f64;
                                    {
                                        let ml_s3388 = self::ml_accept_recursive_declaration_auxiliary_closure_loop32(owners, roles, owner, &(pending_2), declaration);
                                        match ml_s3388 {
                                            self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop32Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop32Done => {
                                                {
                                                    let root_3 = root + 1f64;
                                                    {
                                                        root = root_3;
                                                        continue;
                                                    }
                                                }
                                            }
                                            self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop32Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop32Return(ml_result32) => {
                                                self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop29Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop29Return(ml_result32)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop29Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop29Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptRecursiveDeclarationAuxiliaryClosureLoop28Result {
    MlAcceptRecursiveDeclarationAuxiliaryClosureLoop28Done,
    MlAcceptRecursiveDeclarationAuxiliaryClosureLoop28Return(bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptRecursiveDeclarationAuxiliaryClosureLoop33Result {
    MlAcceptRecursiveDeclarationAuxiliaryClosureLoop33Done,
    MlAcceptRecursiveDeclarationAuxiliaryClosureLoop33Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_recursive_declaration_auxiliary_closure_loop33(
    owners: &[f64],
    roles: &[f64],
    targets: &[f64],
    mut source: f64,
    mut last: f64,
    mut at: f64,
) -> self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop33Result {
    loop {
        return if at < last {
            {
                let target = self::read_required_number_fact(targets, at);
                if ((self::read_required_number_fact(roles, target)) != 0f64)
                    && ((self::read_required_number_fact(owners, source))
                        != (self::read_required_number_fact(owners, target)))
                {
                    self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop33Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop33Return(false)
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop33Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop33Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_recursive_declaration_auxiliary_closure_loop28(
    owners: &[f64],
    roles: &[f64],
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut source: f64,
) -> self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop28Result {
    loop {
        return if source < (roles.len() as f64) {
            {
                let first = self::read_required_number_fact(starts, source);
                {
                    let last = first + self::read_required_number_fact(counts, source);
                    {
                        let at = first;
                        {
                            let ml_s3399 =
                                self::ml_accept_recursive_declaration_auxiliary_closure_loop33(
                                    owners, roles, targets, source, last, at,
                                );
                            match ml_s3399 {
                                self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop33Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop33Done => {
                                    {
                                        let source_2 = source + 1f64;
                                        {
                                            source = source_2;
                                            continue;
                                        }
                                    }
                                }
                                self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop33Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop33Return(ml_result33) => {
                                    self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop28Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop28Return(ml_result33)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop28Result::MlAcceptRecursiveDeclarationAuxiliaryClosureLoop28Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptRecursiveDeclarationAuxiliaryClosureLoop27Result {
    MlAcceptRecursiveDeclarationAuxiliaryClosureLoop27Done,
    MlAcceptRecursiveDeclarationAuxiliaryClosureLoop27Return(bool),
}
