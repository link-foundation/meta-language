//! Extra source identity and its public projection, as load.js classifies it.

use super::super::program::{Expr, Name, Target};

pub(super) fn extra_source(expression: &Expr) -> (&Expr, Option<Name>, bool) {
    let mut source = expression;
    let mut projection_name = None;
    loop {
        match source {
            Expr::Alias { name, item } => {
                if projection_name.is_none() {
                    projection_name = Some(name.clone());
                }
                source = item;
            }
            Expr::LexicalScope { item, .. }
            | Expr::Capture { item, .. }
            | Expr::Precedence { item, .. }
            | Expr::DynamicPrecedence { item, .. }
            | Expr::LexicalPrecedence { item, .. }
            | Expr::Token(item)
            | Expr::ImmediateToken(item) => source = item,
            _ => break,
        }
    }
    // An anonymous terminal extra is padding; reference aliases retain
    // their source identity and public name after lexing.
    if !matches!(source, Expr::Ref(_))
        && projection_name
            .as_ref()
            .is_some_and(|name| name.starts_with('\''))
    {
        projection_name = None;
    }
    // The private scanner extra's grouping declares an anonymous public
    // projection, while its scanner identity remains intact for recovery.
    if projection_name.is_none()
        && let Expr::Seq(items) = source
        && items.len() == 1
        && matches!(items[0], Expr::Ref(Target::External(_)))
    {
        return (&items[0], None, true);
    }
    (source, projection_name, false)
}
