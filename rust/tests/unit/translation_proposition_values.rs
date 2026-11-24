//! Deferred native Surface/checker/codec/emitter consumers. CI alone executes.
use super::issue_195_translation_pairs::{TemporaryDirectory,execute,run};
use super::translation_collection_copies::Diagnostics;
use meta_language::translation::{check::check_program,surface::SProgram,emit_javascript::emit_javascript,emit_rust::emit_rust,emit_lean::emit_lean,emit_rocq::emit_rocq};
use meta_language::{BinaryLinksOptions,LinkNetwork,decode_link_network_binary,encode_link_network_binary,encode_typed_program_links,decode_typed_program_links,emit_typed_program_links};
use serde_json::Value;
use std::{collections::HashMap,process::Command};
fn measured<'a,T>(value:&'a Option<T>,label:&str)->&'a T{value.as_ref().unwrap_or_else(||panic!("required measured product unavailable: {label}"))}

#[test]
fn every_proposition_surface_reaches_native_checks_codecs_and_all_four_products(){
 assert_eq!(std::env::var("CI").as_deref(),Ok("true"),"native products execute only in CI");
 let mut diagnostic=Diagnostics::default();
 let compiler=run(Command::new("rustc").arg("--version"));
 diagnostic.attempt("active Rust range",||{assert!(compiler.ok);let version=compiler.stdout.split_whitespace().nth(1).unwrap();let parts=version.split('.').collect::<Vec<_>>();assert_eq!(parts[0],"1");let minor=parts[1].parse::<u32>().unwrap();assert!((90..100).contains(&minor),"uncovered Rust {version}; >=1.90 <1.100");});
 // The independent source inventory remains available even if the JS peer
 // cannot build a report; missing peer products remain explicit failures.
 let literals=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/proposition-values.lino")).unwrap();
 let originals=literals["declarations"]["propositionValueSurfacePrograms"].as_array().unwrap();
 let refusals=literals["declarations"]["propositionValueSurfaceRefusals"].as_array().unwrap();
 diagnostic.attempt("complete literal inventory",||{assert_eq!(originals.len(),6);assert_eq!(refusals.len(),3);});
 let producer=run(Command::new("node").current_dir(env!("CARGO_MANIFEST_DIR")).args(["--max-old-space-size=352","--max-semi-space-size=1","../js/scripts/produce-proposition-values.mjs"]));
 diagnostic.attempt("JavaScript producer exit",||assert!(producer.ok,"{}",producer.stderr));
 let peer=diagnostic.attempt("JavaScript complete diagnostic envelope",||serde_json::from_str::<Value>(&producer.stdout).unwrap());
 let absent=Value::Null;
 let header=peer.as_ref().unwrap_or(&absent);diagnostic.producer(header,"header");
 diagnostic.attempt("peer complete declaration",||{assert_eq!(header["model"],"proposition value complete Surface controls 1");assert_eq!(header["cases"].as_array().unwrap().len(),6);assert_eq!(header["refusals"].as_array().unwrap().len(),3);});
 let directory=TemporaryDirectory::new("proposition-surfaces");
 for(index,row)in originals.iter().enumerate(){
  let name=row["name"].as_str().unwrap();let paired=&header["cases"][index];diagnostic.producer(paired,name);
  diagnostic.attempt(format!("{name}/exact peer input"),||{assert_eq!(paired["program"],row["program"]);assert_eq!(paired["stdout"],row["stdout"]);});
  let surface=diagnostic.attempt(format!("{name}/native Surface decode"),||serde_json::from_value::<SProgram>(row["program"].clone()).unwrap());
  let checked=diagnostic.attempt(format!("{name}/actual native check"),||check_program(measured(&surface,"native Surface decode")).unwrap());
  let mut codes=HashMap::new();
  for target in ["JavaScript","Rust","Lean","Rocq"]{
   let emitted=diagnostic.attempt(format!("{name}/{target}/independent native emission"),||{let program=measured(&checked,"native checked program");match target{"JavaScript"=>emit_javascript(program),"Rust"=>emit_rust(program),"Lean"=>emit_lean(program),_=>emit_rocq(program)}.unwrap()});
   if let Some(emitted)=emitted{codes.insert(target,emitted.text);}
  }
  let packet=diagnostic.attempt(format!("{name}/native typed archive"),||encode_typed_program_links(measured(&checked,"native checked program")).unwrap());
  let text=diagnostic.attempt(format!("{name}/native typed text"),||measured(&packet,"native typed archive").network.to_lino());
  let binary=diagnostic.attempt(format!("{name}/native typed binary"),||encode_link_network_binary(&measured(&packet,"native typed archive").network,BinaryLinksOptions::default()).unwrap());
  diagnostic.attempt(format!("{name}/exact JS native graph"),||{assert_eq!(paired["links"].as_str().unwrap(),measured(&text,"native typed text"));assert_eq!(paired["root"].as_u64().unwrap(),measured(&packet,"native typed archive").root.as_u64());assert_eq!(paired["binary"],serde_json::to_value(measured(&binary,"native typed binary")).unwrap());});
  let graphs=[diagnostic.attempt(format!("{name}/original graph"),||measured(&packet,"native typed archive").network.clone()),diagnostic.attempt(format!("{name}/text graph"),||LinkNetwork::from_lino(measured(&text,"native typed text")).unwrap()),diagnostic.attempt(format!("{name}/binary graph"),||decode_link_network_binary(measured(&binary,"native typed binary"),BinaryLinksOptions::default()).unwrap())];
  for(transport,graph)in ["original","text","binary"].into_iter().zip(graphs.iter()){
   diagnostic.attempt(format!("{name}/{transport}/exact physical graph"),||{let graph=measured(graph,"reloaded graph");assert_eq!(&graph.to_lino(),measured(&text,"native typed text"));assert_eq!(&encode_link_network_binary(graph,BinaryLinksOptions::default()).unwrap(),measured(&binary,"native typed binary"));});
   diagnostic.attempt(format!("{name}/{transport}/checked program"),||assert_eq!(&decode_typed_program_links(measured(graph,"reloaded graph"),measured(&packet,"native typed archive").root).unwrap(),measured(&checked,"native checked program")));
   for target in ["JavaScript","Rust","Lean","Rocq"]{
    diagnostic.attempt(format!("{name}/{transport}/{target}/complete peer emission"),||{let emitted=emit_typed_program_links(measured(graph,"reloaded graph"),measured(&packet,"native typed archive").root,target).unwrap();let peer=&paired["transportEmissions"][transport][target];assert_eq!(emitted.code,peer["code"].as_str().unwrap());assert_eq!(emitted.code,*codes.get(target).expect("required independent native code unavailable"));assert_eq!(serde_json::to_value(&emitted.mappings).unwrap(),peer["mappings"]);assert_eq!(serde_json::to_value(&emitted.assumptions).unwrap(),peer["assumptions"]);assert_eq!(serde_json::to_value(&emitted.encodings).unwrap(),peer["encodings"]);assert_eq!(serde_json::to_value(&emitted.theorems).unwrap(),peer["theorems"]);});
   }
  }
  // Every transport asserted byte-identical code above; compile each actual
  // native target once rather than repeating identical toolchain workloads.
  for target in ["JavaScript","Rust","Lean","Rocq"]{diagnostic.attempt(format!("{name}/{target}/compile and execute"),||{let code=codes.get(target).expect("required independent native code unavailable");let result=execute(target,code,&directory.0,&format!("proposition_{index}_{target}"));assert!(result.accepted,"{}",result.diagnostics);assert!(result.ran,"{}",result.diagnostics);assert_eq!(result.lines,row["stdout"].as_str().unwrap().lines().map(str::to_owned).collect::<Vec<_>>());});}
 }
 for(index,row)in refusals.iter().enumerate(){let name=row["name"].as_str().unwrap();let paired=&header["refusals"][index];diagnostic.producer(paired,name);diagnostic.attempt(format!("{name}/exact peer refusal input"),||assert_eq!(paired["program"],row["program"]));diagnostic.attempt(format!("{name}/actual native refusal"),||{let surface=serde_json::from_value::<SProgram>(row["program"].clone()).unwrap();let error=check_program(&surface).expect_err("the exact Surface boundary must refuse");assert!(error.reason.contains(row["reasonIncludes"].as_str().unwrap()),"{}",error.reason);});}
 diagnostic.finish();
}
