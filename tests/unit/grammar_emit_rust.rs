use meta_language::{
    emit_rust_parser, CharClassItem, Grammar, RuleKind, RustFieldShape, RustTypeShape,
};

#[test]
fn emits_rust_parser_bundle_for_sum_grammar() {
    let grammar = sum_grammar();

    let (artifacts, report) = emit_rust_parser(&grammar).expect("Rust parser codegen emits");

    assert!(report.lossy.is_empty());
    assert_eq!(
        artifacts.pest_grammar,
        "sum = { num ~ (\"+\" ~ num)* }\nnum = @{ '0'..'9'+ }\n"
    );
    pest_meta::parse_and_optimize(&artifacts.pest_grammar).expect("pest grammar is valid");
    assert_eq!(
        artifacts.parser_struct,
        concat!(
            "#[derive(pest_derive::Parser)]\n",
            "#[grammar_inline = \"sum = { num ~ (\\\"+\\\" ~ num)* }\\nnum = @{ '0'..'9'+ }\\n\"]\n",
            "pub struct SumParser;\n",
        )
    );
    assert_eq!(
        artifacts.ast_shapes,
        vec![
            RustTypeShape::structure("Sum", [RustFieldShape::new("num", "Vec<Num>")]),
            RustTypeShape::structure("Num", [RustFieldShape::new("0", "String")]),
        ]
    );
    assert_eq!(
        artifacts.ast_types,
        concat!(
            "#[derive(Debug, Clone)]\n",
            "pub struct Sum {\n",
            "    pub num: Vec<Num>,\n",
            "}\n",
            "\n",
            "#[derive(Debug, Clone)]\n",
            "pub struct Num(pub String);\n",
        )
    );
}

#[test]
fn emits_enum_shapes_for_top_level_choices_and_records_unordered_loss() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("value")
        .rule(
            "value",
            expr.choice_unordered([
                expr.nt("number"),
                expr.nt("string"),
                expr.term("true"),
                expr.term("false"),
            ]),
        )
        .rule("number", expr.rep1(expr.char_range('0', '9')))
        .rule(
            "string",
            expr.seq([
                expr.term("\""),
                expr.rep0(expr.char_class(true, [CharClassItem::char('"')])),
                expr.term("\""),
            ]),
        )
        .build();

    let (artifacts, report) = emit_rust_parser(&grammar).expect("Rust parser codegen emits");

    assert!(report
        .lossy
        .iter()
        .any(|note| note.contains("unordered choice")));
    assert!(artifacts
        .pest_grammar
        .contains("// NOTE: unordered choice in source is emitted as ordered pest choice."));
    assert_eq!(
        artifacts.ast_shapes[0],
        RustTypeShape::enumeration(
            "Value",
            [
                RustFieldShape::new("Number", "Number"),
                RustFieldShape::new("String", "String"),
                RustFieldShape::new("True", "String"),
                RustFieldShape::new("False", "String"),
            ],
        )
    );
    assert!(artifacts.ast_types.contains("pub enum Value"));
    assert!(artifacts.ast_types.contains("    Number(Number),"));
    assert!(artifacts.ast_types.contains("    True(String),"));
}

fn sum_grammar() -> Grammar {
    let expr = Grammar::expr();
    Grammar::builder()
        .start("sum")
        .rule(
            "sum",
            expr.seq([
                expr.nt("num"),
                expr.rep0(expr.seq([expr.term("+"), expr.nt("num")])),
            ]),
        )
        .rule_with_kind(
            "num",
            expr.rep1(expr.char_range('0', '9')),
            RuleKind::Atomic,
        )
        .build()
}
