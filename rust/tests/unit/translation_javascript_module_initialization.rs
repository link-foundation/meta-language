//! Deferred native portable initialization and complete source CST consumers.
//! Host IO remains the explicitly qualified Node adapter, not native IO proof.
#[path="javascript_module_initialization_codecs.rs"]
mod codecs;
#[path="../../src/translation/javascript_module_initialization.rs"]
mod native;
use super::source_module_compilation::assert_source_module_compilation;
use codecs::*;
use super::issue_195_translation_pairs::run;
use super::translation_collection_copies::{Diagnostics,concrete};
use serde_json::{Value,json};
use std::process::Command;

fn required<'a,T>(value:&'a Option<T>,name:&str)->&'a T{value.as_ref().unwrap_or_else(||panic!("required measured product unavailable: {name}"))}
fn initial(plan:&native::JavaScriptModuleInitializationPlan,row:&Value)->native::JavaScriptModuleInitializationState{
 assert!(native::read_java_script_module_initialization_plan_error(plan.clone()).is_empty());
 native::create_java_script_module_initialization(plan.clone(),row["moduleURL"].as_str().unwrap().to_owned(),native::ml::Big::parse(row["realm"].as_str().unwrap()))
}
fn event(operation:&str,transition:Value,value:Value,abrupt:bool)->Value{json!({"operation":operation,"transition":transition,"value":value,"abrupt":abrupt})}

// Literal plans and IO responses drive this model independently of the peer's
// trace length, success or availability. The bound is a test resource limit;
// exhaustion is a failed assertion, never a successful module result.
fn drive_literal_model(row:&Value)->Value{
 let plan=decode_java_script_module_initialization_plan(&row["plan"]);
 assert_eq!(encode_java_script_module_initialization_plan(&plan),row["plan"]);
 let mut state=initial(&plan,row);let first=encode_java_script_module_initialization_state(&state);
 let mut events=Vec::new();let mut requests=Vec::new();let mut response=0_usize;let mut terminal=false;
 for _ in 0..8192{
  let transition=native::step_java_script_module_initialization(plan.clone(),state.clone());
  let wire=encode_java_script_module_initialization_transition(&transition);
  assert_eq!(encode_java_script_module_initialization_transition(&decode_java_script_module_initialization_transition(&wire)),wire);
  events.push(event("step",wire.clone(),Value::Null,false));
  state=decode_java_script_module_initialization_state(&wire["state"]);
  match wire["$"].as_str().unwrap(){
   "moduleContinue"=>{},
   "moduleReadData"|"moduleTemporalDeadZone"=>{
    if wire["$"]=="moduleReadData"{requests.push(json!(["load",wire["binding"],wire["specifier"]]));}
    else{requests.push(json!(["tdz",wire["binding"],wire["name"]]));}
    // Repeated stepping does not execute or replay any external operation.
    let waiting=encode_java_script_module_initialization_transition(&native::step_java_script_module_initialization(plan.clone(),state.clone()));
    assert_eq!(waiting["$"],"moduleWaiting");assert_eq!(waiting["state"],wire["state"]);
    let input=&row["responses"][response];response+=1;
    let value=decode_java_script_semantic_value(&input["value"]);assert_eq!(encode_java_script_semantic_value(&value),input["value"]);
    let abrupt=input["abrupt"].as_bool().unwrap();
    let resumed=native::resume_java_script_module_initialization(plan.clone(),state,value,abrupt);
    let wire=encode_java_script_module_initialization_transition(&resumed);
    events.push(event("resume",wire.clone(),input["value"].clone(),abrupt));
    state=decode_java_script_module_initialization_state(&wire["state"]);
    if wire["$"]=="moduleAbrupt"{terminal=true;break;}
    assert_eq!(wire["$"],"moduleContinue");
   },
   "moduleEvaluated"|"moduleAbrupt"=>{terminal=true;break;},
   other=>panic!("literal module model produced unsupported outcome {other}: {wire}"),
  }
 }
 assert!(terminal,"complete literal initialization exceeded its resource limit");
 assert_eq!(response,row["responses"].as_array().unwrap().len());
 let final_state=encode_java_script_module_initialization_state(&state);
 assert_eq!(final_state["phase"],row["expectedPhase"]);assert_eq!(json!(requests),row["expectedRequests"]);
 if let Some(failure)=row.get("expectedFailure"){assert_eq!(&final_state["failure"],failure);}
 for pair in row["aliases"].as_array().unwrap(){assert_eq!(native::test_java_script_module_binding_alias(state.clone(),pair[0].as_f64().unwrap(),pair[1].as_f64().unwrap()),pair[2].as_bool().unwrap());}
 let reads=row["plan"]["bindings"].as_array().unwrap().iter().map(|binding|{
  let actual=native::read_java_script_module_binding(plan.clone(),state.clone(),binding.as_f64().unwrap());
  let wire=encode_java_script_module_binding_read(&actual);
  assert_eq!(encode_java_script_module_binding_read(&decode_java_script_module_binding_read(&wire)),wire);
  json!({"binding":binding,"result":wire})
 }).collect::<Vec<_>>();
 json!({"initialState":first,"events":events,"finalState":final_state,"bindingReads":reads})
}

