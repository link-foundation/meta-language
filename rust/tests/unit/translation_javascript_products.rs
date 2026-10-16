//! Relocated frontend regressions; all original test names and assertions are retained.
use meta_language::translation::{
    javascript::{ModuleContext, parse_javascript, parse_javascript_in},
    surface::{External, ExternalParam, SItem, SNode},
    types::Type,
};
#[test]
fn translation_javascript_products_use_closed_fresh_values_and_static_positions() {
    let source = "/** @returns {[number,boolean]} */ function make(){return Object.freeze([5,true]);}\n\
        /** @param {[number,boolean]} value @returns {number} */ function choose(value){const [number,flag]=value;return flag?number+2:number;}\n\
        const value=make();console.log(choose(value)===7);console.log(value[-0]===value[0]);";
    let surface = parse_javascript(source).expect("closed fresh tuple products parse");
    let SItem::Fn(make) = &surface.items[0] else {
        panic!("the factory is a function");
    };
    let SNode::Tuple {
        product_model,
        frozen,
        ..
    } = &make.body.node
    else {
        panic!("the factory retains its source product model");
    };
    assert!(*frozen);
    assert_eq!(product_model.as_deref(), Some("immutable value product"));
    meta_language::translation::check::check_program(&surface)
        .expect("the closed product program checks");
    let value = serde_json::to_value(&surface).expect("source products serialize");
    assert_eq!(
        value["main"]["effects"][2]["expr"]["left"]["k"],
        "tupleIndex"
    );
    assert_eq!(value["main"]["effects"][2]["expr"]["left"]["index"], 0);
}

#[test]
fn translation_javascript_product_destructuring_keeps_one_initializer_evaluation() {
    let source = "function make(){console.log('read');return Object.freeze([Object.freeze([5,true]),2]);}\n\
        const [[value,flag],increment]=make();console.log(flag?value+increment:value);";
    let surface = parse_javascript(source).expect("nested fresh products parse");
    meta_language::translation::check::check_program(&surface)
        .expect("nested product positions check");
    let value = serde_json::to_value(&surface).expect("the source program serializes");
    let effects = &value["main"]["effects"];
    assert_eq!(effects[0]["value"]["path"][1], "make");
    assert_eq!(effects[1]["name"], "value");
    assert_eq!(effects[2]["name"], "flag");
    assert_eq!(effects[3]["name"], "increment");
    assert_eq!(effects[1]["value"]["k"], "tupleIndex");
    assert_eq!(effects[1]["value"]["object"]["k"], "tupleIndex");
}

#[test]
fn translation_javascript_product_sources_refuse_unproved_ownership_and_uses() {
    for source in [
        "function make(){return Object.freeze([[5,true],2]);}",
        "function make(){return Object.freeze([]);}",
        "function Object(value){return value;}function make(){return Object.freeze([5,true]);}",
        "export function make(){return Object.freeze([5,true]);}",
        "const value=Object.freeze([5,true]);console.log(value===value);",
        "/** @returns {[number,boolean]} */ function make(){return [5,true];}console.log(make()[0]);",
        "/** @param {[number,boolean]} value */ function read(value){return value[0];}console.log(read([5,true]));",
        "function read(){const [[value]]=[[5]];return value;}console.log(read());",
    ] {
        assert!(
            parse_javascript(source)
                .and_then(|surface| meta_language::translation::check::check_program(&surface))
                .is_err(),
            "unproved products must refuse: {source}"
        );
    }
}

#[test]
fn translation_javascript_products_in_record_entries_keep_the_existing_use_refusal() {
    let source = "const record={value:Object.freeze([5,true])};console.log(record.value[0]);";
    let error = parse_javascript(source)
        .expect_err("an ordinary record does not establish an immutable product use proof");
    assert_eq!(
        error.construct.as_deref(),
        Some("immutable tuple product use")
    );
    assert_eq!(
        error.reason,
        "a product only supports owned bindings, direct calls, and fixed positional reads"
    );
}

#[test]
fn translation_javascript_products_preserve_legacy_arrays_and_require_actual_callers() {
    let source = "/** @param {[number,number]} pair */function first(pair){return pair[0];}console.log(first([2,3]));";
    let surface = parse_javascript(source).expect("legacy homogeneous positional arrays parse");
    meta_language::translation::check::check_program(&surface)
        .expect("legacy positional reads check");
    for source in [
        "/** @returns {[string,number]} */function pair(){return ['name',5];}",
        "/** @param {string[]} values @returns {[string,number]} */function pair(values){return [values[0],1];}",
    ] {
        let surface = parse_javascript(source).expect("legacy reference-bearing tuples parse");
        meta_language::translation::check::check_program(&surface)
            .expect("legacy reference-bearing tuples check");
    }
    for source in [
        "function make(){return Object.freeze([1,2]);}",
        "function make(){return Object.freeze([5,true]);}",
    ] {
        let error = parse_javascript(source).expect_err("a product return needs actual callers");
        assert!(error.to_string().contains("Object.freeze operand"));
    }
}

