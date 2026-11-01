//! Deferred controls for actual native generic source aliases and shared rules.
//! Not executed during source authoring. Generated module/asset remains pending.
#[path = "../../src/translation/rust_generic_type_alias_rules.rs"]
mod generic_alias_rules;

#[test]
fn explicit_generic_aliases_keep_original_products_and_the_complete_rule_program() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/type-aliases.mjs",
        "explicit Rust generic identity tuple and composed aliases",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-generic-type-alias-program",
            )
            .map_err(|message|meta_language::TypedProgramLinksError {message})
        },
    );
}

#[test]
fn native_generic_alias_bindings_preserve_actual_owners_and_argument_order() {
    let parameters = vec!["alias/parameter/0".to_owned(),"alias/parameter/1".to_owned()];
    let occurrences = vec!["alias/parameter/1".to_owned(),"module/type/T".to_owned(),"alias/parameter/0".to_owned()];
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_substitution_plan(&parameters,&[true,true],&occurrences),vec!["substitution","1","-1","0"]);
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_substitution_plan(&parameters,&[true],&occurrences),vec!["error","generic alias argument arity differs from its declaration"]);
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_substitution_plan(&parameters,&[true,false],&occurrences),vec!["error","generic alias arguments need complete checked source origins"]);
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_expansion_error("crate.Alias",&["crate.Alias".to_owned()],1.0),"recursive source type alias");
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_expansion_error("crate.Alias",&[],4096.0),"source type alias expansion limit exceeded");
}

#[test]
fn generic_template_scalar_leaves_need_the_actual_complete_primitive_owner() {
    assert!(generic_alias_rules::accept_rust_generic_type_alias_concrete_template("fixed",0.0,"primitive","i64","64",true,0.0,true));
    assert!(!generic_alias_rules::accept_rust_generic_type_alias_concrete_template("fixed",0.0,"declaration","i64","64",true,0.0,true));
    assert!(!generic_alias_rules::accept_rust_generic_type_alias_concrete_template("fixed",0.0,"primitive","u64","64",true,0.0,true));
    assert!(!generic_alias_rules::accept_rust_generic_type_alias_concrete_template("fixed",1.0,"primitive","i64","64",true,0.0,true));
    assert!(!generic_alias_rules::accept_rust_generic_type_alias_concrete_template("fixed",0.0,"primitive","i64","64",true,0.0,false));
}

#[test]
fn symbolic_sum_templates_keep_actual_canonical_owners_arity_and_named_leaves() {
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_standard_sum_owner("std::option::Option","",1.0),"std::option::Option");
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_standard_sum_owner("std::result::Result","",2.0),"std::result::Result");
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_standard_sum_owner("std::option::Option","Some",1.0),"");
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_standard_sum_owner("std::option::Option","",2.0),"");
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_standard_sum_owner("local::Option","",1.0),"");
    assert!(generic_alias_rules::accept_rust_generic_type_alias_named_template("named",0.0,"declaration","errors::Error",0.0,true,"errors.Error"));
    assert!(!generic_alias_rules::accept_rust_generic_type_alias_named_template("named",0.0,"declaration","caller::Error",0.0,true,"errors.Error"));
}

#[test]
fn generic_vector_templates_keep_actual_scalar_and_owned_string_element_origins() {
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_vector_owner("std::vec::Vec",false),"std::vec::Vec");
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_vector_owner("std::vec::Vec",true),"");
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_vector_owner("alloc::vec::Vec",false),"");
    assert!(generic_alias_rules::accept_rust_generic_type_alias_vector_arguments("std::vec::Vec",1.0));
    assert!(!generic_alias_rules::accept_rust_generic_type_alias_vector_arguments("std::vec::Vec",2.0));
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_vector_element_error("fixed",0.0,"primitive","i64","64",true,0.0,true,""),"");
    assert_ne!(generic_alias_rules::read_rust_generic_type_alias_vector_element_error("fixed",0.0,"declaration","i64","64",true,0.0,true,""),"");
    assert_ne!(generic_alias_rules::read_rust_generic_type_alias_vector_element_error("fixed",0.0,"primitive","u64","64",true,0.0,true,""),"");
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_vector_element_error("string",0.0,"library","std::string::String","",false,0.0,true,"owned string"),"");
    assert_ne!(generic_alias_rules::read_rust_generic_type_alias_vector_element_error("string",1.0,"primitive","str","",false,0.0,true,"string slice"),"");
}

#[test]
fn the_original_symbolic_vector_alias_is_checked_without_a_runtime_export() {
    // Exact preceding refused source: Rust array element: only direct immutable
    // scalar and owned string elements are admitted. The input stays unchanged.
    let source="type Vector<T>=Vec<T>;fn main(){}";
    let options=meta_language::SelfTranslationOptions {semantic_translation:true,..meta_language::SelfTranslationOptions::default()};
    assert!(meta_language::self_translation_signatures(source,"Rust",&options).unwrap().is_empty());
}

#[test]
fn generic_type_defaults_select_only_earlier_complete_binding_identities(){
    let parameters=vec!["owner/T".to_owned(),"owner/U".to_owned()];
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_default_substitution_plan(&parameters,1.0,1.0,&["owner/T".to_owned()]),vec!["substitution".to_owned(),"0".to_owned()]);
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_default_substitution_plan(&parameters,0.0,0.0,&[]),vec!["substitution".to_owned()]);
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_default_substitution_plan(&parameters,0.0,0.0,&["owner/U".to_owned()])[0],"error");
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_default_argument_plan(&[false,true],1.0,true),vec!["arguments".to_owned(),"1".to_owned()]);
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_default_argument_plan(&[false,true],0.0,false)[0],"error");
    assert!(generic_alias_rules::accept_rust_generic_type_alias_concrete_identifier(&["T".into()],"T",true));
    assert!(!generic_alias_rules::accept_rust_generic_type_alias_concrete_identifier(&["T".into()],"T",false));
}

