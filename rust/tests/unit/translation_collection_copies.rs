//! Deferred controls over shared complete JavaScript originals. Native products
//! execute only when CI invokes this test after shared policy generation.
use super::issue_195_translation_pairs::{TemporaryDirectory,run};
use meta_language::translation::{check::check_program,javascript::parse_javascript};
use meta_language::{BinaryLinksOptions,LinkId,LinkNetwork,LinkType,NetworkProjection,ParseConfiguration,TypedProgramLinksEmission,create_typed_program_links,decode_link_network_binary,decode_typed_program_links,emit_typed_program_links,encode_link_network_binary};
use serde_json::{Value,json};
use std::{collections::HashMap,fs,process::Command};
fn emission(value:TypedProgramLinksEmission)->Value{json!({"code":value.code,"language":value.language,"mappings":value.mappings,"assumptions":value.assumptions,"encodings":value.encodings,"theorems":value.theorems})}
fn javascript_stdout(source:&str)->String{
 let script=format!("const lines=[];require('node:vm').runInNewContext({},{{console:{{log:(...values)=>lines.push(values.map(String).join(' '))}}}},{{timeout:1000}});process.stdout.write(lines.length?lines.join('\\n')+'\\n':'');",serde_json::to_string(source).unwrap());
 let observed=run(Command::new("node").args(["-e",&script]));assert!(observed.ok,"{}",observed.stderr);observed.stdout
}
fn concrete_rows(network:&LinkNetwork,root:LinkId)->Vec<Value>{
 let mut children:HashMap<LinkId,Vec<LinkId>>=HashMap::new();let mut fields=HashMap::new();
 for link in network.links(){
  if link.metadata().link_type()==Some(LinkType::Syntax)&&link.metadata().language()==Some("JavaScript"){if let[parent]=link.references(){children.entry(*parent).or_default().push(link.id());}}
  if link.metadata().link_type()==Some(LinkType::Field)&&link.metadata().term().is_none(){if let[parent,label,child]=link.references(){fields.insert((*parent,*child),network.link(*label).unwrap().metadata().term().unwrap().to_owned());}}
 }
 let mut rows=Vec::new();let mut pending=vec![(root,0_usize,None::<String>)];
 while let Some((id,depth,field))=pending.pop(){assert!(rows.len()<100000);let metadata=network.link(id).unwrap().metadata();let flags=metadata.flags();let range=metadata.span().unwrap().byte_range();rows.push(json!([depth,field,metadata.term(),i32::from(metadata.is_named()),range.start(),range.end(),flags.is_error(),flags.is_missing(),flags.is_extra(),flags.has_error()]));for child in children.get(&id).into_iter().flatten().rev(){pending.push((*child,depth+1,fields.get(&(id,*child)).cloned()));}}
 rows
}
pub(super) fn concrete(source:&str)->Value{
 assert!(source.len()<=65536);let network=LinkNetwork::parse(source,"JavaScript",ParseConfiguration::default());assert!(network.verify_full_match(None).is_clean());
 let document=network.links().find(|link|link.metadata().link_type()==Some(LinkType::Document)).unwrap().id();
 let roots=network.links().filter(|link|link.metadata().link_type()==Some(LinkType::Syntax)&&link.metadata().language()==Some("JavaScript")&&link.references()==[document]).collect::<Vec<_>>();assert_eq!(roots.len(),1);let root=roots[0].id();
 let rows=concrete_rows(&network,root);assert!(rows.iter().any(|row|row[0].as_u64().unwrap()>=3));assert_eq!(rows.len(),network.links().filter(|link|link.metadata().link_type()==Some(LinkType::Syntax)&&link.metadata().language()==Some("JavaScript")).count());
 let text=network.to_lino();let binary=encode_link_network_binary(&network,BinaryLinksOptions::default()).unwrap();let ids=network.projected_links(NetworkProjection::ConcreteSyntax).map(meta_language::Link::id).collect::<Vec<_>>();
 for restored in [network.clone(),LinkNetwork::from_lino(&text).unwrap(),decode_link_network_binary(&binary,BinaryLinksOptions::default()).unwrap()]{
  assert_eq!(restored.to_lino(),text);assert_eq!(restored.reconstruct_text().as_bytes(),source.as_bytes());assert_eq!(concrete_rows(&restored,root),rows);assert_eq!(restored.projected_links(NetworkProjection::ConcreteSyntax).map(meta_language::Link::id).collect::<Vec<_>>(),ids);
  for role in [LinkType::Syntax,LinkType::Token,LinkType::Field,LinkType::Trivia]{assert!(restored.projected_links(NetworkProjection::ConcreteSyntax).any(|link|link.metadata().link_type()==Some(role)));}
  for token in restored.links().filter(|link|link.metadata().link_type()==Some(LinkType::Token)&&!link.metadata().flags().is_missing()){let range=token.metadata().span().unwrap().byte_range();assert_eq!(token.metadata().term().unwrap_or("").as_bytes(),&source.as_bytes()[range.start()..range.end()]);}
  let before=restored.to_lino();for link in restored.projected_links(NetworkProjection::AbstractSyntax){assert!(!matches!(link.metadata().link_type(),Some(LinkType::Token|LinkType::Trivia)));for id in link.references(){assert!(restored.link(*id).is_some());}}
  assert_eq!(restored.to_lino(),before);assert_eq!(restored.projected_links(NetworkProjection::ConcreteSyntax).map(meta_language::Link::id).collect::<Vec<_>>(),ids);
 }
 assert_eq!(encode_link_network_binary(&decode_link_network_binary(&binary,BinaryLinksOptions::default()).unwrap(),BinaryLinksOptions::default()).unwrap(),binary);
 json!({"language":"JavaScript","rootTerm":roots[0].metadata().term(),"sourceBytes":source.len(),"rows":rows})
}
#[derive(Default)]
pub(super) struct Diagnostics{assertions:usize,failures:Vec<String>}
impl Diagnostics{
 pub(super) fn attempt<T>(&mut self,label:impl Into<String>,read:impl FnOnce()->T)->Option<T>{
  let label=label.into();self.assertions+=1;
  match std::panic::catch_unwind(std::panic::AssertUnwindSafe(read)){
   Ok(value)=>Some(value),Err(error)=>{
    let message=error.downcast_ref::<String>().map(String::as_str).or_else(||error.downcast_ref::<&str>().copied()).unwrap_or("non-string assertion panic");
    self.failure(format!("{label}: {message}"));None
   }
  }
 }
 fn failure(&mut self,message:String){assert!(self.failures.len()<8192,"native diagnostic failure budget exceeded");self.failures.push(message.chars().take(4096).collect());}
 pub(super) fn producer(&mut self,row:&Value,label:&str){
  let diagnostics=&row["diagnostics"];
  self.attempt(format!("{label}/diagnostic envelope"),||{assert!(diagnostics["assertions"].is_array());assert!(diagnostics["failures"].is_array());});
  if let Some(failures)=diagnostics["failures"].as_array(){for failure in failures{self.failure(format!("JavaScript/{label}/{}: {}",failure["assertion"].as_str().unwrap_or("unrepresented assertion"),failure["message"].as_str().unwrap_or("unrepresented failure")));}}
 }
 pub(super) fn finish(self){assert!(self.failures.is_empty(),"{} failures across {} native assertion attempts:\n{}",self.failures.len(),self.assertions,self.failures.join("\n"));}
}
fn measured<'a,T>(value:&'a Option<T>,label:&str)->&'a T{value.as_ref().unwrap_or_else(||panic!("required measured native product unavailable: {label}"))}
fn peer<'a>(row:&'a Value,key:&str)->&'a Value{row.get(key).unwrap_or_else(||panic!("required measured JavaScript product unavailable: {key}"))}
#[test]
fn diagnostic_failures_keep_later_native_targets_and_dependency_failures(){
 let mut diagnostic=Diagnostics::default();
 let absent=diagnostic.attempt("deliberate first failure",||->usize{panic!("intentional first failure")});
 diagnostic.attempt("deliberate missing measured dependency",||measured(&absent,"first failure"));
 let later=diagnostic.attempt("independent later target",||7_usize);
 assert_eq!(later,Some(7));assert_eq!(diagnostic.assertions,3);assert_eq!(diagnostic.failures.len(),2);
 assert!(diagnostic.failures[0].contains("deliberate first failure"));assert!(diagnostic.failures[1].contains("required measured native product unavailable"));
}
#[test]
fn actual_collection_copies_retain_all_source_graph_transport_and_native_cells(){
 assert_collection_fixture("../js/scripts/produce-collection-copies.mjs","standard same-kind collection copy controls 1");
}
// Shared physical/semantic transport assertions retain every input cell for
// the independent collection source families. This is a future CI consumer.
pub(super) fn assert_collection_fixture(producer_script:&str,model:&str){
 assert_eq!(std::env::var("CI").as_deref(),Ok("true"),"source products compile and run only in CI");
 let mut diagnostics=Diagnostics::default();
 diagnostics.attempt("active Rust target version",||{let compiler=run(Command::new("rustc").arg("--version"));assert!(compiler.ok,"{}",compiler.stderr);let version=compiler.stdout.split_whitespace().nth(1).unwrap();let parts=version.split('.').map(|part|part.parse::<u32>().unwrap()).collect::<Vec<_>>();assert!(parts[0]==1&&(90..100).contains(&parts[1]),"uncovered Rust target compiler {version}; >=1.90 <1.100");});
 let producer=run(Command::new("node").current_dir(env!("CARGO_MANIFEST_DIR")).args(["--max-old-space-size=352","--max-semi-space-size=1",producer_script]));
 // A nonzero producer still supplies all literal rows and available products.
 // Keep its failure, decode the complete envelope and continue independently.
 diagnostics.attempt("JavaScript producer exit",||assert!(producer.ok,"{}",producer.stderr));
 let fixture:Value=serde_json::from_str(&producer.stdout).expect("producer must retain a complete diagnostic envelope even after cell failures");
 diagnostics.attempt("declared fixture model and dimensions",||{assert_eq!(fixture["model"],model);assert_eq!(fixture["targets"],json!(["JavaScript","Rust","Lean","Rocq"]));assert_eq!(fixture["transports"],json!(["original","text","binary"]));});
 diagnostics.producer(&fixture,"header");
 for anchor in fixture["sourceAnchors"].as_array().unwrap(){diagnostics.attempt(format!("source anchor {}",anchor["path"]),||{let path=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join(anchor["path"].as_str().unwrap());assert!(fs::read_to_string(path).unwrap().contains(anchor["source"].as_str().unwrap()));});}
 let directory=TemporaryDirectory::new("collection-copy-products");let mut cells=0_usize;
 for(index,row)in fixture["cases"].as_array().unwrap().iter().enumerate(){
  let name=row["name"].as_str().unwrap_or("unnamed original");diagnostics.producer(row,name);
  let source=diagnostics.attempt(format!("{name}/literal source"),||row["source"].as_str().expect("complete literal source"));
  let expected=diagnostics.attempt(format!("{name}/literal stdout"),||row["stdout"].as_str().expect("complete literal stdout"));
  diagnostics.attempt(format!("{name}/JavaScript original literal stdout"),||assert_eq!(peer(row,"originalStdout"),&row["stdout"]));
  diagnostics.attempt(format!("{name}/JavaScript frontend literal stdout"),||assert_eq!(peer(row,"observedStdout"),&row["stdout"]));
  diagnostics.attempt(format!("{name}/independent original stdout"),||assert_eq!(javascript_stdout(measured(&source,"literal source")),*measured(&expected,"literal stdout")));
  let physical=diagnostics.attempt(format!("{name}/native complete CST and physical transports"),||concrete(measured(&source,"literal source")));
  diagnostics.attempt(format!("{name}/physical graph parity"),||assert_eq!(measured(&physical,"native CST"),peer(row,"concrete")));
  let surface=diagnostics.attempt(format!("{name}/native source parse"),||parse_javascript(measured(&source,"literal source")).unwrap());
  diagnostics.attempt(format!("{name}/source frontend parity"),||assert_eq!(serde_json::to_value(measured(&surface,"native source parse")).unwrap(),*peer(row,"surface")));
  let checked=diagnostics.attempt(format!("{name}/native source check"),||check_program(measured(&surface,"native source parse")).unwrap());
  if let Some(expected)=row.get("prototypeMethodsExpected").and_then(Value::as_bool){
   diagnostics.attempt(format!("{name}/prototype table and checked instance layout"),||{
   let surface=measured(&surface,"native source parse");let checked=measured(&checked,"native source check");
   assert_eq!(!surface.source_class_methods.is_empty(),expected);assert_eq!(surface.source_class_methods.len()%3,0);assert!(serde_json::to_value(&checked).unwrap().get("sourceClassMethods").is_none());
   for method in surface.source_class_methods.chunks_exact(3){
    let Some(meta_language::translation::ir::Decl::Fn(helper))=checked.declaration(&method[2])else{panic!("actual class helper is required")};
    let meta_language::translation::types::Type::Record{fields,..}=&helper.params[0].ty else{panic!("actual branded receiver is required")};assert!(fields.iter().any(|field|field.name==method[0]));
    for declaration in &checked.declarations{if let meta_language::translation::ir::Decl::Fn(function)=declaration{if let meta_language::translation::types::Type::Record{fields,..}=&function.ret{if fields.iter().any(|field|field.name==method[0]){assert!(!fields.iter().any(|field|field.name==method[1]),"prototype method is absent from instance data");}}}}
   }
   });
  }
  for owner in row["singleCallOwners"].as_array().unwrap(){
   diagnostics.attempt(format!("{name}/operand single call/{owner}"),||{
   let function=row["singleCallFunction"].as_str().unwrap_or("copy");
   let Some(meta_language::translation::ir::Decl::Fn(declaration))=measured(&checked,"native source check").declaration(function)else{panic!("actual single-call function {function} is required")};
   let mut pending=vec![&declaration.body];let mut calls=0_usize;
   while let Some(node)=pending.pop(){if let meta_language::translation::ir::Node::Call{func,..}=&node.node{if func==owner.as_str().unwrap(){calls+=1;}}pending.extend(node.children());}
   assert_eq!(calls,1,"{}: actual native source operand call once",row["name"]);
   });
  }
  let packet=diagnostics.attempt(format!("{name}/native typed archive construction"),||create_typed_program_links(measured(&source,"literal source"),"JavaScript").unwrap());
  diagnostics.attempt(format!("{name}/typed root parity"),||assert_eq!(measured(&packet,"native typed archive").root.as_u64(),peer(row,"root").as_u64().unwrap()));
  let text=diagnostics.attempt(format!("{name}/native typed text encoding"),||measured(&packet,"native typed archive").network.to_lino());
  diagnostics.attempt(format!("{name}/typed text parity"),||assert_eq!(measured(&text,"native typed text"),peer(row,"links").as_str().unwrap()));
  let binary=diagnostics.attempt(format!("{name}/native typed binary encoding"),||encode_link_network_binary(&measured(&packet,"native typed archive").network,BinaryLinksOptions::default()).unwrap());
  diagnostics.attempt(format!("{name}/typed binary parity"),||assert_eq!(json!(measured(&binary,"native typed binary")),*peer(row,"binary")));
  let networks=[
   diagnostics.attempt(format!("{name}/original graph availability"),||measured(&packet,"native typed archive").network.clone()),
   diagnostics.attempt(format!("{name}/native typed text reload"),||LinkNetwork::from_lino(measured(&text,"native typed text")).unwrap()),
   diagnostics.attempt(format!("{name}/native typed binary reload"),||decode_link_network_binary(measured(&binary,"native typed binary"),BinaryLinksOptions::default()).unwrap()),
  ];let mut rust_product=None;let mut original_products=HashMap::new();
  for(network,transport)in networks.iter().zip(["original","text","binary"]){
   diagnostics.attempt(format!("{name}/{transport}/exact text graph"),||assert_eq!(measured(network,"native transported graph").to_lino(),*measured(&text,"native typed text")));
   diagnostics.attempt(format!("{name}/{transport}/typed decode"),||decode_typed_program_links(measured(network,"native transported graph"),measured(&packet,"native typed archive").root).unwrap());
   for target in ["JavaScript","Rust","Lean","Rocq"]{
    cells+=1;
    let emitted=diagnostics.attempt(format!("{name}/{transport}/{target}/emission"),||emission(emit_typed_program_links(measured(network,"native transported graph"),measured(&packet,"native typed archive").root,target).unwrap()));
    if transport=="original"{if let Some(product)=&emitted{original_products.insert(target,product.clone());}}
    diagnostics.attempt(format!("{name}/{transport}/{target}/nonempty target"),||assert!(!measured(&emitted,"native emission")["code"].as_str().unwrap().is_empty()));
    diagnostics.attempt(format!("{name}/{transport}/{target}/target parity"),||{let peer=peer(row,"transportEmissions").get(transport).and_then(|products|products.get(target)).expect("required measured JavaScript transport/target product unavailable");assert_eq!(measured(&emitted,"native emission"),peer);});
    if transport=="original"{diagnostics.attempt(format!("{name}/{target}/original target parity"),||assert_eq!(measured(&emitted,"native emission"),peer(row,"emissions").get(target).expect("required measured original JavaScript target unavailable")));}
    else{diagnostics.attempt(format!("{name}/{transport}/{target}/native transport equality"),||assert_eq!(measured(&emitted,"native emission"),original_products.get(target).expect("required measured native original target unavailable")));}
    if target=="JavaScript"{diagnostics.attempt(format!("{name}/{transport}/{target}/literal stdout"),||assert_eq!(javascript_stdout(measured(&emitted,"native emission")["code"].as_str().unwrap()),*measured(&expected,"literal stdout")));}
    if target=="Rust"&&transport=="original"{rust_product=emitted;}
   }
  }
  diagnostics.attempt(format!("{name}/exact binary replay"),||assert_eq!(encode_link_network_binary(measured(&networks[2],"native binary reload"),BinaryLinksOptions::default()).unwrap(),*measured(&binary,"native typed binary")));
  let file=directory.0.join(format!("copy_{index}.rs"));let executable=directory.0.join(format!("copy_{index}{}",std::env::consts::EXE_SUFFIX));
  let built=diagnostics.attempt(format!("{name}/native emitted Rust compilation"),||{
  fs::write(&file,measured(&rust_product,"native original/Rust emission")["code"].as_str().unwrap()).unwrap();let mut compiler=Command::new("rustc");compiler.args(["--edition","2024"]);
  #[cfg(windows)]
  if let Ok(linker)=std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER"){compiler.arg("-C").arg(format!("linker={linker}"));}
  let built=run(compiler.arg(&file).arg("-o").arg(&executable));assert!(built.ok,"{}: {}",row["name"],built.stderr);executable
  });
  diagnostics.attempt(format!("{name}/native emitted Rust literal stdout"),||{let observed=run(&mut Command::new(measured(&built,"native emitted Rust compilation")));assert!(observed.ok,"{}",observed.stderr);assert_eq!(observed.stdout,*measured(&expected,"literal stdout"));});
 }
 diagnostics.attempt("declared original transport/target cardinality",||assert_eq!(cells,fixture["cases"].as_array().unwrap().len()*3*4));
 for row in fixture["refusals"].as_array().unwrap(){
  let name=row["name"].as_str().unwrap_or("unnamed refusal");diagnostics.producer(row,name);
  let source=diagnostics.attempt(format!("{name}/literal source"),||row["source"].as_str().expect("complete literal source"));
  let physical=diagnostics.attempt(format!("{name}/native complete CST and physical transports"),||concrete(measured(&source,"literal source")));
  diagnostics.attempt(format!("{name}/physical graph parity"),||assert_eq!(measured(&physical,"native CST"),peer(row,"concrete")));
  if let Some(expected)=row["originalStdout"].as_str(){
   diagnostics.attempt(format!("{name}/JavaScript original literal stdout"),||assert_eq!(peer(row,"originalObservedStdout"),expected));
   diagnostics.attempt(format!("{name}/independent original stdout"),||assert_eq!(javascript_stdout(measured(&source,"literal source")),expected));
  }
  let error=diagnostics.attempt(format!("{name}/native frontend refusal"),||parse_javascript(measured(&source,"literal source")).and_then(|surface|check_program(&surface)).expect_err("actual complete source boundary"));
  diagnostics.attempt(format!("{name}/refusal classification parity"),||assert_eq!(measured(&error,"native frontend refusal").kind.as_str(),peer(row,"kind").as_str().unwrap()));
  if let Some(reason)=row["reasonIncludes"].as_str(){diagnostics.attempt(format!("{name}/literal refusal diagnostic"),||assert!(measured(&error,"native frontend refusal").reason.contains(reason)));}
  diagnostics.attempt(format!("{name}/native typed archive refusal"),||assert!(create_typed_program_links(measured(&source,"literal source"),"JavaScript").is_err()));
 }
 diagnostics.finish();
}
