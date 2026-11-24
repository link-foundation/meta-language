//! Deferred controls for actual native generic source aliases and shared rules.
//! Not executed during source authoring. Generated module/asset remains pending.
#[path = "../../src/translation/rust_generic_type_alias_rules.rs"]
mod generic_alias_rules;
#[path = "../../src/translation/rust_function_pointer_rules.rs"]
mod pointer_rules;

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
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_default_owner_phase(true,true),"defer");
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_default_owner_phase(true,false),"prepare");
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_default_owner_phase(false,true),"prepare");
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_default_owner_phase(false,false),"prepare");
}

#[test]
fn owned_sum_alias_selection_never_fabricates_copy_evidence(){
    assert!(!generic_alias_rules::accept_rust_generic_type_alias_standard_sum_leaf("named",0.0,"std::option::Option",1.0,true,true,false,0.0));
    assert!(generic_alias_rules::accept_rust_generic_type_alias_owned_standard_sum_leaf("named",0.0,"std::option::Option",1.0,true,true,0.0,"owned"));
    for profile in ["","copy","borrowed","custom Drop"]{assert!(!generic_alias_rules::accept_rust_generic_type_alias_owned_standard_sum_leaf("named",0.0,"std::option::Option",1.0,true,true,0.0,profile));}
    assert!(generic_alias_rules::read_rust_generic_type_alias_constructor_plan("named",0.0,"library","std::option::Option",1.0,"","",true,"std::option::Option",1.0,true,false,"Some").is_empty());
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_owned_constructor_plan("named",0.0,"library","std::option::Option",1.0,true,"std::option::Option",1.0,true,"owned","Some"),vec!["sum constructor".to_owned(),"1".to_owned()]);
    assert!(generic_alias_rules::read_rust_generic_type_alias_owned_constructor_plan("named",0.0,"library","std::option::Option",1.0,true,"std::option::Option",1.0,true,"copy","Some").is_empty());
}

#[test]
fn private_sum_alias_headers_are_not_concrete_origin_proofs(){
    let parameters=vec!["actual/T".to_owned()];
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_substitution_plan(&parameters,&[false],&parameters)[0],"error");
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_header_substitution_plan(&parameters,true,&[true],&[false],&[true],&parameters),vec!["substitution".to_owned(),"0".to_owned()]);
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_header_substitution_plan(&parameters,false,&[true],&[false],&[true],&parameters)[0],"error");
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_header_substitution_plan(&parameters,true,&[true],&[false],&[false],&parameters)[0],"error");
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_header_constructor_plan(true,true,"named",0.0,"library","std::option::Option",1.0,"std::option::Option",1.0,0.0,"Some"),vec!["sum constructor".to_owned(),"1".to_owned()]);
    assert!(generic_alias_rules::read_rust_generic_type_alias_header_constructor_plan(false,true,"named",0.0,"library","std::option::Option",1.0,"std::option::Option",1.0,0.0,"Some").is_empty());
    assert!(generic_alias_rules::read_rust_generic_type_alias_header_constructor_plan(true,false,"named",0.0,"library","std::option::Option",1.0,"std::option::Option",1.0,0.0,"Some").is_empty());
}

#[test]
fn concrete_callable_alias_leaves_preserve_actual_pointer_profile_proofs(){
    assert!(generic_alias_rules::accept_rust_generic_type_alias_owned_callable_template(true,"function",0.0,"function",0.0,2.0,1.0,&["owned".into()],"copy",true,true,true,false,90.0));
    assert!(!generic_alias_rules::accept_rust_generic_type_alias_owned_callable_template(true,"function",0.0,"function",0.0,2.0,1.0,&["owned".into()],"copy",true,true,true,true,90.0));
    assert!(!generic_alias_rules::accept_rust_generic_type_alias_owned_callable_template(true,"function",0.0,"function",0.0,2.0,1.0,&["owned".into()],"copy",true,false,true,false,90.0));
    assert!(!generic_alias_rules::accept_rust_generic_type_alias_owned_callable_template(true,"function",0.0,"function",0.0,2.0,1.0,&["owned".into()],"copy",true,true,true,false,100.0));
    assert!(!generic_alias_rules::accept_rust_generic_type_alias_concrete_template("function",0.0,"function","","",false,2.0,true));
    assert!(generic_alias_rules::accept_rust_generic_type_alias_concrete_callable_template(true,"function",0.0,"function",0.0,2.0,1.0,&[true],true,true,true,true,false));
    assert!(!generic_alias_rules::accept_rust_generic_type_alias_concrete_callable_template(true,"function",0.0,"function",0.0,2.0,1.0,&[false],true,true,true,true,false));
    assert!(!generic_alias_rules::accept_rust_generic_type_alias_concrete_callable_template(true,"function",0.0,"function",0.0,2.0,1.0,&[true],true,true,true,true,true));
    assert!(generic_alias_rules::accept_rust_generic_type_alias_header_callable_template(true,"function",0.0,"function",0.0,2.0,1.0,&[false],false,true,true,true,false));
    assert!(!generic_alias_rules::accept_rust_generic_type_alias_header_callable_template(true,"function",0.0,"function",0.0,2.0,1.0,&[false],false,true,false,true,false));
}

