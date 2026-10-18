use super::attribute_integer_key;
use crate::translation::grammar_attribute_order_rules::read_grammar_attribute_integer_key;

#[test]
fn actual_attribute_names_keep_shared_decimal_and_unicode_boundaries() {
    let fixture = crate::read_structured_data_lino(include_str!(
        "../../../data/grammar-attribute-order-fixtures.lino"
    ))
    .expect("shared attribute order fixture");
    for case in fixture["integerNames"]
        .as_array()
        .expect("integer name cases")
    {
        let name = case["name"].as_str().expect("attribute name");
        let expected = case["index"].as_f64().expect("expected index");
        assert_eq!(
            attribute_integer_key(name).to_bits(),
            expected.to_bits(),
            "{name}"
        );
    }
    for units in [
        vec![f64::NAN],
        vec![f64::INFINITY],
        vec![f64::NEG_INFINITY],
        vec![48.5],
        vec![-1.0],
        vec![65536.0],
    ] {
        assert_eq!(
            read_grammar_attribute_integer_key(&units).to_bits(),
            (-1.0_f64).to_bits()
        );
    }
}
