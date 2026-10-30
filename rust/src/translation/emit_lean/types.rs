//! Type spelling and representation choices from the existing Lean emitter.

use super::{LeanEmitter, Result, Type, type_error, unsupported};

impl LeanEmitter<'_> {
    pub(super) fn ty(&mut self, ty: &Type) -> Result<String> {
        if ty.is_reference() || matches!(ty, Type::Cell { .. }) {
            return Err(unsupported(
                "mutable structural value",
                "shared reference state needs an explicit target state model",
                None,
            ));
        }
        Ok(match ty {
            Type::Nat => "Nat".to_owned(),
            Type::Int => "Int".to_owned(),
            Type::Fixed { signed, .. } => {
                let represented = if *signed { "Int" } else { "Nat" };
                self.state.machine_integer(ty, represented);
                represented.to_owned()
            }
            Type::Float => {
                self.state.encode(
                    "floats",
                    "a JavaScript Number is a Lean Float, the same IEEE-754 binary64 with the same arithmetic; % is ml_float_rem, the exact truncated remainder, and ml_js_number prints a value as JavaScript does",
                );
                "Float".to_owned()
            }
            Type::Bool => "Bool".to_owned(),
            Type::String => {
                if self.utf16_strings {
                    crate::translation::formal_string_unit_rules::read_formal_string_unit_type(
                        "Lean",
                    )
                } else {
                    "String".to_owned()
                }
            }
            Type::Unit => "Unit".to_owned(),
            Type::Output => if self.utf16_strings {
                "List (Array Nat)"
            } else {
                "List String"
            }
            .to_owned(),
            Type::Option { element, absence } => {
                let element = self.ty(element)?;
                if absence == "both" {
                    format!("(Option (Option {element}))")
                } else {
                    format!("(Option {element})")
                }
            }
            Type::Data { name } => self.state.reference(name, "."),
            Type::Array { element, .. } => {
                self.state.encode(
                    "arrays",
                    "a JavaScript array, which the portable core never mutates, is a Lean Array; a read outside it panics",
                );
                format!("(Array {})", self.ty(element)?)
            }
            Type::Set { element, .. } => {
                let element = self.ty(element)?;
                crate::translation::collection_construction_rules::render_collection_construction_type("Lean", "set", &element, "")
            }
            Type::Map { key, value, .. } => {
                let key = self.ty(key)?;
                let value = self.ty(value)?;
                crate::translation::collection_construction_rules::render_collection_construction_type("Lean", "map", &key, &value)
            }
            Type::Tuple { elements } => {
                let mut parts = Vec::new();
                for element in elements {
                    parts.push(self.ty(element)?);
                }
                if parts.is_empty() {
                    "Unit".to_owned()
                } else {
                    format!("({})", parts.join(" × "))
                }
            }
            Type::Record {
                fields, ordered, ..
            } => {
                let children = fields
                    .iter()
                    .map(|field| self.ty(&field.ty))
                    .collect::<Result<Vec<_>>>()?;
                let payload = crate::translation::record_emission_rules::render_record_type(
                    "Lean", &children,
                );
                if *ordered {
                    crate::translation::ordered_record_layout_rules::render_ordered_record_type(
                        "Lean",
                        &payload,
                        &self.ty(&crate::translation::types::array(Type::Float))?,
                    )
                } else {
                    payload
                }
            }
            Type::Union { .. } => {
                return Err(type_error(
                    crate::translation::type_syntax_rules::read_union_target_refusal("Lean"),
                    None,
                ));
            }
            Type::Absence { .. } => return Err(type_error(
                crate::translation::nullable_type_syntax_rules::read_nullable_type_target_refusal(
                    "Lean",
                ),
                None,
            )),
            Type::Function { params, ret } => {
                let parameters = params
                    .iter()
                    .map(|parameter| self.ty(parameter))
                    .collect::<Result<Vec<_>>>()?;
                let ret = self.ty(ret)?;
                crate::translation::callable_source_rules::render_callable_function_type(
                    "Lean",
                    &parameters,
                    &ret,
                )
            }
            other => {
                return Err(type_error(
                    format!("no Lean type for {}", other.kind()),
                    None,
                ));
            }
        })
    }
}
