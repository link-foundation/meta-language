//! Deferred complete source-class compiler/machine consumers. These replay
//! actual generated portable frames; Node supplies only the recorded IO realm.
use super::issue_195_translation_pairs::run;
use super::translation_collection_copies::{Diagnostics,concrete};
use super::source_module_compilation::assert_source_module_compilation;
use super::collection_source_compilation::assert_collection_source_compilation;
use super::translation_javascript_call_machine::assert_source_machine_trace;
use serde_json::{Value,json};
use std::{collections::BTreeMap,process::Command,fs,path::Path};

fn actual_class_declaration<'a>(source:&'a str,name:&str)->&'a str{
 let program=meta_language::program_representation::analyze_program(source,"JavaScript",Default::default()).unwrap();
 let bindings=program.bindings().iter().filter(|binding|binding.kind()=="class"&&binding.name()==name&&binding.scope()=="scope:0").collect::<Vec<_>>();assert_eq!(bindings.len(),1);
 let declared=bindings[0].declaration();
 let syntax=program.source_mappings().iter().filter(|mapping|mapping.term()=="class_declaration"&&mapping.range().start()<=declared.start()&&mapping.range().end()>=declared.end()).min_by_key(|mapping|mapping.range().end()-mapping.range().start()).unwrap();
 &source[syntax.range().start()..syntax.range().end()]
}

fn assert_source_closure_inputs(definition:&Value,row:&Value){
 let modules=if let Some(modules)=definition.get("modules"){modules.as_array().unwrap().clone()}else{
  definition["paths"].as_array().unwrap().iter().map(|path|{
   let path=path.as_str().unwrap();let source=fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join(path)).unwrap();json!({"path":path,"source":source})
  }).collect::<Vec<_>>()
 };
 let archive=&row["sourceClosureArchive"],inputs=archive["sourceClosure"]["inputs"].as_array().unwrap();
 assert_eq!(inputs.len(),modules.len());assert_eq!(row["moduleConcrete"].as_array().unwrap().len(),modules.len());
 for module in &modules{
  let inputs=inputs.iter().filter(|input|input["path"]==module["path"]).collect::<Vec<_>>();assert_eq!(inputs.len(),1);assert_eq!(inputs[0]["source"],module["source"]);
  let concrete_rows=row["moduleConcrete"].as_array().unwrap().iter().filter(|input|input["path"]==module["path"]).collect::<Vec<_>>();assert_eq!(concrete_rows.len(),1);
  assert_eq!(concrete(module["source"].as_str().unwrap()),concrete_rows[0]["concrete"]);
 }
 assert_eq!(archive["entryPath"],definition["entryPath"]);assert_eq!(archive["capture"],row["capture"]);
 assert_eq!(row["sourceDefinition"]["publicEntry"],definition["entry"]);
 let entries=archive["entries"].as_array().unwrap().iter().filter(|entry|entry["exported"]==definition["entry"]).collect::<Vec<_>>();assert_eq!(entries.len(),1);
 assert_eq!(entries[0]["compiledName"],row["sourceDefinition"]["entry"]);
 if definition.get("scopeSource").is_some(){
  let anchor=&row["sourceAnchor"],path=definition["completeOriginalScopeModule"].as_str().unwrap();
  let original=fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join(path)).unwrap();
  assert_eq!(anchor["path"],path);assert_eq!(anchor["source"],original);assert_eq!(concrete(&original),anchor["concrete"]);
  let declaration=anchor["declaration"].as_str().unwrap();assert_eq!(actual_class_declaration(&original,definition["className"].as_str().unwrap()),declaration);
  let selected=modules.iter().filter(|module|module["path"]==definition["scopeSource"]).collect::<Vec<_>>();assert_eq!(selected.len(),1);assert_eq!(actual_class_declaration(selected[0]["source"].as_str().unwrap(),definition["className"].as_str().unwrap()),declaration);
 }
}

fn assert_joined_program(input:&Value){
 let source=&input["sourceCompilation"]["expected"]["program"];
 let collection=&input["collectionCompilation"]["expected"]["program"];
 assert_eq!(source["realms"],collection["realms"]);assert_eq!(source["exoticKey"],collection["exoticKey"]);
 let mut functions=BTreeMap::new();let mut programs=vec![source,collection];
 let mut units=vec![&input["sourceCompilation"]];let mut count=0_usize;
 while let Some(unit)=units.pop(){
  count+=1;assert!(count<=64);
  if let Some(children)=unit.get("sourceUnits"){
   for child in children.as_array().unwrap(){let compilation=&child["compilation"];programs.push(&compilation["expected"]["program"]);units.push(compilation);}
  }
 }
 for program in programs{
  assert_eq!(program["realms"],source["realms"]);assert_eq!(program["exoticKey"],source["exoticKey"]);
  for function in program["functions"].as_array().unwrap(){
   let identity=function["identity"].as_str().unwrap().parse::<u64>().unwrap();
   if let Some(previous)=functions.insert(identity,function.clone()){assert_eq!(previous,*function,"joining frames cannot replace an actual source or standard owner");}
  }
 }
 assert_eq!(input["combinedProgram"],json!({"$":"callProgram","functions":functions.into_values().collect::<Vec<_>>(),"realms":source["realms"],"exoticKey":source["exoticKey"]}));
}

