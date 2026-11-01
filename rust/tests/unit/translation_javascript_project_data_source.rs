//! Deferred source/transport/compiler consumers. Raw host IO remains the
//! explicitly qualified Node provider; these assertions do not claim native IO.
use super::issue_195_translation_pairs::run;
use super::translation_collection_copies::{Diagnostics,concrete};
use super::source_module_compilation::assert_source_module_compilation;
use super::translation_javascript_module_initialization::replay_peer;
use serde_json::Value;
use std::{fs,path::Path,process::Command};
fn required<'a,T>(value:&'a Option<T>,name:&str)->&'a T{value.as_ref().unwrap_or_else(||panic!("required measured product unavailable: {name}"))}

#[test]
fn actual_project_data_sources_keep_every_input_transport_and_native_compiler_product(){
 assert_eq!(std::env::var("CI").as_deref(),Ok("true"),"native consumers run only in CI");
 let mut diagnostic=Diagnostics::default();
 diagnostic.attempt("active Rust compiler range",||{let version=run(Command::new("rustc").arg("--version"));assert!(version.ok);let text=version.stdout.split_whitespace().nth(1).unwrap();let parts=text.split('.').collect::<Vec<_>>();assert_eq!(parts[0],"1");assert!((90..100).contains(&parts[1].parse::<u32>().unwrap()),"uncovered Rust {text}");});
 let fixture=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/project-data-source-execution.lino")).unwrap();
 let importer=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/runtime-data-module-initialization.lino")).unwrap();
 let source=diagnostic.attempt("independent complete current provider source",||fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../js/src/project-data-files.js")).unwrap());
 let physical=diagnostic.attempt("independent provider full CST and physical transports",||concrete(required(&source,"current provider source")));
 let output=run(Command::new("node").current_dir(env!("CARGO_MANIFEST_DIR")).args(["--max-old-space-size=352","--max-semi-space-size=1","../js/scripts/produce-javascript-project-data-source.mjs"]));
 diagnostic.attempt("producer exit",||assert!(output.ok,"{}",output.stderr));
 let peer=diagnostic.attempt("complete bounded producer report",||{assert!(output.stdout.len()<=64*1024*1024);serde_json::from_str::<Value>(&output.stdout).unwrap()});let absent=Value::Null;let peer=peer.as_ref().unwrap_or(&absent);
 diagnostic.producer(peer,"header");diagnostic.attempt("report model",||assert_eq!(peer["model"],"actual project data source execution controls 1"));
 diagnostic.attempt("every retained source requirement",||assert_eq!(peer["requirements"],fixture["declarations"]["projectDataSourceRequiredControls"]));
 for(group,definitions)in [
  ("cases",&fixture["declarations"]["projectDataSourceOriginals"]),
  ("cacheCases",&fixture["declarations"]["projectDataSourceCacheOriginals"]),
  ("refusals",&fixture["declarations"]["projectDataSourceRefusals"]),
  ("importerCases",&importer["declarations"]["runtimeDataModuleOriginals"]),
 ]{
  let definitions=definitions.as_array().expect("complete independent literal inventory");
  diagnostic.attempt(format!("{group}/complete peer inventory"),||assert_eq!(peer[group].as_array().unwrap().len(),definitions.len()));
  for(index,definition)in definitions.iter().enumerate(){let row=&peer[group][index];let label=format!("{group}/{index}/{}",definition["name"]);
   diagnostic.producer(row,&label);diagnostic.attempt(format!("{label}/exact complete input"),||assert_eq!(&row["definition"],definition));
   if group=="refusals"{
    diagnostic.attempt(format!("{label}/independent refused source CST and transports"),||assert_eq!(concrete(definition["source"].as_str().unwrap()),row["concrete"]));
    continue;
   }
   if group=="cacheCases"{
    // The exact independently declared padding recipe avoids storing an 8 MiB
    // fixture or flattening large VM strings into every diagnostic snapshot.
    for key in ["values","originalValues"]{diagnostic.attempt(format!("{label}/{key}/literal cache values"),||assert_eq!(row[key],definition["expectedValues"]));}
    for(key,expected)in [("roots","expectedDistinctRootHandles"),("nested","expectedDistinctNestedHandles")]{diagnostic.attempt(format!("{label}/{key}/actual fresh identities"),||{let ids=row[key].as_array().unwrap();assert_eq!(ids.len(),3);let distinct=ids.iter().map(|value|value.as_str().unwrap()).collect::<std::collections::HashSet<_>>().len()==ids.len();assert_eq!(distinct,definition[expected].as_bool().unwrap());});}
    continue;
   }
   if group=="importerCases"{
    diagnostic.attempt(format!("{label}/independent importer default CST and transports"),||assert_eq!(concrete(definition["source"].as_str().unwrap()),row["concrete"]));
    replay_peer(&mut diagnostic,&label,row);
    continue;
   }
   diagnostic.attempt(format!("{label}/complete actual source bytes"),||assert_eq!(row["capture"]["source"].as_str().unwrap(),required(&source,"current provider source")));
   diagnostic.attempt(format!("{label}/provider CST parity"),||assert_eq!(&row["concrete"],required(&physical,"provider CST")));
   let transports=diagnostic.attempt(format!("{label}/every source transport"),||{let transports=row["transports"].as_array().unwrap();assert_eq!(transports.iter().map(|row|row["transport"].as_str().unwrap()).collect::<Vec<_>>(),["original","text","binary"]);transports});
   for transport in transports.into_iter().flatten(){let transport_label=format!("{label}/{}",transport["transport"]);
    diagnostic.attempt(format!("{transport_label}/exact restored source plan"),||assert_eq!(transport["capture"],row["capture"]));
    diagnostic.attempt(format!("{transport_label}/actual generated native capability compiler"),||assert_source_module_compilation(&transport["compilation"]));
    diagnostic.attempt(format!("{transport_label}/complete original completion parity"),||{assert_eq!(transport["result"],row["originalResult"]);assert_eq!(transport["error"],row["originalError"]);});
    diagnostic.attempt(format!("{transport_label}/retained actual host operation inputs"),||assert!(!transport["events"].as_array().unwrap().is_empty()));
   }
  }
 }
 diagnostic.finish();
}
