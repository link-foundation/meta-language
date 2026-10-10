//! Type checking for portable string predicate expressions.
use super::{
    BOOL, Checker, Env, Expr, Node, Result, SExpr, SNode, STRING, type_error, unsupported,
};

impl Checker {
    pub(super) fn string_map(&mut self, node: &SExpr, env: &Env, path: &[String]) -> Result<Expr> {
        let SNode::StringMap { op, object } = &node.node else {
            unreachable!("string map dispatch")
        };
        let string = self.expr(object, env, path, None, true)?;
        if string.ty != STRING {
            return Err(unsupported(
                &format!("method call .{op}()"),
                &format!(".{op}() is portable on strings, not on {}", string.ty.key()),
                node.span,
            ));
        }
        Ok(Expr::new(
            Node::StringMap {
                op: op.clone(),
                string: Box::new(string),
            },
            STRING,
        ))
    }

    pub(super) fn string_test(&mut self, node: &SExpr, env: &Env, path: &[String]) -> Result<Expr> {
        let SNode::StringTest { op, object, search } = &node.node else {
            unreachable!("string predicate dispatch")
        };
        let span = node.span;
        let string = self.expr(object, env, path, None, true)?;
        if string.ty != STRING {
            return Err(unsupported(
                &format!("method call .{op}()"),
                &format!(".{op}() is portable on strings, not on {}", string.ty.key()),
                span,
            ));
        }
        let search_value = self.expr(search, env, path, Some(&STRING), true)?;
        let search_value = self.coerce(search_value, &STRING, search.span)?;
        if search_value.ty != STRING {
            return Err(type_error(
                format!(".{op}() of {}", search_value.ty.key()),
                search.span,
            ));
        }
        Ok(Expr::new(
            Node::StringTest {
                op: op.clone(),
                string: Box::new(string),
                search: Box::new(search_value),
            },
            BOOL,
        ))
    }
}