#[test]
fn actual_ordinary_classes_keep_full_source_transports_and_native_execution(){
 assert_eq!(std::env::var("CI").as_deref(),Ok("true"),"native source consumers run only in CI");
 let mut diagnostic=Diagnostics::default();
 diagnostic.attempt("active Rust compiler range",||{let output=run(Command::new("rustc").arg("--version"));assert!(output.ok);let version=output.stdout.split_whitespace().nth(1).unwrap();let parts=version.split('.').collect::<Vec<_>>();assert_eq!(parts[0],"1");assert!((90..100).contains(&parts[1].parse::<u32>().unwrap()),"uncovered Rust {version}");});
 let fixture=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/javascript-ordinary-class-source.lino")).unwrap();
 let closure_fixture=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/javascript-ordinary-class-closures.lino")).unwrap();
 let output=run(Command::new("node").current_dir(env!("CARGO_MANIFEST_DIR")).args(["--max-old-space-size=352","--max-semi-space-size=1","../js/scripts/produce-javascript-ordinary-class-source.mjs"]));
 diagnostic.attempt("producer exit",||assert!(output.ok,"{}",output.stderr));
 let peer=diagnostic.attempt("complete bounded producer report",||{assert!(output.stdout.len()<=64*1024*1024);serde_json::from_str::<Value>(&output.stdout).unwrap()});let absent=Value::Null;let peer=peer.as_ref().unwrap_or(&absent);
 diagnostic.producer(peer,"header");diagnostic.attempt("report model",||assert_eq!(peer["model"],"actual ordinary class source execution controls 1"));
 for(group,declarations,export)in [
  ("cases",&fixture["declarations"],"javascriptOrdinaryClassOriginals"),
  ("closures",&closure_fixture["declarations"],"javascriptOrdinaryClassClosureOriginals"),
  ("workloads",&closure_fixture["declarations"],"javascriptOrdinaryClassScopeWorkloads"),
  ("refusals",&fixture["declarations"],"javascriptOrdinaryClassRefusals"),
 ]{
  let definitions=declarations[export].as_array().unwrap();
  diagnostic.attempt(format!("{group}/complete inventory"),||assert_eq!(peer[group].as_array().unwrap().len(),definitions.len()));
  for(index,definition)in definitions.iter().enumerate(){let row=&peer[group][index];let label=format!("{group}/{index}/{}",definition["name"]);
   diagnostic.producer(row,&label);diagnostic.attempt(format!("{label}/exact retained original input"),||assert_eq!(row["definition"],*definition));
   let source_definition=if group=="closures"||group=="workloads"{&row["sourceDefinition"]}else{definition};
   diagnostic.attempt(format!("{label}/independent default CST and physical transports"),||assert_eq!(concrete(source_definition["source"].as_str().unwrap()),row["concrete"]));
   if group=="closures"||group=="workloads"{diagnostic.attempt(format!("{label}/every complete original source and public entry"),||assert_source_closure_inputs(definition,row));}
   if group=="refusals"{continue;}
   let transports=diagnostic.attempt(format!("{label}/every source transport"),||{let rows=row["transports"].as_array().unwrap();assert_eq!(rows.iter().map(|row|row["transport"].as_str().unwrap()).collect::<Vec<_>>(),["original","text","binary"]);rows});
   for transport in transports.into_iter().flatten(){let label=format!("{label}/{}",transport["transport"]);
    diagnostic.attempt(format!("{label}/complete original source receipt"),||{assert_eq!(transport["capture"],row["capture"]);assert_eq!(transport["capture"]["source"],source_definition["source"]);});
    diagnostic.attempt(format!("{label}/actual source compilation"),||assert_source_module_compilation(&transport["sourceCompilation"]));
    diagnostic.attempt(format!("{label}/actual standard collection compilation"),||assert_collection_source_compilation(&transport["collectionCompilation"]));
    diagnostic.attempt(format!("{label}/exact shared owner frame composition"),||assert_joined_program(transport));
    diagnostic.attempt(format!("{label}/actual native source execution and external responses"),||assert_source_machine_trace(&transport["combinedProgram"],&transport["trace"]));
    if definition["expectedBatchResumption"]==true{diagnostic.attempt(format!("{label}/real bounded batch resumption"),||assert!(transport["trace"]["events"].as_array().unwrap().iter().any(|event|event["operation"]=="resumeBatch")));}
    diagnostic.attempt(format!("{label}/independent literal result or exception"),||{
     if definition.get("expectedError").is_some(){assert_eq!(transport["error"],definition["expectedError"]);assert_eq!(transport["trace"]["final"]["$"],"machineThrow");}
     else{assert_eq!(transport["result"],definition["expected"]);assert_eq!(transport["error"],Value::Null);assert_eq!(transport["trace"]["final"]["$"],"machineCompleted");}
     assert_eq!(transport["result"],row["originalResult"]);assert_eq!(transport["error"],row["originalError"]);
    });
   }
  }
 }
 diagnostic.finish();
}
