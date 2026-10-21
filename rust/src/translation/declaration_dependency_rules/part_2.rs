// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlReadDeclarationDependencyComponentsLoop9Result;
use super::contains_declaration_dependency;
use super::ml;
use super::ml_array;
use super::read_required_number_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_dependency_components_loop10(
    targets: &[f64],
    visited: &[f64],
    mut pending: Vec<f64>,
    mut start: f64,
    mut edge: f64,
) -> Vec<f64> {
    loop {
        if edge > start {
            {
                let edge_2 = edge - 1f64;
                {
                    let target = self::read_required_number_fact(targets, edge_2);
                    if (self::read_required_number_fact(visited, target)) == 0f64 {
                        {
                            let pending_2 = {
                                let (ml_append_right, ml_append_left) = (vec![target], pending);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                (pending, edge) = (pending_2, edge_2);
                                continue;
                            }
                        }
                    }
                    {
                        {
                            edge = edge_2;
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
pub fn ml_read_declaration_dependency_components_loop9(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut visited: Vec<f64>,
    mut finished: Vec<f64>,
    mut pending: Vec<f64>,
) -> self::MlReadDeclarationDependencyComponentsLoop9Result {
    loop {
        if (pending.len() as f64) > 0f64 {
            {
                let instruction =
                    self::read_required_number_fact(&(pending), ((pending.len() as f64) - 1f64));
                {
                    let pending_2 =
                        self::ml_array::slice(&pending, 0f64, ((pending.len() as f64) - 1f64));
                    if instruction < 0f64 {
                        {
                            let finished_2 = {
                                let (ml_append_right, ml_append_left) =
                                    (vec![((-instruction) - 1f64)], finished);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                (finished, pending) = (finished_2, pending_2);
                                continue;
                            }
                        }
                    }
                    {
                        if (self::read_required_number_fact(&(visited), instruction)) == 0f64 {
                            {
                                let visited_2 = self::ml_array::append(
                                    self::ml_array::append(
                                        self::ml_array::slice(&visited, 0f64, instruction),
                                        vec![1f64],
                                    ),
                                    self::ml_array::slice(
                                        &visited,
                                        (instruction + 1f64),
                                        f64::INFINITY,
                                    ),
                                );
                                {
                                    let pending_3 = {
                                        let (ml_append_right, ml_append_left) =
                                            (vec![((-instruction) - 1f64)], pending_2);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    {
                                        let start =
                                            self::read_required_number_fact(starts, instruction);
                                        {
                                            let edge = start
                                                + self::read_required_number_fact(
                                                    counts,
                                                    instruction,
                                                );
                                            {
                                                let pending_4 = self::ml_read_declaration_dependency_components_loop10(targets, &(visited_2), pending_3, start, edge);
                                                {
                                                    (visited, pending) = (visited_2, pending_4);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        {
                            {
                                pending = pending_2;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlReadDeclarationDependencyComponentsLoop9Result::MlReadDeclarationDependencyComponentsLoop9Done(visited, finished)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_dependency_components_loop8(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    mut visited: Vec<f64>,
    mut finished: Vec<f64>,
    mut root: f64,
) -> Vec<f64> {
    loop {
        return if root < (starts.len() as f64) {
            if (self::read_required_number_fact(&(visited), root)) == 1f64 {
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
                    let pending = vec![root];
                    {
                        let ml_s6 = self::ml_read_declaration_dependency_components_loop9(
                            starts, counts, targets, visited, finished, pending,
                        );
                        match ml_s6 {
                            self::MlReadDeclarationDependencyComponentsLoop9Result::MlReadDeclarationDependencyComponentsLoop9Done(visited_2, finished_2) => {
                                {
                                    let root_3 = root + 1f64;
                                    {
                                        (visited, finished, root) = (visited_2, finished_2, root_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            finished
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_dependency_components_loop11(
    starts: &[f64],
    mut assigned: Vec<f64>,
    mut node: f64,
) -> Vec<f64> {
    loop {
        if node < (starts.len() as f64) {
            {
                let assigned_2 = {
                    let (ml_append_right, ml_append_left) = (vec![0f64], assigned);
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let node_2 = node + 1f64;
                    {
                        (assigned, node) = (assigned_2, node_2);
                        continue;
                    }
                }
            }
        }
        return { assigned };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_declaration_dependency_components_loop14(
    vertices: &[f64],
    leaders: &[f64],
    mut result: Vec<f64>,
    mut node: f64,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (vertices.len() as f64) {
            if (self::read_required_number_fact(vertices, at)) == node {
                {
                    let result_2 = {
                        let (ml_append_right, ml_append_left) =
                            (vec![self::read_required_number_fact(leaders, at)], result);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (result, at) = (result_2, at_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let at_3 = at + 1f64;
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

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
                                            let ml_s7 = self::ml_read_declaration_dependency_components_loop15(starts, counts, targets, assigned, pending, members, leader, at);
                                            match ml_s7 {
                                                self::MlReadDeclarationDependencyComponentsLoop15Result::MlReadDeclarationDependencyComponentsLoop15Done(assigned_2, members_2, leader_2) => {
                                                    {
                                                        let ml_values6 = members_2;
                                                        {
                                                            let ml_index6 = self::ml::Big::from_i128(0);
                                                            {
                                                                let ml_s8 = self::ml_read_declaration_dependency_components_loop16(vertices, leaders, leader_2, &(ml_values6), ml_index6);
                                                                match ml_s8 {
                                                                    self::MlReadDeclarationDependencyComponentsLoop16Result::MlReadDeclarationDependencyComponentsLoop16Done(vertices_2, leaders_2) => {
                                                                        {
                                                                            (assigned, vertices, leaders, position) = (assigned_2, vertices_2, leaders_2, position_2);
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
