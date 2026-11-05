//! Integer-domain conversions over the shared modular cast definitions.
use super::*;

impl LeanEmitter<'_> {
    pub(super) fn cast(
        &mut self,
        arg: &Expr,
        from: &Type,
        to: &Type,
        flavor: Flavor,
        depth: usize,
    ) -> Result<String> {
        let arg = self.expr(arg, depth)?;
        if flavor == Flavor::Wrapping {
            let (
                Type::Fixed {
                    signed: source_signed,
                    ..
                },
                Type::Fixed { bits, signed },
            ) = (from, to)
            else {
                return Err(unsupported(
                    "wrapping integer cast",
                    "requires actual fixed integer source and target types",
                    None,
                ));
            };
            let code =
                crate::translation::fixed_integer_cast_rules::render_wrapping_fixed_integer_cast(
                    "Lean",
                    &arg,
                    f64::from(*bits),
                    *signed,
                    *source_signed,
                );
            if code.is_empty() {
                return Err(unsupported(
                    "wrapping integer cast",
                    "the integer width needs an exact target encoding",
                    None,
                ));
            }
            self.state.encode(
                "fixed integer casts",
                &crate::translation::fixed_integer_cast_rules::read_fixed_integer_cast_statement(),
            );
            return Ok(code);
        }
        let natural_from = from.is_natural();
        let natural_to = to.is_natural();
        if natural_from == natural_to {
            return Ok(arg);
        }
        if natural_from {
            return Ok(format!("(Int.ofNat {arg})"));
        }
        if flavor == Flavor::Checked {
            unthreaded("a checked conversion to a natural");
        }
        Ok(format!("(Int.toNat {arg})"))
    }
}
