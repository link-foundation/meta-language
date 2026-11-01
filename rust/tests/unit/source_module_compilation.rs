//! Separate generated model namespace for actual initialized function captures.
#[path="../../src/translation/javascript_error_class_source.rs"]
mod native;
#[path="javascript_error_class_source_codecs.rs"]
mod codecs;
use codecs::*;
use serde_json::Value;

fn assert_compilation_unit(input:&Value){
 let plan=decode_java_script_error_module_plan(&input["plan"]);
 let registry=decode_java_script_error_module_registry(&input["registry"]);
 assert_eq!(encode_java_script_error_module_plan(&plan),input["plan"]);
 assert_eq!(encode_java_script_error_module_registry(&registry),input["registry"]);
 let bindings=input["bindings"].as_array().expect("actual initialized module bindings").iter().map(|wire|{
  let value=decode_java_script_error_module_binding(wire);
  assert_eq!(encode_java_script_error_module_binding(&value),*wire);
  value
 }).collect::<Vec<_>>();
 let actual=if let Some(owners)=input.get("capabilities"){
  let capabilities=owners.as_array().expect("actual source-qualified capability owners").iter().map(|wire|{
   let owner=decode_java_script_source_capability_owner(wire);
   assert_eq!(encode_java_script_source_capability_owner(&owner),*wire);
   owner
  }).collect::<Vec<_>>();
  native::compile_java_script_error_module_plan_with_capabilities(plan,registry,bindings,capabilities)
 }else{native::compile_java_script_error_module_plan_with_bindings(plan,registry,bindings)};
 let wire=encode_java_script_error_module_compilation(&actual);
 assert_eq!(wire["error"],"","actual initialized captures must compile completely");
 assert_eq!(wire,input["expected"]);
 assert_eq!(encode_java_script_error_module_compilation(&decode_java_script_error_module_compilation(&wire)),wire);
}

// Every linked source unit is independently compiled; the root program alone
// must not stand in for an attached loader/helper implementation.
pub(super) fn assert_source_module_compilation(input:&Value){
 let mut pending=vec![(input,0_usize)];let mut count=0_usize;
 while let Some((unit,depth))=pending.pop(){
  count+=1;assert!(count<=64&&depth<=32,"source unit compilation report exceeds its explicit budget");
  assert_compilation_unit(unit);
  if let Some(children)=unit.get("sourceUnits"){
   for child in children.as_array().unwrap(){
    let source=child["capture"]["source"].as_str().unwrap();assert!(!source.is_empty());
    // This complete native concrete parse also checks physical transports;
    // source-only metadata is not treated as executable semantics evidence.
    super::translation_collection_copies::concrete(source);
    pending.push((&child["compilation"],depth+1));
   }
  }
 }
}
