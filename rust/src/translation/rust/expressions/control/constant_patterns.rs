//! Actual standard primitive constant expressions into typed literal patterns.
use super::{Result, RustParser, SNode, SPattern, SPatternNode};
use crate::translation::diagnostics::type_error;
use crate::translation::surface::UnaryOp;
use crate::translation::types::Type;
impl RustParser {
    pub(super) fn primitive_constant_pattern(
        &mut self,
        path: &[String],
    ) -> Result<Option<SPattern>> {
        if !self.ordinary_completion_context() {
            return Ok(None);
        }
        let Some(value) = self.read_primitive_integer_constant(path)? else {
            return Ok(None);
        };
        let origin = self.origins.origin(&value, path);
        let owner = origin.pointer_integer_owner();
        let (literal, negative) = if let SNode::Unary {
            op: UnaryOp::Neg,
            arg,
        } = &value.node
        {
            (arg.as_ref(), true)
        } else {
            (&value, false)
        };
        let SNode::Num {
            value: digits,
            ty: Some(ty),
            ..
        } = &literal.node
        else {
            return Err(type_error(
                "a primitive constant pattern needs its actual checked integer value and source owner",
                value.span,
            ));
        };
        if !matches!(ty, Type::Fixed { .. }) || owner.is_empty() {
            return Err(type_error(
                "a primitive constant pattern needs its actual checked integer value and source owner",
                value.span,
            ));
        }
        Ok(Some(SPattern {
            node: SPatternNode::TypedNumLit {
                value: digits.clone(),
                ty: ty.clone(),
                owner,
                negative,
            },
            span: value.span,
        }))
    }
}
