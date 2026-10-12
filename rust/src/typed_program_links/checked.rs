//! Adapt the supported typed IR to the existing checker, preserving policies.
use super::{Result, error};
use crate::translation::check::check_program;
use crate::translation::ir::{Decl, Effect, Expr, Item, LitValue, Node, Program};
use crate::translation::surface::{
    SEffect, SExpr, SFn, SItem, SMain, SNode, SParam, SProgram, ShowStyle,
};
use crate::translation::types::Type;
use regex::Regex;
use std::sync::OnceLock;

fn valid_float(value: &str) -> bool {
    static SPELLING: OnceLock<Regex> = OnceLock::new();
    SPELLING
        .get_or_init(|| {
            Regex::new(
                r"(?i)^(?:-?(?:[0-9]+(?:\.[0-9]+)?)(?:e[+-]?[0-9]+)?|NaN|Infinity|-Infinity)$",
            )
            .unwrap()
        })
        .is_match(value)
}

fn expression(value: &Expr) -> Result<SExpr> {
    if let Node::Lit {
        value: LitValue::Text(text),
    } = &value.node
    {
        if value.ty == Type::Float && !valid_float(text) {
            return Err(error("a linked floating-point literal is invalid"));
        }
    }
    let node = match &value.node {
        Node::Lit {
            value: LitValue::Bool(boolean),
        } if value.ty == Type::Bool => SNode::Bool { value: *boolean },
        Node::Lit {
            value: LitValue::Text(text),
        } if value.ty == Type::String => SNode::Str {
            value: text.clone(),
        },
        Node::Lit {
            value: LitValue::Text(text),
        } if value.ty.is_ordered() => SNode::Num {
            value: text.clone(),
            ty: Some(value.ty.clone()),
            negative: false,
            unit: false,
        },
        Node::Unit => SNode::Unit,
        Node::Var { name } => SNode::Name {
            path: vec![name.clone()],
        },
        Node::Call { func, args } => SNode::App {
            func: Box::new(SExpr::name(func)),
            args: args.iter().map(expression).collect::<Result<_>>()?,
        },
        Node::Unary { op, arg, .. } => SNode::Unary {
            op: *op,
            arg: Box::new(expression(arg)?),
        },
        Node::Binary {
            op,
            left,
            right,
            rounding,
            ..
        } => SNode::Binary {
            op: *op,
            left: Box::new(expression(left)?),
            right: Box::new(expression(right)?),
            rounding: *rounding,
        },
        Node::If {
            cond,
            then,
            otherwise,
        } => SNode::If {
            cond: Box::new(expression(cond)?),
            then: Box::new(expression(then)?),
            otherwise: Box::new(expression(otherwise)?),
        },
        Node::Let { name, value, body } => SNode::Let {
            name: name.clone(),
            ty: Some(value.ty.clone()),
            value: Box::new(expression(value)?),
            body: Box::new(expression(body)?),
        },
        Node::ToString { arg, console } => {
            if *console {
                SNode::Show {
                    arg: Box::new(expression(arg)?),
                    style: ShowStyle::JsConsole,
                }
            } else {
                SNode::ToString {
                    arg: Box::new(expression(arg)?),
                }
            }
        }
        _ => {
            return Err(error(format!(
                "the Rust typed links adapter does not yet recheck {} expressions",
                value.kind()
            )));
        }
    };
    Ok(SExpr::new(node, value.span))
}
pub(super) fn validate(program: &Program) -> Result<()> {
    if program.schema_version != 1 {
        return Err(error("the linked value is not a checked typed program"));
    }
    if !program.imports.is_empty()
        || !program.externals.is_empty()
        || program.output_threaded
        || program.aborts_threaded
    {
        return Err(error(
            "the Rust typed links adapter does not yet recheck imported or threaded programs",
        ));
    }
    let mut items = Vec::new();
    for item in &program.items {
        let Item::Decl { full_name } = item else {
            return Err(error(
                "the Rust typed links adapter does not yet recheck module items",
            ));
        };
        let Some(Decl::Fn(function)) = program.declaration(full_name) else {
            return Err(error(
                "the Rust typed links adapter currently rechecks function declarations",
            ));
        };
        if !function.module_path.is_empty() {
            return Err(error(
                "the Rust typed links adapter does not yet recheck qualified declarations",
            ));
        }
        items.push(SItem::Fn(SFn {
            name: function.name.clone(),
            params: function
                .params
                .iter()
                .map(|parameter| SParam {
                    name: parameter.name.clone(),
                    ty: Some(parameter.ty.clone()),
                    default_value: parameter.default_value.clone(),
                    guard: parameter.guard.clone(),
                    span: None,
                    rocq_type: None,
                })
                .collect(),
            ret: Some(function.ret.clone()),
            body: expression(&function.body)?,
            span: function.span,
            generated: function.generated,
        }));
    }
    if items.len() != program.declarations.len() {
        return Err(error(
            "the linked declaration mapping and ordered items disagree",
        ));
    }
    let main = program
        .main
        .as_ref()
        .map(|main| -> Result<SMain> {
            let effects = main
                .effects
                .iter()
                .map(|effect| match effect {
                    Effect::Print { expr, span } => Ok(SEffect::Print {
                        expr: expression(expr)?,
                        style: ShowStyle::JsConsole,
                        span: *span,
                    }),
                    Effect::Let {
                        name,
                        value,
                        constant,
                        span,
                    } => Ok(SEffect::Let {
                        name: name.clone(),
                        ty: Some(value.ty.clone()),
                        value: expression(value)?,
                        constant: *constant,
                        span: *span,
                    }),
                    _ => Err(error(
                        "the Rust typed links adapter does not yet recheck this main effect",
                    )),
                })
                .collect::<Result<_>>()?;
            Ok(SMain {
                effects,
                span: main.span,
                sequential_async: main.sequential_async,
            })
        })
        .transpose()?;
    let surface = SProgram {
        language: program.source_language,
        items,
        main,
        imports: Vec::new(),
        externals: Vec::new(),
    };
    let checked = check_program(&surface).map_err(|failure| error(failure.to_string()))?;
    if checked != *program {
        return Err(error(
            "the linked program has inconsistent types, bindings or operation semantics",
        ));
    }
    Ok(())
}
