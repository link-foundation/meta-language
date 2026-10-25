//! Actual immutable binder facts and named projections; shared source owns key search.
use super::{
    BOOL, Checker, Env, Expr, FLOAT, Language, Node, Result, SExpr, SNode, STRING, Type,
    unsupported,
};
use crate::translation::ir::LitValue;
use crate::translation::javascript_own_property_rules as rules;
use crate::translation::surface::{BinaryOp, UnaryOp};
use crate::translation::types::option;

impl Checker {
    pub(super) fn own_property_branch_environments(
        &self,
        condition: &SExpr,
        env: &Env,
    ) -> (Env, Env) {
        let mut yes = env.clone();
        let mut no = env.clone();
        if self.language != Language::JavaScript {
            return (yes, no);
        }
        let (guard, negated) = if let SNode::Unary {
            op: UnaryOp::Not,
            arg,
        } = &condition.node
        {
            (arg.as_ref(), true)
        } else {
            (condition, false)
        };
        let SNode::Math { op, args, .. } = &guard.node else {
            return (yes, no);
        };
        let arm = rules::read_java_script_own_property_guard_arm("math", op, negated);
        if arm == 0.0 {
            return (yes, no);
        }
        let object = args.first().and_then(|argument| env.local(&argument.value));
        let key = args.get(1).and_then(|argument| env.local(&argument.value));
        let owner = object.and_then(|name| env.vars.get(name));
        let index = key.and_then(|name| env.vars.get(name));
        let count = serde_json::Number::from(args.len())
            .as_f64()
            .expect("actual source operand count is a Number");
        if !rules::accept_java_script_own_property_guard_bindings(
            count,
            args.iter().any(|argument| argument.spread),
            object.is_some(),
            key.is_some(),
            owner.map_or("", Type::kind),
            index.map_or("", Type::kind),
            matches!(owner, Some(Type::Record { open: true, .. })),
        ) {
            return (yes, no);
        }
        let pair = (
            object.expect("admitted actual object binder").to_owned(),
            key.expect("admitted actual String binder").to_owned(),
        );
        if arm == 1.0 {
            yes.own_properties.push(pair);
        } else {
            no.own_properties.push(pair);
        }
        (yes, no)
    }
    pub(super) fn guarded_record_index(
        &mut self,
        node: &SExpr,
        object: Expr,
        env: &Env,
        path: &[String],
        expected: Option<&Type>,
    ) -> Result<Option<Expr>> {
        let SNode::Index {
            object: source,
            index,
            ..
        } = &node.node
        else {
            return Ok(None);
        };
        let Type::Record { fields, open, .. } = &object.ty else {
            return Ok(None);
        };
        if !matches!(index.node, SNode::Name { .. }) {
            return Ok(None);
        }
        let key = self.expr(index, env, path, None, false)?;
        let object_name = env.local(source);
        let key_name = env.local(index);
        let guarded = object_name.zip(key_name).is_some_and(|(object, key)| {
            env.own_properties
                .iter()
                .any(|pair| pair.0 == object && pair.1 == key)
        });
        if !rules::accept_java_script_guarded_record_index(
            object.ty.kind(),
            *open,
            key.ty.kind(),
            guarded,
        ) {
            return Err(unsupported(
                "dynamic record index",
                "an actual positive Object.hasOwn branch must guard unchanged local record/String binders; inherited and exotic property domains need their complete protocol",
                node.span,
            ));
        }
        let element =
            fields
                .first()
                .map(|field| field.ty.clone())
                .unwrap_or_else(|| match expected {
                    Some(Type::Option { element, .. }) => element.as_ref().clone(),
                    Some(ty) => ty.clone(),
                    None => FLOAT,
                });
        if fields.iter().any(|field| !field.ty.same(&element)) {
            return Err(unsupported(
                "heterogeneous dynamic record index",
                "every actual field needs its shared dynamic value union",
                node.span,
            ));
        }
        let result = option(element, "undefined");
        let capture_name = self.fresh_name("ml_guarded_record_receiver");
        let text_name = self.fresh_name("ml_guarded_record_key");
        let position_name = self.fresh_name("ml_guarded_record_position");
        let owned = Expr::var(&capture_name, object.ty.clone());
        let text = Expr::var(&text_name, STRING);
        let position = Expr::var(&position_name, FLOAT);
        let names = crate::translation::ordered_record_values::property_names(
            fields.iter().map(|field| field.name.as_str()),
        );
        let value = Expr::new(
            Node::Math {
                op: "readJavaScriptRecordPropertyIndex".to_owned(),
                args: vec![names, text],
            },
            FLOAT,
        );
        let mut body = Expr::new(
            Node::None {
                absence: "undefined".to_owned(),
            },
            result.clone(),
        );
        let count = serde_json::Number::from(fields.len())
            .as_f64()
            .expect("actual field count is a Number");
        for index in rules::read_java_script_guarded_record_projection_positions(count) {
            let index = crate::translation::frontend_rules::ml_array::number_index(index)
                .expect("actual canonical field position");
            let field = &fields[index];
            let read = Expr::new(
                Node::RecordField {
                    object: Box::new(owned.clone()),
                    field: field.name.clone(),
                    index,
                },
                field.ty.clone(),
            );
            let condition = Expr::new(
                super::binary_node(
                    BinaryOp::Eq,
                    Box::new(position.clone()),
                    Box::new(Expr::lit(FLOAT, LitValue::Text(index.to_string()))),
                    Some(FLOAT),
                    None,
                    None,
                    None,
                ),
                BOOL,
            );
            body = Expr::new(
                Node::If {
                    cond: Box::new(condition),
                    then: Box::new(self.coerce(read, &result, node.span)?),
                    otherwise: Box::new(body),
                },
                result.clone(),
            );
        }
        body = Expr::new(
            Node::Let {
                name: position_name,
                value: Box::new(value),
                body: Box::new(body),
            },
            result.clone(),
        );
        body = Expr::new(
            Node::Let {
                name: text_name,
                value: Box::new(key),
                body: Box::new(body),
            },
            result.clone(),
        );
        Ok(Some(Expr::new(
            Node::Let {
                name: capture_name,
                value: Box::new(object),
                body: Box::new(body),
            },
            result,
        )))
    }
}
