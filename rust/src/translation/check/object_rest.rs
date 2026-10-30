//! Selected actual typed fields and source order into the existing record IR.
use super::{Checker, Env, Expr, Node, Result, SArrayItem, Span, Type, array, unsupported};
use crate::translation::ir::RecordEntry;
use crate::translation::object_rest_rules as rules;
impl Checker {
    pub(super) fn object_rest(
        &mut self,
        args: &[SArrayItem],
        span: Option<Span>,
        env: &Env,
        path: &[String],
    ) -> Result<Expr> {
        let (source, excluded) =
            crate::translation::object_rest::operands(args, span, self.language.as_str())?;
        let value = self.expr(&source, env, path, None, false)?;
        let value = Self::reference_value(value);
        let Type::Record {
            fields,
            open,
            ordered,
            ..
        } = &value.ty
        else {
            return Err(unsupported(
                "object rest receiver",
                "every actual enumerable own field and its source order must be represented",
                span,
            ));
        };
        if rules::read_java_script_object_rest_owner_form(
            &rules::ml_string::Text::from_text(value.ty.kind()),
            *open,
        )
        .to_string()
            != "copy"
            || !*ordered
        {
            return Err(unsupported(
                "object rest receiver",
                "every actual enumerable own field and its source order must be represented",
                span,
            ));
        }
        let names = fields
            .iter()
            .map(|field| field.name.clone())
            .collect::<Vec<_>>();
        let positions = crate::translation::object_rest::positions(&names, &excluded);
        let selected = positions
            .iter()
            .map(|position| fields[*position].clone())
            .collect::<Vec<_>>();
        let ty = Type::Record {
            fields: selected.clone(),
            ordered: true,
            open: false,
            reference: false,
        };
        let name = self.fresh_name("ml_object_rest_receiver");
        let object = Expr::new(Node::Var { name: name.clone() }, value.ty.clone());
        let entries = positions
            .iter()
            .map(|index| {
                let field = &fields[*index];
                RecordEntry {
                    name: field.name.clone(),
                    value: Expr::new(
                        Node::RecordField {
                            object: Box::new(object.clone()),
                            field: field.name.clone(),
                            index: *index,
                        },
                        field.ty.clone(),
                    ),
                }
            })
            .collect();
        let source_names = crate::translation::ordered_record_values::property_names(
            fields.iter().map(|field| field.name.as_str()),
        );
        let target_names = crate::translation::ordered_record_values::property_names(
            selected.iter().map(|field| field.name.as_str()),
        );
        let order = Expr::new(
            Node::Math {
                op: "copyJavaScriptRecordPropertyOrder".to_owned(),
                args: vec![
                    target_names,
                    source_names,
                    Expr::new(
                        Node::Method {
                            object: Box::new(object),
                            op: "readJavaScriptRecordPropertyOrder".to_owned(),
                            args: Vec::new(),
                        },
                        array(Type::Float),
                    ),
                ],
            },
            array(Type::Float),
        );
        let body = Expr::new(
            Node::Record {
                entries,
                order: Some(Box::new(order)),
            },
            ty.clone(),
        );
        Ok(Expr::new(
            Node::Let {
                name,
                value: Box::new(value),
                body: Box::new(body),
            },
            ty,
        )
        .with_span(span))
    }
}
