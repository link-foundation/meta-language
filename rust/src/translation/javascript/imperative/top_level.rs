//! Top-level statements that declare or assign variables, lowered to main's bindings.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use super::{
    Ctx, Fall, JavaScriptParser, Lowering, Result, SCtor, SData, SExpr, SItem, SNode, Span, Stmt,
    Var, ctor_row, flat, internal, named, node, statement_uses, union, untyped, visible,
};
use crate::translation::surface::SEffect;

/// Top-level statements are one expression, whose value carries the
/// top-level variables they declare or assign; main binds each of them for
/// the statements after it. `variables` are main's bindings before them.
pub(in crate::translation::javascript) fn lower_top_level(
    parser: &mut JavaScriptParser,
    statements: &[Stmt],
    variables: &[String],
    place: Span,
) -> Result<Vec<SEffect>> {
    let mut lowering = Lowering {
        function: "main".to_owned(),
        renames: 0,
        generated: std::mem::take(&mut parser.generated),
        count: parser.generated_count,
    };
    let list = flat(statements);
    let declared: HashSet<String> = list
        .iter()
        .filter_map(|statement| match statement {
            Stmt::Let { name, .. } => Some(name.clone()),
            _ => None,
        })
        .collect();
    let assigned = statement_uses(&list).assigned;
    let kept = Rc::new(union([&declared, &assigned]));
    // What falling off the end carried, and the data type that carries several variables.
    let carried: Rc<RefCell<(Vec<Var>, Option<String>)>> = Rc::default();
    let fall: Fall = {
        let carried = Rc::clone(&carried);
        let kept = Rc::clone(&kept);
        Rc::new(move |lowering, scope| {
            let entries: Vec<Var> = visible(scope)
                .into_iter()
                .filter(|entry| kept.contains(&entry.source))
                .collect();
            let value = match entries.as_slice() {
                [only] => named(&only.ir, place),
                [] => node(SNode::Bool { value: true }, place),
                _ => {
                    // An if or switch at the end falls off it in each branch, each with the same variables.
                    let known = carried.borrow().1.clone();
                    let data = known.unwrap_or_else(|| lowering.generated_name("top"));
                    let value = node(
                        SNode::CtorObject {
                            tag: format!("{data}_done"),
                            fields: entries
                                .iter()
                                .map(|entry| (entry.ir.clone(), named(&entry.ir, place)))
                                .collect(),
                        },
                        place,
                    );
                    carried.borrow_mut().1 = Some(data);
                    value
                }
            };
            carried.borrow_mut().0 = entries;
            Ok(value)
        })
    };
    let before: HashSet<String> = variables.iter().cloned().collect();
    let ctx = Ctx {
        fall,
        ret: Rc::new(move |_, _| Err(internal("return at the top level", place))),
        brk: Rc::new(move |_| Err(internal("break outside a loop", place))),
        cont: Rc::new(move |_| Err(internal("continue outside a loop", place))),
        live: Rc::new(union([&before, kept.as_ref()])),
        brk_live: Rc::default(),
        cont_live: Rc::default(),
    };
    let scope: Vec<Var> = variables
        .iter()
        .map(|name| Var {
            source: name.clone(),
            ir: name.clone(),
        })
        .collect();
    let value = lowering.seq(&list, 0, &ctx, &scope);
    let (carried, data) = carried.take();
    let effects = value.map(|value| {
        let bind = |name: String, value: SExpr| SEffect::Let {
            name,
            ty: None,
            value,
            constant: false,
            span: Some(place),
        };
        match (carried.as_slice(), data) {
            ([only], _) => vec![bind(only.source.clone(), value)],
            ([], _) | (_, None) => vec![bind(lowering.generated_name("top"), value)],
            (_, Some(data)) => {
                let ctor = format!("{data}_done");
                lowering.generated.push(SItem::Data(SData {
                    name: data.clone(),
                    ctors: vec![SCtor {
                        name: ctor.clone(),
                        fields: carried.iter().map(|entry| untyped(&entry.ir)).collect(),
                    }],
                    span: Some(place),
                    generated: true,
                }));
                let whole = format!("{data}_value");
                let names: Vec<String> = carried.iter().map(|entry| entry.ir.clone()).collect();
                let project = |entry: &Var| {
                    node(
                        SNode::Match {
                            scrutinees: vec![named(&whole, place)],
                            rows: vec![ctor_row(
                                &data,
                                &ctor,
                                &names,
                                named(&entry.ir, place),
                                place,
                            )],
                        },
                        place,
                    )
                };
                let mut effects = vec![bind(whole.clone(), value)];
                effects.extend(
                    carried
                        .iter()
                        .map(|entry| bind(entry.source.clone(), project(entry))),
                );
                effects
            }
        }
    });
    parser.generated = lowering.generated;
    parser.generated_count = lowering.count;
    effects
}
