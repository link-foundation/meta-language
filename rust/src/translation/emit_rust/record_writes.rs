//! Shared construction over actual record type and value owners.
use super::{Expr, Node, Result, RustEmitter, Type, unsupported};
use crate::translation::record_write_source_interfaces as rules;

impl RustEmitter<'_> {
    pub(super) fn record_field_write(&mut self, expression: &Expr) -> Result<String> {
        let Node::Method { object, op, args } = &expression.node else {
            unreachable!("selected record write")
        };
        let field = rules::read_record_field_write_name(op);
        let Type::Record {
            fields, ordered, ..
        } = &object.ty
        else {
            unreachable!("checked record owner")
        };
        let index = fields
            .iter()
            .position(|entry| entry.name == field)
            .ok_or_else(|| {
                unsupported(
                    "shared record field assignment",
                    "the actual field owner is missing",
                    expression.span,
                )
            })?;
        self.temporaries += 1;
        let receiver = self.expr(object)?;
        let mut arguments = Vec::new();
        for argument in args {
            let value = argument.as_ref().ok_or_else(|| {
                unsupported(
                    "shared record field assignment",
                    "the actual operand is missing",
                    expression.span,
                )
            })?;
            arguments.push(self.expr(value)?);
        }
        let initial = serde_json::Number::from(self.temporaries)
            .as_f64()
            .expect("a bounded temporary count is a Number");
        let suffix =
            rules::read_record_field_write_suffix(initial, &receiver, &arguments).to_string();
        self.temporaries = suffix
            .parse()
            .expect("the selected suffix is an actual bounded integer");
        self.state.encode(
            "write-record-field",
            &rules::read_record_field_write_statement(),
        );
        if fields[index].optional {
            let numeric = if *ordered {
                fields.iter().map(|field| {
                    crate::translation::javascript_literal_property_rules::read_java_script_literal_property_index(
                        &field.name.encode_utf16().map(f64::from).collect::<Vec<_>>(),
                    )
                }).collect::<Vec<_>>()
            } else {
                Vec::new()
            };
            let position = serde_json::Number::from(index)
                .as_f64()
                .expect("an actual field index");
            let followers = rules::read_optional_record_field_order_followers(&numeric, position)
                .iter()
                .map(|field| format!("{field:.1}"))
                .collect::<Vec<_>>();
            return Ok(rules::render_optional_rust_record_field_write(
                &receiver,
                &index.to_string(),
                &arguments[0],
                &suffix,
                *ordered,
                &followers,
            ));
        }
        Ok(rules::render_record_field_write(
            "Rust",
            &rules::read_record_field_write_operator(op),
            &receiver,
            "",
            &if *ordered {
                format!("0.{index}")
            } else {
                index.to_string()
            },
            &arguments,
            &suffix,
        ))
    }
}