#[test]
fn translation_javascript_recursive_product_inference_has_a_finite_refusal() {
    let error = parse_javascript("function make(){return Object.freeze([make(),true]);}")
        .expect_err("a recursive inferred product needs a declared data type");
    assert_eq!(
        error.reason,
        "a recursive inferred type needs an explicit data type"
    );
}

#[test]
fn translation_javascript_product_fields_resolve_shadowed_global_bindings_and_tdz() {
    for name in ["NaN", "Infinity", "undefined"] {
        let source = format!(
            "function make({name}){{return Object.freeze([{name},true]);}}const value=make(5);console.log(value[0]===5);"
        );
        let surface =
            parse_javascript(&source).expect("global binding names can be lexical parameters");
        meta_language::translation::check::check_program(&surface)
            .expect("the source product retains its actual parameter");
        let SItem::Fn(function) = &surface.items[0] else {
            panic!("the factory is a function");
        };
        let SNode::Tuple { items, .. } = &function.body.node else {
            panic!("the source is a frozen tuple");
        };
        assert!(matches!(&items[0].node, SNode::Name { path } if path == &[name.to_owned()]));
        let source = format!(
            "function make(){{const value=Object.freeze([{name},true]);const {name}=5;return value;}}console.log(make()[0]);"
        );
        assert!(
            parse_javascript(&source)
                .expect_err("the later lexical binding remains in its temporal dead zone")
                .reason
                .contains("temporal dead zone")
        );
        let source = format!(
            "function make(){{return Object.freeze([{name},true]);}}const {name}=5;console.log(make()[0]);"
        );
        assert!(
            parse_javascript(&source)
                .expect_err("an uncaptured module binding cannot become a built-in constant")
                .construct
                .as_deref()
                .is_some_and(|construct| construct.contains("global binding"))
        );
        let source = format!(
            "function make(){{return Object.freeze([{name},true]);}}import {{value as {name}}} from './values.js';"
        );
        assert!(
            parse_javascript_in(
                &source,
                &ModuleContext {
                    module_directory: Some(vec!["example".to_owned()]),
                    ..ModuleContext::default()
                }
            )
            .expect_err("an imported alias cannot become a built-in constant")
            .construct
            .as_deref()
            .is_some_and(|construct| construct.contains("global binding"))
        );
    }
}

#[test]
fn translation_javascript_whole_product_conditions_keep_an_explicit_obligation() {
    for condition in [
        "value",
        "value?true:false",
        "!value",
        "value&&true",
        "value||false",
    ] {
        let source = format!(
            "function check(){{const value=Object.freeze([5,true]);if({condition})return true;return false;}}console.log(check());"
        );
        let error = parse_javascript(&source).expect_err("a whole product condition refuses");
        if !condition.contains("&&") && !condition.contains("||") {
            assert_eq!(
                error.construct.as_deref(),
                Some("immutable tuple product use")
            );
        }
    }
}

#[test]
fn translation_javascript_parameter_defaults_share_product_caller_and_escape_proofs() {
    let source = "function read(value=Object.freeze([5,true])){return value[0];}console.log(read()===5);console.log(read(Object.freeze([7,true]))===7);";
    let surface = parse_javascript(source).expect("fresh default products stay in closed calls");
    meta_language::translation::check::check_program(&surface)
        .expect("the default product positions check");
    let context = ModuleContext {
        externals: vec![External::Function {
            name: "escape".to_owned(),
            params: vec![ExternalParam {
                name: "value".to_owned(),
                ty: Type::Tuple {
                    elements: vec![Type::Float, Type::Bool],
                },
            }],
            ret: Type::Bool,
        }],
        ..ModuleContext::default()
    };
    let error = parse_javascript_in(
        "function run(flag=escape(Object.freeze([5,true]))){return flag;}console.log(run());",
        &context,
    )
    .expect_err("a product cannot escape through a parameter default");
    assert!(error.reason.contains("unknown or external caller"));
}

#[test]
fn translation_javascript_default_parameter_bindings_keep_temporal_dead_zones() {
    for name in ["NaN", "Infinity", "undefined"] {
        for parameters in [
            format!("{name}={name}"),
            format!("value={name},{name}=5"),
            format!("{name}=`${{{name}}}`"),
        ] {
            let source =
                format!("function make({parameters}){{return Object.freeze([5,true]);}}make();");
            let error = parse_javascript(&source)
                .expect_err("defaults cannot substitute constants for parameter bindings");
            assert!(
                error.reason.contains("temporal dead zone")
                    || error.construct.as_deref().is_some_and(|construct| {
                        construct.contains("default reading parameter")
                    })
            );
        }
    }
}
