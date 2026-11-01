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
fn concrete(source:&str,expected:&Value){
 assert!(source.len()<=65536);let network=LinkNetwork::parse(source,"JavaScript",ParseConfiguration::default());assert!(network.verify_full_match(None).is_clean());
 let document=network.links().find(|link|link.metadata().link_type()==Some(LinkType::Document)).unwrap().id();
 let roots=network.links().filter(|link|link.metadata().link_type()==Some(LinkType::Syntax)&&link.metadata().language()==Some("JavaScript")&&link.references()==[document]).collect::<Vec<_>>();assert_eq!(roots.len(),1);let root=roots[0].id();
 assert_eq!(expected["language"],"JavaScript");assert_eq!(expected["rootTerm"].as_str(),roots[0].metadata().term());assert_eq!(expected["sourceBytes"].as_u64().unwrap(),u64::try_from(source.len()).unwrap());
 let rows=concrete_rows(&network,root);assert_eq!(json!(rows),expected["rows"]);assert!(rows.iter().any(|row|row[0].as_u64().unwrap()>=3));assert_eq!(rows.len(),network.links().filter(|link|link.metadata().link_type()==Some(LinkType::Syntax)&&link.metadata().language()==Some("JavaScript")).count());
 let text=network.to_lino();let binary=encode_link_network_binary(&network,BinaryLinksOptions::default()).unwrap();let ids=network.projected_links(NetworkProjection::ConcreteSyntax).map(meta_language::Link::id).collect::<Vec<_>>();
 for restored in [network.clone(),LinkNetwork::from_lino(&text).unwrap(),decode_link_network_binary(&binary,BinaryLinksOptions::default()).unwrap()]{
  assert_eq!(restored.to_lino(),text);assert_eq!(restored.reconstruct_text().as_bytes(),source.as_bytes());assert_eq!(concrete_rows(&restored,root),rows);assert_eq!(restored.projected_links(NetworkProjection::ConcreteSyntax).map(meta_language::Link::id).collect::<Vec<_>>(),ids);
  for role in [LinkType::Syntax,LinkType::Token,LinkType::Field,LinkType::Trivia]{assert!(restored.projected_links(NetworkProjection::ConcreteSyntax).any(|link|link.metadata().link_type()==Some(role)));}
  for token in restored.links().filter(|link|link.metadata().link_type()==Some(LinkType::Token)&&!link.metadata().flags().is_missing()){let range=token.metadata().span().unwrap().byte_range();assert_eq!(token.metadata().term().unwrap_or("").as_bytes(),&source.as_bytes()[range.start()..range.end()]);}
  let before=restored.to_lino();for link in restored.projected_links(NetworkProjection::AbstractSyntax){assert!(!matches!(link.metadata().link_type(),Some(LinkType::Token|LinkType::Trivia)));for id in link.references(){assert!(restored.link(*id).is_some());}}
  assert_eq!(restored.to_lino(),before);assert_eq!(restored.projected_links(NetworkProjection::ConcreteSyntax).map(meta_language::Link::id).collect::<Vec<_>>(),ids);
 }
 assert_eq!(encode_link_network_binary(&decode_link_network_binary(&binary,BinaryLinksOptions::default()).unwrap(),BinaryLinksOptions::default()).unwrap(),binary);
}
#[test]
fn actual_collection_copies_retain_all_source_graph_transport_and_native_cells(){
 assert_eq!(std::env::var("CI").as_deref(),Ok("true"),"source products compile and run only in CI");
 let compiler=run(Command::new("rustc").arg("--version"));assert!(compiler.ok,"{}",compiler.stderr);let version=compiler.stdout.split_whitespace().nth(1).unwrap();let parts=version.split('.').map(|part|part.parse::<u32>().unwrap()).collect::<Vec<_>>();assert!(parts[0]==1&&(90..100).contains(&parts[1]),"uncovered Rust target compiler {version}; >=1.90 <1.100");
 let producer=run(Command::new("node").current_dir(env!("CARGO_MANIFEST_DIR")).args(["--max-old-space-size=352","--max-semi-space-size=1","../js/scripts/produce-collection-copies.mjs"]));assert!(producer.ok,"{}",producer.stderr);let fixture:Value=serde_json::from_str(&producer.stdout).unwrap();
 assert_eq!(fixture["model"],"standard same-kind collection copy controls 1");assert_eq!(fixture["targets"],json!(["JavaScript","Rust","Lean","Rocq"]));assert_eq!(fixture["transports"],json!(["original","text","binary"]));
 eprintln!("collection copy selected source Node {} and target Rust {}",fixture["nodeVersion"],version);
 for anchor in fixture["sourceAnchors"].as_array().unwrap(){let path=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join(anchor["path"].as_str().unwrap());assert!(fs::read_to_string(path).unwrap().contains(anchor["source"].as_str().unwrap()));}
 let directory=TemporaryDirectory::new("collection-copy-products");let mut cells=0_usize;
 for(index,row)in fixture["cases"].as_array().unwrap().iter().enumerate(){
  let source=row["source"].as_str().unwrap();let expected=row["stdout"].as_str().unwrap();assert_eq!(row["originalStdout"],row["stdout"]);assert_eq!(row["observedStdout"],row["stdout"]);assert_eq!(javascript_stdout(source),expected);concrete(source,&row["concrete"]);
  let surface=parse_javascript(source).unwrap();assert_eq!(serde_json::to_value(&surface).unwrap(),row["surface"]);check_program(&surface).unwrap();
  let packet=create_typed_program_links(source,"JavaScript").unwrap();assert_eq!(packet.root.as_u64(),row["root"].as_u64().unwrap());assert_eq!(packet.network.to_lino(),row["links"].as_str().unwrap());let binary=encode_link_network_binary(&packet.network,BinaryLinksOptions::default()).unwrap();assert_eq!(json!(binary),row["binary"]);
  for network in [packet.network.clone(),LinkNetwork::from_lino(row["links"].as_str().unwrap()).unwrap(),decode_link_network_binary(&binary,BinaryLinksOptions::default()).unwrap()]{
   assert_eq!(network.to_lino(),packet.network.to_lino());decode_typed_program_links(&network,packet.root).unwrap();
   for target in ["JavaScript","Rust","Lean","Rocq"]{let emitted=emission(emit_typed_program_links(&network,packet.root,target).unwrap());assert_eq!(emitted,row["emissions"][target]);if target=="JavaScript"{assert_eq!(javascript_stdout(emitted["code"].as_str().unwrap()),expected);}cells+=1;}
  }
  assert_eq!(encode_link_network_binary(&decode_link_network_binary(&binary,BinaryLinksOptions::default()).unwrap(),BinaryLinksOptions::default()).unwrap(),binary);
  let file=directory.0.join(format!("copy_{index}.rs"));let executable=directory.0.join(format!("copy_{index}{}",std::env::consts::EXE_SUFFIX));fs::write(&file,row["emissions"]["Rust"]["code"].as_str().unwrap()).unwrap();let mut compiler=Command::new("rustc");compiler.args(["--edition","2024"]);
  #[cfg(windows)]
  if let Ok(linker)=std::env::var("CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER"){compiler.arg("-C").arg(format!("linker={linker}"));}
  let built=run(compiler.arg(&file).arg("-o").arg(&executable));assert!(built.ok,"{}: {}",row["name"],built.stderr);let observed=run(&mut Command::new(&executable));assert!(observed.ok,"{}",observed.stderr);assert_eq!(observed.stdout,expected);
 }
 assert_eq!(cells,fixture["cases"].as_array().unwrap().len()*3*4);
 for row in fixture["refusals"].as_array().unwrap(){let source=row["source"].as_str().unwrap();concrete(source,&row["concrete"]);if let Some(expected)=row["originalStdout"].as_str(){assert_eq!(row["originalObservedStdout"],expected);assert_eq!(javascript_stdout(source),expected);}let error=parse_javascript(source).and_then(|surface|check_program(&surface)).expect_err("actual complete source boundary");assert_eq!(error.kind.as_str(),row["kind"].as_str().unwrap());if let Some(reason)=row["reasonIncludes"].as_str(){assert!(error.reason.contains(reason));}assert!(create_typed_program_links(source,"JavaScript").is_err());}
}
