//! Deferred complete source-class compiler/machine consumers. These replay
//! actual generated portable frames; Node supplies only the recorded IO realm.
use super::issue_195_translation_pairs::run;
use super::translation_collection_copies::{Diagnostics,concrete};
use super::source_module_compilation::assert_source_module_compilation;
use super::collection_source_compilation::assert_collection_source_compilation;
use super::translation_javascript_call_machine::assert_source_machine_trace;
use serde_json::{Value,json};
use std::{collections::{BTreeMap,BTreeSet},process::Command,fs,path::Path};

fn actual_class_declaration<'a>(source:&'a str,name:&str)->&'a str{
 let program=meta_language::program_representation::analyze_program(source,"JavaScript",Default::default()).unwrap();
 let bindings=program.bindings().iter().filter(|binding|binding.kind()=="class"&&binding.name()==name&&binding.scope()=="scope:0").collect::<Vec<_>>();assert_eq!(bindings.len(),1);
 let declared=bindings[0].declaration();
 let syntax=program.source_mappings().iter().filter(|mapping|mapping.term()=="class_declaration"&&mapping.range().start()<=declared.start()&&mapping.range().end()>=declared.end()).min_by_key(|mapping|mapping.range().end()-mapping.range().start()).unwrap();
 &source[syntax.range().start()..syntax.range().end()]
}