#[test]
fn defaulted_declarations_and_actual_partial_applications_reach_native_frontend(){
    let options=meta_language::SelfTranslationOptions {semantic_translation:true,..meta_language::SelfTranslationOptions::default()};
    for source in [
        "type Identity<T=u8>=T;fn main(){}",
        "type Pair<T=u8,U=T>=(T,U);pub fn result()->u16{let value:Pair<u16>=(7u16,9u16);value.1}",
        "type Values<T=u8>=Vec<T>;fn make()->Values{vec![7u8]}pub fn result()->usize{make().len()}",
        "type Values<T=u8>=Vec<T>;pub fn result()->usize{Values::new().len()}",
        "type Maybe<T=u8>=Option<T>;pub fn result()->u8{let value=Maybe::Some(7u8);value.unwrap_or(0u8)}",
        "type Outcome<T=u8,E=i64>=Result<T,E>;pub fn result()->u8{let value=Outcome::Err(9i64);value.unwrap_or(7u8)}",
        "type Scalar<T=u8>=T;pub fn result()->u8{Scalar::MAX}",
        "type Scalar<T=u8>=T;pub fn result()->bool{Scalar::MAX<255u8}",
        "type Scalar<T=u8>=T;pub fn result()->u8{let make:fn()->u8=Scalar::default;make()}",
        "type Text<T=String>=T;pub fn result()->usize{Text::from(\"abc\").len()}",
        "#[derive(Copy,Clone)]enum State{Ready}type Value<T=State>=T;pub fn result()->u8{let value=Value::Ready;match value{State::Ready=>7u8}}",
        "type Identity<T=u8>=T;type Pair<T=i64,U=Identity<T>>=(T,U);type Forward<T=u16>=Pair<T>;pub fn result()->u16{let value:Forward=(7u16,9u16);value.1}",
    ]{meta_language::self_translation_signatures(source,"Rust",&options).unwrap();}
    for source in [
        "type Bad<T=T>=T;fn main(){}",
        "type Bad<T=U,U=u8>=(T,U);fn main(){}",
        "type Bad<T=u8,U>=(T,U);fn main(){}",
        "type Bad<T=Missing>=T;pub fn result()->u8{let value:Bad<u8>=7u8;value}",
        "type First<T=Second>=T;type Second<U=First>=U;fn main(){}",
        "type Values<T>=Vec<T>;pub fn result()->usize{Values::new().len()}",
        "type Maybe<T=u8>=Option<T>;fn main(){let value=Maybe::Err(7u8);}",
    ]{assert!(meta_language::self_translation_signatures(source,"Rust",&options).is_err());}
}

#[test]
fn standard_sum_signatures_require_actual_layouts_and_retained_payload_owners() {
    use meta_language::translation::ir::DataDecl;
    use meta_language::translation::source_signatures::{SourceSignature,validate_source_signature,validate_source_signature_with_data};
    use meta_language::translation::types::Type;
    use serde_json::json;
    let scalar=json!({"kind":"fixed","bits":64,"signed":true});
    let scalar_owner=json!({"kind":"primitive","path":["i64"],"arguments":[]});
    let mut declaration:DataDecl=serde_json::from_value(json!({
        "name":"Sum","fullName":"actual.Sum","modulePath":["actual"],
        "ctors":[{"name":"None","fields":[]},{"name":"Some","fields":[{"name":"field0","type":scalar.clone()}]}],
        "sourceFieldSignatures":{"model":"Rust data type field signatures 1","language":"Rust","dataTypeName":"actual.Sum","ctors":[{"name":"None","fields":[]},{"name":"Some","fields":[{"type":scalar,"wrappers":[],"lifetimes":[],"owner":scalar_owner.clone()}]}]}
    })).unwrap();
    let result=Type::Data {name:"actual.Sum".to_owned()};
    let signature:SourceSignature=serde_json::from_value(json!({
        "model":"Rust source function signature 2","language":"Rust","parameters":[],"lifetimeBindings":[],
        "result":{"type":{"kind":"data","name":"actual.Sum"},"wrappers":[],"lifetimes":[],"owner":{"kind":"library","path":["std","option","Option"],"arguments":[scalar_owner]}}
    })).unwrap();
    assert!(validate_source_signature(&signature,&[],&result,"Rust").is_err());
    let validate=|signature:&SourceSignature,declaration:&DataDecl|validate_source_signature_with_data(signature,&[],&result,"Rust",|ty|Ok(ty.clone()),|ty|match ty {Type::Data {name} if name==&declaration.full_name=>Some(declaration),_=>None});
    validate(&signature,&declaration).unwrap();
    let mut forged=signature.clone();
    forged.result.owner.arguments[0].path=vec!["isize".to_owned()];
    assert!(validate(&forged,&declaration).is_err(),"matching numeric width cannot replace a retained i64 owner");
    declaration.ctors.reverse();
    declaration.source_field_signatures.as_mut().unwrap().ctors.reverse();
    assert!(validate(&signature,&declaration).is_err(),"actual constructor order remains part of the shared shape contract");
    declaration.ctors.reverse();
    declaration.source_field_signatures.as_mut().unwrap().ctors.reverse();
    declaration.source_field_signatures=None;
    assert!(validate(&signature,&declaration).is_err(),"actual field-signature ownership cannot be omitted");
}
