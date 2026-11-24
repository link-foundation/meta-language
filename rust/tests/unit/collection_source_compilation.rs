//! Shared generated namespace: compile the actual supplied standard owners.
#[path="../../src/translation/javascript_collection_call_source.rs"]
mod native;
#[path="javascript_collection_call_source_codecs.rs"]
mod codecs;
use codecs::*;
use serde_json::Value;

pub(super) fn assert_collection_source_compilation(input:&Value){
 let directory=decode_java_script_collection_callable_directory(&input["directory"]);
 assert_eq!(encode_java_script_collection_callable_directory(&directory),input["directory"]);
 let actual=native::compile_java_script_collection_call_directory(directory);
 let wire=encode_java_script_collection_call_compilation(&actual);
 assert_eq!(wire["error"],"","every standard operation must compile from its actual owner directory");
 assert_eq!(wire,input["expected"]);
 assert_eq!(encode_java_script_collection_call_compilation(&decode_java_script_collection_call_compilation(&wire)),wire);
}
