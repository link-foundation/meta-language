use pest::Parser as _;

#[derive(pest_derive::Parser)]
#[grammar_inline = "sum = { num ~ (\"+\" ~ num)* }\nnum = @{ '0'..'9'+ }\n"]
pub struct SumParser;

#[derive(Debug, Clone)]
pub struct Sum {
    pub num: Vec<Num>,
}

#[derive(Debug, Clone)]
pub struct Num(pub String);

#[test]
fn committed_pest_derive_fixture_parses_sum_samples() {
    let pairs = SumParser::parse(Rule::sum, "1+2+3").expect("sum parses");
    assert_eq!(pairs.as_str(), "1+2+3");
    assert!(SumParser::parse(Rule::sum, "123").is_ok());
    assert!(SumParser::parse(Rule::sum, "+1").is_err());

    let ast = Sum {
        num: vec![Num("1".to_string()), Num("2".to_string())],
    };
    assert_eq!(ast.num.len(), 2);
    assert_eq!(ast.num[0].0, "1");
}
