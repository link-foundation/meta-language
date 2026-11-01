//! Separate generated model namespace for actual initialized function captures.
#[path="../../src/translation/javascript_error_class_source.rs"]
mod native;
#[path="javascript_error_class_source_codecs.rs"]
mod codecs;
use codecs::*;
use serde_json::Value;

pub(super) fn assert_initialized_function_compilation(input:&Value){
 let plan=decode_java_script_error_module_plan(&input["plan"]);
 let registry=decode_java_script_error_module_registry(&input["registry"]);
 assert_eq!(encode_java_script_error_module_plan(&plan),input["plan"]);
 assert_eq!(encode_java_script_error_module_registry(&registry),input["registry"]);
 let bindings=input["bindings"].as_array().expect("actual initialized module bindings").iter().map(|wire|{
  let value=decode_java_script_error_module_binding(wire);
  assert_eq!(encode_java_script_error_module_binding(&value),*wire);
  value
 }).collect::<Vec<_>>();
 let actual=native::compile_java_script_error_module_plan_with_bindings(plan,registry,bindings);
 let wire=encode_java_script_error_module_compilation(&actual);
 assert_eq!(wire["error"],"","actual initialized captures must compile completely");
 assert_eq!(wire,input["expected"]);
 assert_eq!(encode_java_script_error_module_compilation(&decode_java_script_error_module_compilation(&wire)),wire);
}
