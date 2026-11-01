//! Future CI consumers retain complete originals plus every malformed input.
//! No native compiler, generator or test was invoked while drafting this file.
use super::issue_195_translation_pairs::run;
use meta_language::translation::{ir::Program,emit_javascript::emit_javascript,emit_rust::emit_rust,emit_lean::emit_lean,emit_rocq::emit_rocq};
use meta_language::{encode_typed_program_links,LinkNetwork,LinkId,BinaryLinksOptions,decode_link_network_binary,decode_typed_program_links,emit_typed_program_links};
use serde_json::{Value,json};
use std::process::Command;
#[test]
fn class_owners_retain_complete_original_cst_codec_and_four_target_requirements(){
 super::translation_collection_copies::assert_collection_fixture(
  "../js/scripts/produce-closed-class-owner-witnesses.mjs",
  "persistent closed class owner controls 1",
 );
}
#[test]
fn edited_owner_inputs_reach_each_native_archive_and_emitter_boundary(){
 assert_eq!(std::env::var("CI").as_deref(),Ok("true"),"source products execute only in CI");
 let producer=run(Command::new("node").current_dir(env!("CARGO_MANIFEST_DIR")).args(["--max-old-space-size=352","--max-semi-space-size=1","../js/scripts/produce-closed-class-owner-witnesses.mjs","--archive-edits-only"]));
 assert!(producer.ok,"{}",producer.stderr);let fixture:Value=serde_json::from_str(&producer.stdout).unwrap();
 assert_eq!(fixture["model"],"persistent closed class owner controls 1");
 let rows=fixture["archiveEdits"].as_array().unwrap();assert_eq!(rows.len(),16);
 let expected=["dropped witness","null witness","empty witness","wrong model","wrong version","wrong constructor reference","wrong helper reference","wrong brand","empty owner","null owner","numeric owner","erased owner","retagged plain record","direct debug observation","nested debug observation","prototype identity observation"];
 for(row,name)in rows.iter().zip(expected){
  assert_eq!(row["name"],name);assert!(row["source"].as_str().unwrap().contains("class Box"));
  assert_eq!(row["assertions"],json!(["checkJavaScriptClassOwners","encodeTypedProgramLinks","emitJavaScript","emitRust","emitLean","emitRocq"]));
  // A malformed typed field can fail in serde itself. Each consumer still
  // receives the original exact JSON cell and must refuse at that boundary or
  // in its semantic owner validator; no malformed cell is skipped.
  for consumer in ["typed archive","JavaScript","Rust","Lean","Rocq"]{
   match serde_json::from_value::<Program>(row["program"].clone()){
    Err(error)=>assert!(!error.to_string().is_empty(),"{name}/{consumer}"),
    Ok(program)=>{
     let rejected=match consumer{
      "typed archive"=>encode_typed_program_links(&program).is_err(),
      "JavaScript"=>emit_javascript(&program).is_err(),
      "Rust"=>emit_rust(&program).is_err(),
      "Lean"=>emit_lean(&program).is_err(),
      "Rocq"=>emit_rocq(&program).is_err(),
      _=>unreachable!(),
     };assert!(rejected,"{name}/{consumer}: edited owner input must refuse");
    }
   }
  }
 }
 let graphs=fixture["graphEdits"].as_array().unwrap();assert_eq!(graphs.len(),3);
 for(row,name)in graphs.iter().zip(["restored direct debug observation","restored nested debug observation","retargeted constructor edge"]){
  assert_eq!(row["name"],name);assert_eq!(row["targets"],json!(["JavaScript","Rust","Lean","Rocq"]));assert_eq!(row["transports"],json!(["original","text","binary"]));
  let root=LinkId::from_u64(row["root"].as_u64().unwrap());
  let bytes=serde_json::from_value::<Vec<u8>>(row["binary"].clone()).unwrap();
  let original=LinkNetwork::from_lino(row["links"].as_str().unwrap()).unwrap();
  for network in [original.clone(),LinkNetwork::from_lino(&original.to_lino()).unwrap(),decode_link_network_binary(&bytes,BinaryLinksOptions::default()).unwrap()]{
   assert_eq!(network.to_lino(),row["links"].as_str().unwrap());assert!(decode_typed_program_links(&network,root).is_err(),"{name}");
   for target in ["JavaScript","Rust","Lean","Rocq"]{assert!(emit_typed_program_links(&network,root,target).is_err(),"{name}/{target}");}
  }
 }
}