// Re-derive lexical sites, parameter/capture membership and write targets from
// actual native CST fields and bindings, independently of the JS input plan.
fn assert_source_arrow_inputs(capture:&Value){
 let Some(templates)=capture["plan"].get("templates") else{return};
 let source=capture["source"].as_str().unwrap();let program=meta_language::program_representation::analyze_program(source,"JavaScript",Default::default()).unwrap();
 let syntax=program.source_mappings();assert!(syntax.len()<=100000);let network=program.network();
 let by_id=syntax.iter().map(|row|(row.link_id(),row)).collect::<BTreeMap<_,_>>();let mut fields=BTreeMap::new();let mut named=BTreeSet::new();
 for link in network.links(){
  if link.metadata().link_type()==Some(meta_language::LinkType::Syntax)&&link.metadata().is_named()&&!link.metadata().flags().is_extra(){named.insert(link.id().as_u64());}
  if link.metadata().link_type()==Some(meta_language::LinkType::Field)&&link.metadata().term().is_none(){if let[parent,label,child]=link.references(){fields.insert((parent.as_u64(),network.link(*label).unwrap().metadata().term().unwrap().to_owned()),child.as_u64());}}
 }
 assert!(fields.len()<=100000);let mut writes=BTreeSet::new();let mut write_ranges=Vec::new();let mut work=0_usize;
 for assignment in syntax.iter().filter(|row|matches!(row.term(),"assignment_expression"|"augmented_assignment_expression"|"update_expression")){
  let label=if assignment.term()=="update_expression"{"argument"}else{"left"};let mut target=*by_id.get(&fields[&(assignment.link_id(),label.to_owned())]).unwrap();let mut depth=0;
  while target.term()=="parenthesized_expression"{
   depth+=1;assert!(depth<=128);let mut child=None;
   for candidate in syntax{work+=1;assert!(work<=1000000);if candidate.range().start()<=target.range().start()||candidate.range().end()>=target.range().end()||!named.contains(&candidate.link_id()){continue;}if child.is_none_or(|old:&meta_language::program_representation::ProgramSourceMapping|candidate.range().end()-candidate.range().start()>old.range().end()-old.range().start()){child=Some(candidate);}}
   target=child.expect("a parenthesized write needs its actual expression child");
  }
  if matches!(target.term(),"member_expression"|"subscript_expression"){continue;}
  if target.term()!="identifier"{write_ranges.push((target.range().start(),target.range().end()));continue;}writes.insert((target.range().start(),target.range().end()));
 }
 let mut unique=BTreeMap::new();for row in syntax.iter().filter(|row|row.term()=="arrow_function"){unique.insert((row.range().start(),row.range().end()),row);}
 let mut arrows=unique.into_values().collect::<Vec<_>>();arrows.sort_by_key(|row|(row.range().start(),std::cmp::Reverse(row.range().end())));assert!(arrows.len()<=4096);
 let units=|byte:usize|source[..byte].encode_utf16().count();let bindings=program.bindings();
 let mut parameter_owners=BTreeMap::<usize,Vec<usize>>::new();
 for(index,binding)in bindings.iter().enumerate().filter(|(_,binding)|binding.kind()=="parameter"){
  if let Some((owner,_))=arrows.iter().enumerate().filter(|(_,arrow)|arrow.range().start()<=binding.declaration().start()&&arrow.range().end()>=binding.declaration().end()).min_by_key(|(_,arrow)|arrow.range().end()-arrow.range().start()){parameter_owners.entry(owner).or_default().push(index);}
 }

 for template in templates.as_array().unwrap(){
  let arrow=arrows[template["ordinal"].as_u64().unwrap() as usize];let start=arrow.range().start();let end=arrow.range().end();
  let origin=capture["origins"].as_array().unwrap().iter().find(|row|row["identity"]==template["owner"]).unwrap();assert_eq!(origin["coordinateUnit"],"UTF-16 code units");assert!(origin["start"].as_u64().unwrap() as usize<=units(start));assert!(origin["end"].as_u64().unwrap() as usize>=units(end));
  assert!(syntax.iter().any(|row|matches!(row.term(),"function_declaration"|"method_definition"|"class_declaration")&&units(row.range().start())==origin["start"].as_u64().unwrap() as usize&&units(row.range().end())==origin["end"].as_u64().unwrap() as usize));
  let parameter=*by_id.get(fields.get(&(arrow.link_id(),"parameters".to_owned())).or_else(||fields.get(&(arrow.link_id(),"parameter".to_owned()))).expect("actual arrow parameter field")).unwrap();
  let ordinal=template["ordinal"].as_u64().unwrap() as usize;
  let expected_parameters=parameter_owners.get(&ordinal).into_iter().flatten().copied().filter(|index|bindings[*index].declaration().start()>=parameter.range().start()&&bindings[*index].declaration().end()<=parameter.range().end()).collect::<Vec<_>>();
  let mut declared_parameters=Vec::new();
  for value in template["parameters"].as_array().unwrap(){if value["$"]=="errorObjectParameter"{let synthetic=value["identity"].as_u64().unwrap() as usize;assert!(synthetic>=bindings.len());for field in value["fields"].as_array().unwrap(){declared_parameters.push(field["identity"].as_u64().unwrap() as usize);}}else{declared_parameters.push(value["identity"].as_u64().unwrap() as usize);}}
  assert_eq!(declared_parameters,expected_parameters);
  let header=if parameter.term()=="formal_parameters"{network.links().filter(|link|link.metadata().link_type()==Some(meta_language::LinkType::Syntax)&&link.metadata().is_named()&&!link.metadata().flags().is_extra()&&link.references().len()==1&&link.references()[0].as_u64()==parameter.link_id()).map(|link|*by_id.get(&link.id().as_u64()).unwrap()).collect::<Vec<_>>()}else{vec![parameter]};
  let mut header=header;header.sort_by_key(|row|row.range().start());assert_eq!(template["parameters"].as_array().unwrap().len(),header.len());
  let length=header.iter().position(|row|matches!(row.term(),"assignment_pattern"|"rest_pattern")).unwrap_or(header.len());assert_eq!(template["length"].as_u64().unwrap() as usize,length);
  let mut expected_captures=Vec::new();
  for(index,binding)in bindings.iter().enumerate(){
   let declaration=binding.declaration();if binding.scope()=="scope:0"||declaration.start()>=start&&declaration.end()<=end||!binding.references().iter().any(|range|range.start()>=start&&range.end()<=end){continue;}
   assert!(matches!(binding.kind(),"parameter"|"const"|"let"));assert!(declaration.end()<=start);
   if binding.kind()!="parameter"{if let Some(initializer)=syntax.iter().filter(|row|row.term()=="variable_declarator"&&row.range().start()<=declaration.start()&&row.range().end()>=declaration.end()).min_by_key(|row|row.range().end()-row.range().start()){assert!(initializer.range().end()<=start);}}
   for reference in binding.references(){assert!(!writes.contains(&(reference.start(),reference.end()))&&!write_ranges.iter().any(|(start,end)|{work+=1;assert!(work<=1000000);reference.start()>=*start&&reference.end()<=*end}),"an immutable capture cannot hide a native CST binding write");}
   expected_captures.push(index);
  }
  assert_eq!(template["captures"].as_array().unwrap().iter().map(|row|row.as_u64().unwrap() as usize).collect::<Vec<_>>(),expected_captures);
 }
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
 let mut functions=BTreeMap::new();let mut templates=BTreeMap::new();let mut templates_selected=false;let mut programs=vec![source,collection];
 let mut units=vec![&input["sourceCompilation"]];let mut count=0_usize;
 while let Some(unit)=units.pop(){
  count+=1;assert!(count<=64);
  if let Some(children)=unit.get("sourceUnits"){
   for child in children.as_array().unwrap(){let compilation=&child["compilation"];programs.push(&compilation["expected"]["program"]);units.push(compilation);}
  }
 }
 for program in programs{
  assert_eq!(program["realms"],source["realms"]);assert_eq!(program["exoticKey"],source["exoticKey"]);
  if let Some(rows)=program.get("templates"){
   templates_selected=true;
   for template in rows.as_array().unwrap(){let owner=template["owner"].as_str().unwrap().parse::<u64>().unwrap();let ordinal=template["ordinal"].as_u64().unwrap();if let Some(previous)=templates.insert((owner,ordinal),template.clone()){assert_eq!(previous,*template,"joining lexical sites cannot replace their source templates");}}
  }
  for function in program["functions"].as_array().unwrap(){
   let identity=function["identity"].as_str().unwrap().parse::<u64>().unwrap();
   if let Some(previous)=functions.insert(identity,function.clone()){assert_eq!(previous,*function,"joining frames cannot replace an actual source or standard owner");}
  }
 }
 let mut expected=json!({"$":"callProgram","functions":functions.into_values().collect::<Vec<_>>(),"realms":source["realms"],"exoticKey":source["exoticKey"]});
 if templates_selected{expected["templates"]=json!(templates.into_values().collect::<Vec<_>>());}
 assert_eq!(input["combinedProgram"],expected);
}

