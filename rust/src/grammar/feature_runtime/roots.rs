//! The root of a whole-input run of the native executor: the chosen result
//! with its tokens widened over the separators they took, as the end of
//! `parse` in `js/src/grammar-runtime/executor.js`, and the ERROR leaves of
//! an input the start rule does not take.

use std::rc::Rc;

use super::executor::{Executor, Run};
use super::operations::State;
use super::program::Name;
use super::results::{Res, Tree, TreeType, children_of, concat, is_separator};

impl Executor<'_> {
    /// The rest of the input from `start` as an ERROR leaf. As tree-sitter
    /// keeps the white space at the end of the input out of an ERROR node,
    /// the separators that end the input follow the leaf; the rest costs the
    /// same.
    pub(super) fn rest_leaves(&mut self, start: usize, state: &State) -> Run<Vec<Rc<Tree>>> {
        let mut at = self.end;
        while at > start
            && matches!(
                self.bytes[at - 1],
                b'\t' | b'\n' | 0x0b | 0x0c | b'\r' | b' '
            )
        {
            at -= 1;
        }
        if at > start && at < self.end {
            let tail = self.skip_trivia(at, state)?;
            if tail.end == self.end && tail.leaves.iter().all(|leaf| is_separator(leaf)) {
                let mut leaves = vec![Rc::new(Tree::new(TreeType::Error, None, start, at))];
                leaves.extend(tail.leaves.iter().cloned());
                return Ok(leaves);
            }
        }
        Ok(vec![Rc::new(Tree::new(
            TreeType::Error,
            None,
            start,
            self.end,
        ))])
    }

    /// The root of the chosen complete `result` and its `trailing` trivia,
    /// its tokens widened over the separators they took (see
    /// `widen_tokens`).
    pub(super) fn root(
        &self,
        start_rule: usize,
        result: &Res,
        trailing: &[Rc<Tree>],
        several: bool,
    ) -> Tree {
        let ambiguous = several || result.ambiguous;
        if self.separator_taken.is_empty() {
            return self.built_root(start_rule, &result.children, trailing, ambiguous);
        }
        let children = self.widen_tokens(&result.children, &[]);
        let widened = self.widen_tokens(trailing, &[]);
        self.built_root(
            start_rule,
            children.as_deref().unwrap_or(&result.children),
            widened.as_deref().unwrap_or(trailing),
            ambiguous,
        )
    }

    fn built_root(
        &self,
        start_rule: usize,
        children: &[Rc<Tree>],
        trailing: &[Rc<Tree>],
        ambiguous: bool,
    ) -> Tree {
        match children {
            [only] if only.ty == TreeType::Node => {
                let mut root = (**only).clone();
                root.end = self.end;
                root.children = concat(&only.children, trailing);
                root.ambiguous = only.ambiguous || ambiguous;
                root
            }
            children => {
                let kind = &self.program.rules[start_rule].node_kind;
                let mut root = Tree::node(kind, self.begin, self.end, concat(children, trailing));
                root.ambiguous = ambiguous;
                root
            }
        }
    }

    /// A tree-sitter lexer skips a separator only where no valid token goes
    /// on with it: where one does, the token it lexes starts at the
    /// separator, whichever token that is (Make's ` endef` after a
    /// `raw_line`, which could take the blank). So a token after separators
    /// that a token lexed there took (see `separator_taken`) starts at the
    /// first of them, taking them, and so does each node it begins: where
    /// that token was lexed in the nodes the token is in, `ancestors`, or in
    /// nodes below them begun at the separator (Make's shell text of a recipe
    /// line). A token lexed in other nodes was lexed in another parse state
    /// (Make's `text` of a variable assignment after `VPATH =`, Rocq's
    /// comment text after `*)`). None where nothing is widened. It mirrors
    /// widenTokens in js/src/grammar-runtime/executor.js.
    fn widen_tokens(
        &self,
        children: &[Rc<Tree>],
        ancestors: &[Option<Name>],
    ) -> Option<Vec<Rc<Tree>>> {
        let mut out: Option<Vec<Rc<Tree>>> = None;
        for (index, child) in children.iter().enumerate() {
            let mut next = child.clone();
            if child.ty == TreeType::Node {
                let mut inner_ancestors = ancestors.to_vec();
                inner_ancestors.push(child.rule.clone());
                if let Some(inner) = self.widen_tokens(&child.children, &inner_ancestors) {
                    let mut copy = (**child).clone();
                    if let Some(first) = inner.iter().find(|item| !item.trivia)
                        && first.start < child.start
                    {
                        copy.start = first.start;
                    }
                    copy.children = children_of(inner);
                    next = Rc::new(copy);
                }
            } else if child.ty == TreeType::Token && !child.trivia {
                let mut from = index;
                while from > 0
                    && is_separator(&children[from - 1])
                    && children[from - 1].end == children[from].start
                {
                    from -= 1;
                }
                let taken = children[from..index].iter().position(|leaf| {
                    self.separator_taken.get(&leaf.start).is_some_and(|chains| {
                        chains.iter().any(|chain| {
                            chain.len() >= ancestors.len()
                                && ancestors
                                    .iter()
                                    .zip(chain)
                                    .all(|(name, (kind, _))| name.as_ref() == Some(kind))
                                && chain[ancestors.len()..]
                                    .iter()
                                    .all(|(_, position)| *position >= leaf.start)
                        })
                    })
                });
                if let Some(taken) = taken {
                    let kept = out.get_or_insert_with(|| children[..index].to_vec());
                    kept.truncate(kept.len() - (index - from - taken));
                    // An anonymous token keeps its name, an anonymous alias
                    // of its text.
                    let mut copy = (**child).clone();
                    copy.kind = Some(child.kind.clone().unwrap_or_else(|| {
                        let text = String::from_utf8_lossy(&self.bytes[child.start..child.end]);
                        Name::from(format!("'{text}"))
                    }));
                    copy.start = children[from + taken].start;
                    kept.push(Rc::new(copy));
                    continue;
                }
            }
            if let Some(kept) = out.as_mut() {
                kept.push(next);
            } else if !Rc::ptr_eq(&next, child) {
                let mut kept = children[..index].to_vec();
                kept.push(next);
                out = Some(kept);
            }
        }
        out
    }

    // The root when the start rule matches nothing even with repairs.
    pub(super) fn error_root(&self, start_rule: usize) -> Tree {
        let kind = &self.program.rules[start_rule].node_kind;
        let error = Tree::new(TreeType::Error, None, self.begin, self.end);
        Tree::node(
            kind,
            self.begin,
            self.end,
            children_of(vec![Rc::new(error)]),
        )
    }
}
