//! Existing pattern constraints, relocated without changing their bodies.

use super::{BOOL, Env, Inference, Result, SPattern, SPatternNode, STRING, Span, Term, Type};

impl Inference {
    pub(super) fn pattern(
        &mut self,
        pattern: &SPattern,
        ty: &Term,
        scope: &mut Env,
        place: Option<Span>,
    ) -> Result<()> {
        match &pattern.node {
            SPatternNode::Alias {
                name,
                pattern: inner,
            } => {
                scope.insert(name.clone(), ty.clone());
                self.pattern(inner, ty, scope, place)
            }
            SPatternNode::Ctor { path, args } => {
                let (name, tag) = match path.as_slice() {
                    [.., name, tag] => (name.clone(), tag.clone()),
                    _ => return Ok(()),
                };
                self.unify(ty, &Term::Known(Type::Data { name: name.clone() }), place)?;
                let count = self
                    .data
                    .iter()
                    .find(|(data, _)| *data == name)
                    .and_then(|(_, ctors)| ctors.iter().find(|ctor| ctor.name == tag))
                    .map_or(0, |ctor| ctor.fields.len());
                for (index, arg) in args.iter().enumerate() {
                    let term = if index < count {
                        self.field(&name, &tag, index)
                    } else {
                        self.fresh(false)
                    };
                    self.pattern(arg, &term, scope, place)?;
                }
                Ok(())
            }
            SPatternNode::BindOrCtor { name } | SPatternNode::Bind { name } => {
                scope.insert(name.clone(), ty.clone());
                Ok(())
            }
            SPatternNode::NumLit { .. } => {
                let literal = self.fresh(true);
                self.unify(ty, &literal, place)
            }
            SPatternNode::TypedNumLit {
                ty: literal_type, ..
            } => self.unify(ty, &Term::Known(literal_type.clone()), place),
            SPatternNode::ByteLit { .. } => self.unify(
                ty,
                &Term::Known(Type::Fixed {
                    bits: 8,
                    signed: false,
                }),
                place,
            ),
            SPatternNode::BoolLit { .. } => self.unify(ty, &Term::Known(BOOL), place),
            SPatternNode::StrLit { .. } => self.unify(ty, &Term::Known(STRING), place),
            _ => Ok(()),
        }
    }
}