#[test]
fn actual_ordinary_classes_keep_full_source_transports_and_native_execution(){
 assert_eq!(std::env::var("CI").as_deref(),Ok("true"),"native source consumers run only in CI");
 let mut diagnostic=Diagnostics::default();
 diagnostic.attempt("active Rust compiler range",||{let output=run(Command::new("rustc").arg("--version"));assert!(output.ok);let version=output.stdout.split_whitespace().nth(1).unwrap();let parts=version.split('.').collect::<Vec<_>>();assert_eq!(parts[0],"1");assert!((90..100).contains(&parts[1].parse::<u32>().unwrap()),"uncovered Rust {version}");});
 let fixture=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/javascript-ordinary-class-source.lino")).unwrap();
 let wrapper_materialization_fixture=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/javascript-wrapper-materialization.lino")).unwrap();
 let javascript_object_assignment_coercion_fixture=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/javascript-object-assignment-coercion.lino")).unwrap();
 let javascript_array_mutation_boxing_fixture=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/javascript-array-mutation-boxing.lino")).unwrap();
 let javascript_array_read_boxing_fixture=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/javascript-array-read-boxing.lino")).unwrap();
 let array_concatenation_fixture=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/javascript-array-concatenation-source.lino")).unwrap();
 let array_copy_fixture=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/javascript-array-copy-source.lino")).unwrap();
 let array_slice_fixture=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/javascript-array-slice-source.lino")).unwrap();
 let array_join_fixture=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/javascript-array-join-source.lino")).unwrap();
 let construction_fixture=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/javascript-construction-source.lino")).unwrap();
 let array_creation_fixture=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/javascript-array-creation.lino")).unwrap();
 let primitive_wrapper_fixture=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/javascript-primitive-wrappers.lino")).unwrap();
 let object_from_entries_fixture=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/javascript-object-from-entries-source.lino")).unwrap();
 let string_case_fixture=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/javascript-string-case-profile.lino")).unwrap();
 let collection_for_each_fixture=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/javascript-collection-for-each-source.lino")).unwrap();
 let closure_fixture=meta_language::read_structured_data_lino(include_str!("../../../parity/fixtures/javascript-ordinary-class-closures.lino")).unwrap();
 let output=run(Command::new("node").current_dir(env!("CARGO_MANIFEST_DIR")).args(["--max-old-space-size=352","--max-semi-space-size=1","../js/scripts/produce-javascript-ordinary-class-source.mjs"]));
 diagnostic.attempt("producer exit",||assert!(output.ok,"{}",output.stderr));
 let peer=diagnostic.attempt("complete bounded producer report",||{assert!(output.stdout.len()<=64*1024*1024);serde_json::from_str::<Value>(&output.stdout).unwrap()});let absent=Value::Null;let peer=peer.as_ref().unwrap_or(&absent);
 diagnostic.producer(peer,"header");diagnostic.attempt("report model",||assert_eq!(peer["model"],"actual ordinary class source execution controls 1"));
 for(group,declarations,export)in [
  ("cases",&fixture["declarations"],"javascriptOrdinaryClassOriginals"),
  ("objectAssignmentCoercion",&javascript_object_assignment_coercion_fixture["declarations"],"javascriptObjectAssignmentCoercionOriginals"),
  ("arrayMutationBoxing",&javascript_array_mutation_boxing_fixture["declarations"],"javascriptArrayMutationBoxingOriginals"),
  ("arrayReadBoxing",&javascript_array_read_boxing_fixture["declarations"],"javascriptArrayReadBoxingOriginals"),
  ("arrayConcatenation",&array_concatenation_fixture["declarations"],"javascriptArrayConcatenationOriginals"),
  ("arrayCopy",&array_copy_fixture["declarations"],"javascriptArrayCopyOriginals"),
  ("arraySlice",&array_slice_fixture["declarations"],"javascriptArraySliceOriginals"),
  ("arrayJoin",&array_join_fixture["declarations"],"javascriptArrayJoinOriginals"),
  ("construction",&construction_fixture["declarations"],"javascriptConstructionOriginals"),
  ("arrayCreation",&array_creation_fixture["declarations"],"javascriptArrayCreationOriginals"),
  ("wrapperMaterialization",&wrapper_materialization_fixture["declarations"],"javascriptWrapperMaterializationOriginals"),
  ("primitiveWrappers",&primitive_wrapper_fixture["declarations"],"javascriptPrimitiveWrapperOriginals"),
  ("objectFromEntries",&object_from_entries_fixture["declarations"],"javascriptObjectFromEntriesOriginals"),
  ("collectionForEach",&collection_for_each_fixture["declarations"],"javascriptCollectionForEachOriginals"),
  ("stringCaseProfiles",&string_case_fixture["declarations"],"javascriptStringCaseProfileOriginals"),
  ("closures",&closure_fixture["declarations"],"javascriptOrdinaryClassClosureOriginals"),
  ("workloads",&closure_fixture["declarations"],"javascriptOrdinaryClassScopeWorkloads"),
  ("refusals",&fixture["declarations"],"javascriptOrdinaryClassRefusals"),
 ]{
  if group=="stringCaseProfiles"{diagnostic.attempt("active Unicode16 original case profile",||{assert_eq!(peer["nodeVersion"].as_str().unwrap().split('.').next(),Some("24"),"uncovered Node range for Unicode16 original casing");assert_eq!(peer["unicodeVersion"],"16.0","uncovered original Unicode profile");});}
  let definitions=declarations[export].as_array().unwrap();
  diagnostic.attempt(format!("{group}/complete inventory"),||assert_eq!(peer[group].as_array().unwrap().len(),definitions.len()));
  for(index,definition)in definitions.iter().enumerate(){let row=&peer[group][index];let label=format!("{group}/{index}/{}",definition["name"]);
   diagnostic.producer(row,&label);diagnostic.attempt(format!("{label}/exact retained original input"),||assert_eq!(row["definition"],*definition));
   let source_definition=if group=="closures"||group=="workloads"{&row["sourceDefinition"]}else{definition};
   diagnostic.attempt(format!("{label}/independent default CST and physical transports"),||assert_eq!(concrete(source_definition["source"].as_str().unwrap()),row["concrete"]));
   if group=="closures"||group=="workloads"{diagnostic.attempt(format!("{label}/every complete original source and public entry"),||assert_source_closure_inputs(definition,row));}
   if group=="refusals"{continue;}
   diagnostic.attempt(format!("{label}/actual native arrow lexical site and immutable capture proof"),||assert_source_arrow_inputs(&row["capture"]));
   let transports=diagnostic.attempt(format!("{label}/every source transport"),||{let rows=row["transports"].as_array().unwrap();assert_eq!(rows.iter().map(|row|row["transport"].as_str().unwrap()).collect::<Vec<_>>(),["original","text","binary"]);rows});
   for transport in transports.into_iter().flatten(){let label=format!("{label}/{}",transport["transport"]);
    diagnostic.attempt(format!("{label}/complete original source receipt"),||{assert_eq!(transport["capture"],row["capture"]);assert_eq!(transport["capture"]["source"],source_definition["source"]);});
    diagnostic.attempt(format!("{label}/actual source compilation"),||assert_source_module_compilation(&transport["sourceCompilation"]));
    diagnostic.attempt(format!("{label}/actual standard collection compilation"),||assert_collection_source_compilation(&transport["collectionCompilation"]));
    diagnostic.attempt(format!("{label}/exact shared owner frame composition"),||assert_joined_program(transport));
    diagnostic.attempt(format!("{label}/actual native source execution and external responses"),||assert_source_machine_trace(&transport["combinedProgram"],&transport["trace"]));
    if definition["expectedBatchResumption"]==true{diagnostic.attempt(format!("{label}/real bounded batch resumption"),||assert!(transport["trace"]["events"].as_array().unwrap().iter().any(|event|event["operation"]=="resumeBatch")));}
    if let Some(expected)=definition.get("expectedArgumentState"){diagnostic.attempt(format!("{label}/every retained argument effect"),||{assert_eq!(&transport["argumentState"],expected);assert_eq!(&row["originalArgumentState"],expected);});}
    diagnostic.attempt(format!("{label}/independent literal result or exception"),||{
     if let Some(expected)=definition.get("expectedError"){for(key,value)in expected.as_object().unwrap(){assert_eq!(&transport["error"][key],value);}assert_eq!(transport["trace"]["final"]["$"],"machineThrow");}
     else{assert_eq!(transport["result"],definition["expected"]);assert_eq!(transport["error"],Value::Null);assert_eq!(transport["trace"]["final"]["$"],"machineCompleted");}
     assert_eq!(transport["result"],row["originalResult"]);assert_eq!(transport["error"],row["originalError"]);
    });
   }
  }
 }
 diagnostic.finish();
}
