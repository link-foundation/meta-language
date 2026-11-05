//! Deferred actual frontend/codec/emitter controls; no original executes here.
use meta_language::translation::{check::check_program,rust::parse_rust_with_source_version,emit_rust::emit_rust,emit_lean::emit_lean,emit_rocq::emit_rocq,emit_javascript::emit_javascript};
#[path="../../src/translation/callable_data_derivation_rules.rs"]
mod rules;
#[test]
fn callable_data_derivation_policy_retains_nonobserving_traits_and_old_profiles(){
    assert_eq!(rules::read_callable_data_derivation("Rust",true,false,false,false),"Clone");assert_eq!(rules::read_callable_data_derivation("Lean",true,false,false,false),"Inhabited");
    assert_eq!(rules::read_callable_data_derivation("Rust",false,false,false,false),"Clone, Debug, PartialEq, Eq");assert_eq!(rules::read_callable_data_derivation("Rust",false,true,false,false),"Clone, Debug, PartialEq");
    assert_eq!(rules::read_callable_data_derivation("Lean",false,false,false,false),"Repr, DecidableEq, Inhabited");assert_eq!(rules::read_callable_data_derivation("Lean",false,false,false,true),"Repr, Inhabited");assert_eq!(rules::read_callable_data_derivation("Lean",false,false,true,false),"Inhabited");
    assert!(rules::read_callable_data_profile_error(true,false,true).contains("recursive"));assert!(!rules::read_callable_data_profile_error(true,true,false).is_empty());assert!(rules::read_callable_data_profile_error(false,false,false).is_empty());
    for operation in ["display","comparison","proof"]{assert!(!rules::read_callable_data_observation_error(operation,true).is_empty());assert!(rules::read_callable_data_observation_error(operation,false).is_empty());}
    assert!(rules::read_callable_data_observation_error("call",true).is_empty());assert!(rules::read_callable_data_traversal_error(256.0,65536.0).is_empty());assert!(!rules::read_callable_data_traversal_error(257.0,1.0).is_empty());
    for invalid in [-1.0,0.5,f64::NAN,f64::INFINITY,f64::NEG_INFINITY]{assert!(!rules::read_callable_data_traversal_error(invalid,1.0).is_empty());assert!(!rules::read_callable_data_traversal_error(0.0,invalid).is_empty());}
}
#[test]
fn callable_data_actual_originals_emit_without_identity_debug_or_equality_instances(){
    for source in [
        "type Mapper<T,U>=fn(T)->U;fn widen(value:u8)->u16{u16::from(value)}fn main(){let map:Option<Mapper<u8,u16>>=Some(widen);match map{Some(read)=>println!(\"{}\",read(7u8)),None=>println!(\"none\")}}",
        "fn widen(value:u8)->u16{u16::from(value)}fn main(){let map:Result<fn(u8)->u16,u8>=Ok(widen);match map{Ok(read)=>println!(\"{}\",read(7u8)),Err(error)=>println!(\"{}\",error)}}",
        "enum Callback{Value(fn(u8)->u16)}fn widen(value:u8)->u16{u16::from(value)}fn main(){let map=Callback::Value(widen);match map{Callback::Value(read)=>println!(\"{}\",read(7u8))}}",
        "enum Chain{Link(fn(u8)->u8,Box<Chain>),End}fn read(value:u8)->u8{value}fn main(){let chain=Chain::Link(read,Box::new(Chain::End));match chain{Chain::Link(callback,tail)=>println!(\"{}\",callback(7u8)),Chain::End=>println!(\"empty\")}}",
    ]{for version in ["1.90.0","1.99.0"]{
        let program=check_program(&parse_rust_with_source_version(source,version).unwrap()).unwrap();
        let rust=emit_rust(&program).unwrap();assert!(rust.text.contains("#[derive(Clone)]"));
        let lean=emit_lean(&program).unwrap().text;assert!(lean.contains("deriving Inhabited")||lean.contains("instance : Inhabited"));emit_rocq(&program).unwrap();emit_javascript(&program).unwrap();
        let packet=meta_language::encode_typed_program_links(&program).unwrap();
        let binary_options=meta_language::BinaryLinksOptions::default();
        let binary=meta_language::encode_link_network_binary(&packet.network,binary_options).unwrap();
        for graph in [meta_language::LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),meta_language::decode_link_network_binary(&binary,meta_language::BinaryLinksOptions::default()).unwrap()]{
            let restored=meta_language::decode_typed_program_links(&graph,packet.root).unwrap();assert_eq!(restored,program);
            for target in ["JavaScript","Rust","Lean","Rocq"]{assert_eq!(meta_language::emit_typed_program_links(&graph,packet.root,target).unwrap(),meta_language::emit_typed_program_links(&packet.network,packet.root,target).unwrap());}
        }
    }}
}
#[test]
fn recursive_callable_default_is_a_real_finite_constructor_witness(){
    let kinds=["fixed","fixed","function","reference","constructor","constructor","data","reference"].map(str::to_owned);
    assert_eq!(rules::read_callable_data_recursive_plan(&kinds,&[0.0,0.0,2.0,2.0,4.0,4.0,6.0,6.0],&[0.0,1.0,2.0,3.0,4.0,5.0],&[-1.0,-1.0,-1.0,0.0,-1.0,-1.0,-1.0,0.0],&[6.0],7.0),["profile","true","true","true","1"].map(str::to_owned));
    assert_eq!(rules::render_callable_data_lean_default("Chain","Chain.End",0.0),"instance : Inhabited Chain := ⟨Chain.End⟩");
    assert!(!rules::read_callable_data_profile_error(true,false,true).is_empty());
}
#[test]
fn every_recursive_original_refusal_exercises_each_requested_emission_endpoint(){
    for source in [
        "enum Negative{End,More(fn(Negative)->u8)}fn main(){}",
        "enum Endless{More(fn(u8)->u8,Box<Endless>)}fn main(){}",
        "enum First{End,More(fn(u8)->u8,Box<Second>)}enum Second{More(Box<First>)}fn main(){}",
    ]{for version in ["1.90.0","1.99.0"]{for target in ["Lean","Rust"]{
        let result=parse_rust_with_source_version(source,version).and_then(|surface|check_program(&surface)).and_then(|program|if target=="Lean"{emit_lean(&program)}else{emit_rust(&program)});assert_eq!(result.unwrap_err().kind.as_str(),"unsupported","{target}: {source}");
    }}}
}
#[test]
fn checked_recursive_layouts_reach_both_emitters_independently_without_frontend_refusal_masking(){
    let scalar=serde_json::json!({"kind":"fixed","bits":8,"signed":false});
    let callback=serde_json::json!({"kind":"function","params":[scalar.clone()],"ret":scalar.clone()});
    for items in [
        serde_json::json!([{"k":"data","name":"Negative","ctors":[{"name":"End","fields":[]},{"name":"More","fields":[{"name":"read","type":{"kind":"function","params":[{"kind":"data","name":"Negative"}],"ret":scalar.clone()}}]}]}]),
        serde_json::json!([{"k":"data","name":"Endless","ctors":[{"name":"More","fields":[{"name":"read","type":callback.clone()},{"name":"tail","type":{"kind":"data","name":"Endless"}}]}]}]),
        serde_json::json!([{"k":"data","name":"First","ctors":[{"name":"End","fields":[]},{"name":"More","fields":[{"name":"read","type":callback.clone()},{"name":"tail","type":{"kind":"data","name":"Second"}}]}]},{"k":"data","name":"Second","ctors":[{"name":"More","fields":[{"name":"tail","type":{"kind":"data","name":"First"}}]}]}]),
    ]{
        let surface:meta_language::translation::surface::SProgram=serde_json::from_value(serde_json::json!({"language":"Rust","items":items})).unwrap();let checked=check_program(&surface).unwrap();
        assert_eq!(emit_lean(&checked).unwrap_err().kind.as_str(),"unsupported");assert_eq!(emit_rust(&checked).unwrap_err().kind.as_str(),"unsupported");
    }
}
#[test]
fn callable_aggregate_original_observations_keep_explicit_unsupported_boundary(){
    for source in ["fn read(value:u8)->u8{value}fn main(){let value:Option<fn(u8)->u8>=Some(read);assert_eq!(value,value);}","fn read(value:u8)->u8{value}fn main(){let value:Option<fn(u8)->u8>=Some(read);println!(\"{:?}\",value);}"]{for version in ["1.90.0","1.99.0"]{
        let result=parse_rust_with_source_version(source,version).and_then(|surface|check_program(&surface));assert_eq!(result.unwrap_err().kind.as_str(),"unsupported");
    }}
}