#[test]
fn symbolic_function_alias_syntax_keeps_raw_bindings_and_the_real_pointer_budget(){
    assert!(generic_alias_rules::accept_rust_generic_type_alias_function_template_head("fn","fn"));
    assert!(!generic_alias_rules::accept_rust_generic_type_alias_function_template_head("fn","r#fn"));
    assert!(!generic_alias_rules::accept_rust_generic_type_alias_function_template_head("unsafe","unsafe"));
    assert!(generic_alias_rules::accept_rust_generic_type_alias_function_parameter_name("_","_"));
    assert!(!generic_alias_rules::accept_rust_generic_type_alias_function_parameter_name("self","self"));
    assert!(generic_alias_rules::read_rust_generic_type_alias_function_template_error(1024.0).is_empty());
    assert_eq!(generic_alias_rules::read_rust_generic_type_alias_function_template_error(1025.0),"the complete pointer signature exceeds its source budget");
}

#[test]
fn owned_pointer_domain_extension_keeps_copy_profile_and_selected_source_versions(){
    use self::pointer_rules as pointer;
    assert!(!pointer::read_rust_function_pointer_type_error(true,&[false],true).is_empty());
    assert!(pointer::read_rust_function_pointer_owned_domain_error(true,&["owned".to_owned()],"copy",90.0).is_empty());
    assert!(pointer::read_rust_function_pointer_owned_domain_error(true,&["copy".to_owned()],"owned",99.0).is_empty());
    assert!(!pointer::read_rust_function_pointer_owned_domain_error(true,&["copy".to_owned()],"copy",90.0).is_empty());
    assert!(!pointer::read_rust_function_pointer_owned_domain_error(true,&["owned".to_owned()],"copy",100.0).is_empty());
    assert!(!pointer::read_rust_function_pointer_owned_domain_error(true,&["borrowed".to_owned()],"owned",90.0).is_empty());
    assert!(pointer::read_rust_function_pointer_domain_error(true,&[false],true,&["owned".to_owned()],"copy",90.0).is_empty());
}