pub(super) fn replay_peer(diagnostic:&mut Diagnostics,label:&str,row:&Value){
 let plan=diagnostic.attempt(format!("{label}/complete plan transport"),||{let plan=decode_java_script_module_initialization_plan(&row["plan"]);assert_eq!(encode_java_script_module_initialization_plan(&plan),row["plan"]);assert!(native::read_java_script_module_initialization_plan_error(plan.clone()).is_empty());plan});
 let mut state=diagnostic.attempt(format!("{label}/complete initial state transport"),||{let state=decode_java_script_module_initialization_state(&row["initialState"]);assert_eq!(encode_java_script_module_initialization_state(&state),row["initialState"]);assert!(native::read_java_script_module_initialization_state_error(required(&plan,"plan").clone(),state.clone()).is_empty());state});
 let events=diagnostic.attempt(format!("{label}/complete nonempty event inventory"),||{let events=row["events"].as_array().unwrap();assert!(!events.is_empty()&&events.len()<=8192);events});
 for(index,expected)in events.into_iter().flatten().enumerate(){
  state=diagnostic.attempt(format!("{label}/event {index}"),||{
   let plan=required(&plan,"plan").clone();let state=required(&state,"previous state").clone();
   let transition=match expected["operation"].as_str().unwrap(){
    "step"=>{assert!(expected["value"].is_null());assert_eq!(expected["abrupt"],false);native::step_java_script_module_initialization(plan,state)},
    "resume"=>{let value=decode_java_script_semantic_value(&expected["value"]);assert_eq!(encode_java_script_semantic_value(&value),expected["value"]);native::resume_java_script_module_initialization(plan,state,value,expected["abrupt"].as_bool().unwrap())},
    other=>panic!("unrepresented module operation {other}"),
   };
   let wire=encode_java_script_module_initialization_transition(&transition);assert_eq!(wire,expected["transition"]);
   assert_eq!(encode_java_script_module_initialization_transition(&decode_java_script_module_initialization_transition(&wire)),wire);
   decode_java_script_module_initialization_state(&wire["state"])
  });
 }
 diagnostic.attempt(format!("{label}/actual terminal state"),||{let wire=encode_java_script_module_initialization_state(required(&state,"last state"));assert_eq!(wire,row["finalState"]);assert!(["evaluated","errored"].contains(&wire["phase"].as_str().unwrap()));});
 let reads=diagnostic.attempt(format!("{label}/every actual binding read"),||{let reads=row["bindingReads"].as_array().unwrap();let bindings=row["plan"]["bindings"].as_array().unwrap();assert_eq!(reads.len(),bindings.len());for(binding,read)in bindings.iter().zip(reads){assert_eq!(binding,&read["binding"]);}reads});
 for read in reads.into_iter().flatten(){diagnostic.attempt(format!("{label}/binding {}",read["binding"]),||{let actual=native::read_java_script_module_binding(required(&plan,"plan").clone(),required(&state,"last state").clone(),read["binding"].as_f64().unwrap());assert_eq!(encode_java_script_module_binding_read(&actual),read["result"]);assert_eq!(encode_java_script_module_binding_read(&decode_java_script_module_binding_read(&read["result"])),read["result"]);});}
}

#[test]
fn portable_module_initialization_preserves_all_literal_models_and_source_transports(){
 assert_eq!(std::env::var("CI").as_deref(),Ok("true"),"native tests execute only in CI");
 let mut diagnostic=Diagnostics::default();
 diagnostic.attempt("active Rust range",||{let version=run(Command::new("rustc").arg("--version"));assert!(version.ok);let text=version.stdout.split_whitespace().nth(1).unwrap();let parts=text.split('.').collect::<Vec<_>>();assert_eq!(parts[0],"1");assert!((90..100).contains(&parts[1].parse::<u32>().unwrap()),"uncovered Rust {text}");});
 let literals=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/runtime-data-module-initialization.lino")).unwrap();let declarations=&literals["declarations"];
 let output=run(Command::new("node").current_dir(env!("CARGO_MANIFEST_DIR")).args(["--max-old-space-size=352","--max-semi-space-size=1","../js/scripts/produce-runtime-data-module-initialization.mjs"]));
 diagnostic.attempt("producer exit",||assert!(output.ok,"{}",output.stderr));
 let peer=diagnostic.attempt("complete bounded producer report",||{assert!(output.stdout.len()<=32*1024*1024);serde_json::from_str::<Value>(&output.stdout).unwrap()});let absent=Value::Null;let peer=peer.as_ref().unwrap_or(&absent);
 diagnostic.producer(peer,"header");diagnostic.attempt("report model",||assert_eq!(peer["model"],"actual runtime Links policy initialization controls 1"));
 for(group,export)in [("models","runtimeDataModuleModelCases"),("cases","runtimeDataModuleOriginals"),("refusals","runtimeDataModuleRefusals")]{
  let rows=declarations[export].as_array().unwrap();diagnostic.attempt(format!("{group}/full inventory"),||assert_eq!(peer[group].as_array().unwrap().len(),rows.len()));
  for(index,definition)in rows.iter().enumerate(){let row=&peer[group][index];let label=format!("{group}/{index}/{}",definition["name"]);diagnostic.producer(row,&label);diagnostic.attempt(format!("{label}/exact independent input"),||assert_eq!(&row["definition"],definition));
   if group=="models"{let actual=diagnostic.attempt(format!("{label}/independent native literal model"),||drive_literal_model(definition));for field in ["initialState","events","finalState","bindingReads"]{diagnostic.attempt(format!("{label}/{field}/parity"),||assert_eq!(required(&actual,"independent native model")[field],row[field]));}}
   else{diagnostic.attempt(format!("{label}/complete native CST and physical transport"),||assert_eq!(concrete(definition["source"].as_str().unwrap()),row["concrete"]));}
   if group!="refusals"{replay_peer(&mut diagnostic,&label,row);}
   if group=="cases"{
    if definition.get("expectedError").is_some(){diagnostic.attempt(format!("{label}/failed initialization does not publish function captures"),||assert!(row["functionCompilation"].is_null()));}
    else{diagnostic.attempt(format!("{label}/actual initialized source function compilation"),||assert_source_module_compilation(&row["functionCompilation"]));}
   }
  }
 }
 diagnostic.finish();
}