#[test]
fn defaulted_declarations_and_actual_partial_applications_reach_native_frontend(){
    let options=meta_language::SelfTranslationOptions {semantic_translation:true,..meta_language::SelfTranslationOptions::default()};
    for source in [
        "type Callback<T>=fn(T)->u8;fn main(){}",
        "type Callback<T=fn(String)->u8>=T;fn main(){}",
        "type Mapper<T,U>=fn(T)->U;fn widen(value:u8)->u16{u16::from(value)}pub fn result()->u16{let map:Mapper<u8,u16>=widen;map(7u8)}",
        "type Mapper<T,U>=fn(T)->U;fn widen(value:u8)->u16{u16::from(value)}pub fn result()->u16{let map:Option<Mapper<u8,u16>>=Some(widen);match map{Some(read)=>read(7u8),None=>0u16}}",
        "type Mapper<T,U>=fn(T)->U;type Word<T=u16>=T;pub fn result()->u16{let map:Option<Mapper<u8,u16>>=Some(Word::from);match map{Some(read)=>read(7u8),None=>0u16}}",
        "type Mapper<T,U>=fn(T)->Option<U>;fn widen(value:u8)->Option<u16>{Some(u16::from(value))}pub fn result()->u16{let map:Mapper<u8,u16>=widen;map(7u8).unwrap_or(0u16)}",
        "type Mapper<T,U>=fn(T)->U;type TryMapper<T,U>=Mapper<T,Option<U>>;fn widen(value:u8)->Option<u16>{Some(u16::from(value))}pub fn result()->u16{let map:TryMapper<u8,u16>=widen;map(7u8).unwrap_or(0u16)}",
        "type Action<T>=fn(T);fn done(value:u8){}pub fn result()->u8{let run:Action<u8>=done;run(7u8);7u8}",
        "type Identity<r#fn>=r#fn;pub fn result()->u8{let value:Identity<u8>=7u8;value}",
        "#[derive(Copy,Clone)]struct r#fn{value:u8}type Identity<T=r#fn>=T;pub fn result()->u8{let value:Identity=r#fn{value:7u8};value.value}",
        "type Mapper<T,U>=fn(T)->U;fn read(value:String)->u8{7u8}fn main(){let map:Mapper<String,u8>=read;}",
        "type Callback<T=fn(String)->u8>=T;fn read(value:String)->u8{7u8}fn main(){let map:Callback=read;}",
        "type Identity<T>=T;fn done(value:String){}fn main(){let run:Identity<fn(String)>=done;}",
        "type Mapper<T,U>=fn(T)->Option<U>;fn read(value:u8)->Option<String>{None}fn main(){let map:Mapper<u8,String>=read;}",
        "type Maybe<T=String>=Option<T>;fn main(){let value:Maybe=None;}",
        "type Identity<T=u8>=T;fn main(){}",
        "type Pair<T=u8,U=T>=(T,U);pub fn result()->u16{let value:Pair<u16>=(7u16,9u16);value.1}",
        "type Values<T=u8>=Vec<T>;fn make()->Values{vec![7u8]}pub fn result()->usize{make().len()}",
        "type Values<T=u8>=Vec<T>;pub fn result()->usize{Values::new().len()}",
        "type Maybe<T=u8>=Option<T>;pub fn result()->u8{let value=Maybe::Some(7u8);value.unwrap_or(0u8)}",
        "type Maybe<T>=Option<T>;pub fn result()->u8{let value=Maybe::<u8>::Some(7u8);value.unwrap_or(0u8)}",
        "type Identity<T>=T;pub fn result()->u8{let value:Identity<Option<u8>>=Some(7u8);value.unwrap_or(0u8)}",
        "type Concrete=Option<u8>;type Pair<T>=(T,Concrete);pub fn result()->u8{let value:Pair<u8>=(7u8,Some(9u8));value.1.unwrap_or(0u8)}",
        "type Maybe<T=String>=Option<T>;pub fn result()->usize{let value=Maybe::Some(String::from(\"abc\"));match value{Some(text)=>text.len(),None=>0usize}}",
        "type Maybe<T>=Option<T>;pub fn result()->usize{let word=String::from(\"abc\");let value=Maybe::<String>::Some(word);match value{Some(text)=>text.len(),None=>0usize}}",
        "type Callback<T=fn(u8)->u16>=T;fn widen(value:u8)->u16{u16::from(value)}pub fn result()->u16{let convert:Callback=widen;convert(7u8)}",
        "type Callback<T=fn(u8)->u8>=T;pub fn result()->u8{let increment:Callback=|value|value.wrapping_add(1u8);increment(7u8)}",
        "type Callback<T=fn()>=T;fn done(){}pub fn result()->u8{let run:Callback=done;run();7u8}",
        "type Callback<T=fn()->Option<u8>>=T;fn make()->Option<u8>{Some(7u8)}pub fn result()->u8{let read:Callback=make;read().unwrap_or(0u8)}",
        "type Identity<T>=T;fn make()->Option<u8>{Some(7u8)}pub fn result()->u8{let read:Identity<fn()->Option<u8>>=make;read().unwrap_or(0u8)}",
        "type Outcome<T=u8,E=i64>=Result<T,E>;pub fn result()->u8{let value=Outcome::Err(9i64);value.unwrap_or(7u8)}",
        "type Scalar<T=u8>=T;pub fn result()->u8{Scalar::MAX}",
        "type Scalar<T=u8>=T;pub fn result()->bool{Scalar::MAX<255u8}",
        "type Scalar<T=u8>=T;pub fn result()->u8{let make:fn()->u8=Scalar::default;make()}",
        "type Text<T=String>=T;pub fn result()->usize{Text::from(\"abc\").len()}",
        "#[derive(Copy,Clone)]enum State{Ready}type Value<T=State>=T;pub fn result()->u8{let value=Value::Ready;match value{State::Ready=>7u8}}",
        "type Byte<T=u8>=T;pub fn result()->u8{match 255u8{Byte::MIN=>1u8,Byte::MAX=>2u8,_=>3u8}}",
        "type Byte<T=u8>=T;pub fn result()->u8{match 9u8{Byte::MIN..=Byte::MAX=>7u8}}",
        "#[derive(Copy,Clone)]enum State{Ready}type Value<T=State>=T;pub fn result()->u8{match State::Ready{Value::Ready=>7u8}}",
        "type Maybe<T=u8>=Option<T>;pub fn result()->u8{let value:Maybe=Some(7u8);match value{Maybe::Some(inner)=>inner,Maybe::None=>0u8}}",
        "type Word<T=u16>=T;pub fn result()->u16{let value:Option<u8>=Some(7u8);let mapped=value.map(Word::from);mapped.unwrap_or(0u16)}",
        "type Item<T=u8>=T;pub fn result()->u32{let count:fn(u8)->u32=Item::count_ones;count(7u8)}",
        "struct Value{value:u8}type Alias<T=Value>=T;impl Alias{fn new(value:u8)->Self{Self{value}}fn read(&self)->u8{self.value}}pub fn result()->u8{let value=Alias::new(7u8);value.read()}",
        "type Alias<T=Value>=T;impl Alias{fn read(&self)->u8{self.value}}struct Value{value:u8}pub fn result()->u8{let value=Value{value:7u8};Alias::read(&value)}",
        "mod values{pub struct Value{value:u8}pub type Alias<T=Value>=T;impl Alias{pub fn new(value:u8)->Self{Self{value}}pub fn read(&self)->u8{self.value}}}pub fn result()->u8{let value=values::Alias::new(7u8);value.read()}",
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
        "type Index<T=usize>=T;fn read(value:u64)->u8{match value{Index::MAX=>7u8,_=>0u8}}fn main(){}",
        "type Item<T=usize>=T;pub fn result()->u32{let count:fn(u64)->u32=Item::count_ones;count(7u64)}",
        "type Item<T=usize>=T;pub fn result()->u32{let value:Option<u64>=Some(7u64);let mapped=value.map(Item::count_ones);mapped.unwrap_or(0u32)}",
        "type Alias<T=Missing>=T;impl Alias{fn read()->u8{7u8}}fn main(){}",
        "type Alias<T=Vec<u8>>=T;impl Alias{fn read()->u8{7u8}}fn main(){}",
        "type Identity<T>=T;fn main(){let value:Identity<Option<Absent>>=None;}",
        "type Maybe<T>=Option<T>;pub fn result()->usize{let word=String::from(\"abc\");let value=Maybe::<String>::Some(word);word.len()}",
        "type Callback<T=fn(usize)->u64>=T;fn read(value:u64)->u64{value}pub fn result()->u64{let convert:Callback=read;convert(7usize)}",
        "type Callback<T=fn(u8)->usize>=T;fn read(value:u8)->u64{u64::from(value)}pub fn result()->usize{let convert:Callback=read;convert(7u8)}",
        "type Callback<T=fn(u8)->u8>=T;pub fn result()->u8{let offset=1u8;let read:Callback=|value|value.wrapping_add(offset);read(7u8)}",
        "type Callback<T=&'static fn(u8)->u16>=T;fn main(){}",
        "type Callback<T>=fn(T:u8)->u8;fn main(){}",
        "type Mapper<T,U>=extern \"C\" fn(T)->U;fn main(){}",
        "type Mapper<T,U>=unsafe fn(T)->U;fn main(){}",
        "type Mapper<T,U>=fn(T)->U;fn read(value:&'static String)->u8{7u8}fn main(){let map:Mapper<&'static String,u8>=read;}",
        "struct Value{value:u8}type Alias<T=Value>=T;impl<T> Alias<T>{fn read()->u8{7u8}}fn main(){}",
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
