//! Deferred actual native/frontend/graph controls over the shared complete
//! original Rust inputs. This module has not run during source drafting.
//! Source-generated sum/expression policy products must be emitted first.
use super::issue_195_translation_pairs::{TemporaryDirectory,run};
use meta_language::translation::check::check_program;
use meta_language::translation::rust::parse_rust_with_source_version;
use meta_language::{BinaryLinksOptions,LinkId,LinkNetwork,LinkType,NetworkProjection,ParseConfiguration,create_typed_program_links_with_rust_source_version,
 decode_link_network_binary,decode_typed_program_links,emit_typed_program_links,encode_link_network_binary};
use serde_json::{Value,json};
use std::fs;
use std::path::{Path,PathBuf};
use std::process::Command;
use std::collections::HashMap;

fn provision_selected_toolchains(){
    assert_eq!(std::env::var("CI").as_deref(),Ok("true"),"endpoint toolchain setup is CI-only");
    let setup=run(Command::new("rustup").args(["toolchain","install","1.90.0","1.99.0","--profile","minimal"]));
    eprintln!("standard sum endpoint setup stdout:\n{}\nstderr:\n{}",setup.stdout,setup.stderr);
    assert!(setup.ok,"all declared native snapshots must install; no transport/version cell is skipped: {}\n{}",setup.stdout,setup.stderr);
}
fn producer()->Value{
    assert_eq!(std::env::var("CI").as_deref(),Ok("true"),"native compilers and integration producers run only in CI");
    let result=run(Command::new("node").current_dir(env!("CARGO_MANIFEST_DIR")).args([
        "--max-old-space-size=352","--max-semi-space-size=1","../js/scripts/produce-rust-standard-sum-integration.mjs"]));
    assert!(result.ok,"{}",result.stderr);serde_json::from_str(&result.stdout).expect("actual complete producer JSON")
}
fn compile(directory:&Path,name:&str,source:&str,toolchain:&str)->PathBuf{
    let file=directory.join(format!("{name}.rs"));let binary=directory.join(format!("{name}{}",std::env::consts::EXE_SUFFIX));
    fs::write(&file,source).unwrap();let output=run(Command::new("rustup").args(["run",toolchain,"rustc","--edition","2024","--crate-type","bin"]).arg(&file).arg("-o").arg(&binary));
    assert!(output.ok,"{name}: {}",output.stderr);binary
}
fn stdout(binary:&Path)->String{let output=run(&mut Command::new(binary));assert!(output.ok,"{}",output.stderr);output.stdout}
fn javascript_stdout(code:&str)->String{
    let script=format!("const lines=[];require('node:vm').runInNewContext({},{{console:{{log:(...v)=>lines.push(v.map(String).join(' '))}},process:{{argv:[]}}}},{{timeout:1000}});process.stdout.write(lines.length?lines.join('\\n')+'\\n':'');",serde_json::to_string(code).unwrap());
    let output=run(Command::new("node").args(["-e",&script]));assert!(output.ok,"{}",output.stderr);output.stdout
}
fn objects<'a>(value:&'a Value,result:&mut Vec<&'a Value>){
    let mut pending=vec![value];while let Some(value)=pending.pop(){assert!(result.len()<100000,"finite fixture graph traversal");result.push(value);match value{Value::Array(values)=>pending.extend(values),Value::Object(values)=>pending.extend(values.values()),_=>{}}}
}
fn assert_no_private(value:&Value,label:&str){
    let mut nodes=Vec::new();objects(value,&mut nodes);for node in nodes{
        assert_ne!(node["k"],"rustStandardSumTry","{label}");assert_ne!(node["op"],"rustStandardSumTry","{label}");
        if let Some(path)=node["path"].as_array(){assert!(path.iter().all(|part|!part.as_str().is_some_and(|part|part.starts_with("#rust-expression-hole:")||part.starts_with("#rust-ordinary-"))),"{label}: private continuation hole");}
    }
}

fn concrete_rows(network:&LinkNetwork,root:LinkId)->Vec<Value>{
    let mut children:HashMap<LinkId,Vec<LinkId>>=HashMap::new();let mut fields=HashMap::new();
    for link in network.links(){
        if link.metadata().link_type()==Some(LinkType::Syntax)&&link.metadata().language()==Some("Rust"){if let[parent]=link.references(){children.entry(*parent).or_default().push(link.id());}}
        if link.metadata().link_type()==Some(LinkType::Field)&&link.metadata().term().is_none(){if let[parent,label,child]=link.references(){fields.insert((*parent,*child),network.link(*label).unwrap().metadata().term().unwrap().to_owned());}}
    }
    let mut rows=Vec::new();let mut pending=vec![(root,0_usize,None::<String>)];
    while let Some((identity,depth,field))=pending.pop(){
        assert!(rows.len()<100000,"finite original concrete tree");let metadata=network.link(identity).unwrap().metadata();let flags=metadata.flags();let range=metadata.span().expect("actual CST source interval").byte_range();
        rows.push(json!([depth,field,metadata.term(),i32::from(metadata.is_named()),range.start(),range.end(),flags.is_error(),flags.is_missing(),flags.is_extra(),flags.has_error()]));
        for child in children.get(&identity).into_iter().flatten().rev(){pending.push((*child,depth+1,fields.get(&(identity,*child)).cloned()));}
    }rows
}
fn assert_concrete_source(source:&str,expected:&Value,label:&str){
    let network=LinkNetwork::parse(source,"Rust",ParseConfiguration::default());assert!(network.verify_full_match(None).is_clean(),"{label}: original source CST recovery");
    let document=network.links().find(|link|link.metadata().link_type()==Some(LinkType::Document)).expect("actual original document").id();
    let roots=network.links().filter(|link|link.metadata().link_type()==Some(LinkType::Syntax)&&link.metadata().language()==Some("Rust")&&link.references()==[document]).collect::<Vec<_>>();
    assert_eq!(roots.len(),1,"{label}: original physical grammar root");let root=roots[0].id();
    assert_eq!(roots[0].metadata().term(),expected["rootTerm"].as_str());assert_eq!(expected["language"],"Rust");assert_eq!(source.len(),usize::try_from(expected["sourceBytes"].as_u64().unwrap()).unwrap());
    let rows=concrete_rows(&network,root);assert_eq!(serde_json::to_value(&rows).unwrap(),expected["rows"],"{label}: full normalized physical tree with fields/order/spans/flags");
    assert_eq!(rows.len(),network.links().filter(|link|link.metadata().link_type()==Some(LinkType::Syntax)&&link.metadata().language()==Some("Rust")).count());
    assert!(rows.iter().any(|row|row[0].as_u64().unwrap()>=3),"actual nested syntax beyond a root with tokens");
    let text=network.to_lino();let binary=encode_link_network_binary(&network,BinaryLinksOptions::default()).unwrap();
    let concrete=network.projected_links(NetworkProjection::ConcreteSyntax).map(meta_language::Link::id).collect::<Vec<_>>();
    for restored in [network.clone(),LinkNetwork::from_lino(&text).unwrap(),decode_link_network_binary(&binary,BinaryLinksOptions::default()).unwrap()]{
        assert_eq!(restored.to_lino(),text,"{label}: every own concrete graph identity/reference/metadata field");assert_eq!(restored.reconstruct_text().as_bytes(),source.as_bytes(),"{label}: exact complete original source bytes");
        assert_eq!(concrete_rows(&restored,root),rows);assert_eq!(restored.projected_links(NetworkProjection::ConcreteSyntax).map(meta_language::Link::id).collect::<Vec<_>>(),concrete);
        for role in [LinkType::Syntax,LinkType::Token,LinkType::Trivia,LinkType::Field]{assert!(restored.projected_links(NetworkProjection::ConcreteSyntax).any(|link|link.metadata().link_type()==Some(role)),"{label}: complete concrete role {role:?}");}
        for token in restored.links().filter(|link|link.metadata().link_type()==Some(LinkType::Token)&&!link.metadata().flags().is_missing()){
            let range=token.metadata().span().expect("actual token source interval").byte_range();assert!(range.start()<=range.end()&&range.end()<=source.len());assert_eq!(token.metadata().term().unwrap_or("").as_bytes(),&source.as_bytes()[range.start()..range.end()],"{label}: actual physical token bytes");
        }
        let before=restored.to_lino();let abstract_view=restored.projected_links(NetworkProjection::AbstractSyntax).collect::<Vec<_>>();
        for link in abstract_view{assert!(!matches!(link.metadata().link_type(),Some(LinkType::Token|LinkType::Trivia)));assert!(std::ptr::eq(link,restored.link(link.id()).unwrap()));for reference in link.references(){assert!(restored.link(*reference).is_some(),"concrete provenance stays resolvable");}}
        assert_eq!(restored.to_lino(),before,"explicit AST projection never mutates complete concrete source");assert_eq!(restored.projected_links(NetworkProjection::ConcreteSyntax).map(meta_language::Link::id).collect::<Vec<_>>(),concrete);
    }
    assert_eq!(encode_link_network_binary(&decode_link_network_binary(&binary,BinaryLinksOptions::default()).unwrap(),BinaryLinksOptions::default()).unwrap(),binary);
}

#[test]
fn actual_standard_sum_originals_retain_all_native_frontend_transport_and_emission_cells(){
    provision_selected_toolchains();
    let fixture=producer();assert_eq!(fixture["versions"],json!(["1.90.0","1.99.0"]));
    assert_eq!(fixture["targets"],json!(["JavaScript","Rust","Lean","Rocq"]));assert_eq!(fixture["transports"],json!(["original","text","binary"]));
    assert_eq!(fixture["nativeControls"].as_array().unwrap().len(),7);
    let version=run(Command::new("rustc").arg("--version"));assert!(version.ok,"{}",version.stderr);
    let active=version.stdout.split_whitespace().nth(1).unwrap();let parts=active.split('.').take(2).map(|part|part.parse::<u32>().unwrap()).collect::<Vec<_>>();
    assert!(parts[0]==1&&(90..100).contains(&parts[1]),"uncovered native source compiler {active}");
    for(index,row)in fixture["cases"].as_array().unwrap().iter().enumerate(){
        let label=row["name"].as_str().unwrap();let source=row["source"].as_str().unwrap();let expected=row["stdout"].as_str().unwrap();
        assert_concrete_source(source,&row["concrete"],label);
        assert_eq!(row["sourceVersionRange"],">=1.90.0 <1.100.0");assert!(source.len()<=65536);
        let directory=TemporaryDirectory::new(&format!("standard_sum_{index}"));
        assert_eq!(row["selections"].as_array().unwrap().len(),fixture["versions"].as_array().unwrap().len());
        for(selection_index,selection)in row["selections"].as_array().unwrap().iter().enumerate(){
            let selected=selection["version"].as_str().unwrap();assert_eq!(selection["version"],fixture["versions"][selection_index]);
            let compiler=run(Command::new("rustup").args(["run",selected,"rustc","--version"]));assert!(compiler.ok,"{label}: the exact declared native snapshot must be installed; {}",compiler.stderr);
            assert_eq!(compiler.stdout.split_whitespace().nth(1),Some(selected),"{label}: actual compiler identity for every declared snapshot");
            let original=compile(&directory.0,&format!("original_{selection_index}"),source,selected);
            assert_eq!(stdout(&original),expected,"{label}/{selected}: actual original Rust stdout");
            assert_eq!(selection["observedStdout"],expected,"{label}: actual future JS producer observation versus literal");
            let surface=parse_rust_with_source_version(source,selected).unwrap_or_else(|error|panic!("{label}/{selected}: {error}"));
            let raw=serde_json::to_value(&surface).unwrap();assert_no_private(&raw,label);assert_eq!(raw,selection["surface"],"{label}/{selected}: full actual surface parity");
            let checked=check_program(&surface).unwrap();
            let packet=create_typed_program_links_with_rust_source_version(source,selected,Some(&json!({"fixture":label}))).unwrap();
            assert_eq!(packet.root.as_u64(),selection["root"].as_u64().unwrap());assert_eq!(packet.network.to_lino(),selection["links"].as_str().unwrap(),"{label}: actual complete typed Links parity");
            let binary=encode_link_network_binary(&packet.network,BinaryLinksOptions::default()).unwrap();assert_eq!(serde_json::to_value(&binary).unwrap(),selection["binary"]);
            let canonical=emit_typed_program_links(&packet.network,packet.root,"Rust").unwrap();let target=compile(&directory.0,&format!("translated_{selection_index}"),&canonical.code,selected);
            let graphs=[packet.network.clone(),LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),decode_link_network_binary(&binary,BinaryLinksOptions::default()).unwrap()];
            assert_eq!(graphs.len(),fixture["transports"].as_array().unwrap().len());
            for(transport,network)in graphs.iter().enumerate(){
                assert_eq!(network.to_lino(),packet.network.to_lino(),"{label}/{transport}: all original typed links");
                let program=decode_typed_program_links(network,packet.root).unwrap();assert_eq!(program,checked,"{label}: actual native checked program replay");
                let value=serde_json::to_value(&program).unwrap();assert_no_private(&value,label);
                let mut nodes=Vec::new();objects(&value,&mut nodes);
                if let Some(owner)=row["nominalOwner"].as_str(){for constructor in ["Some","None"]{assert!(nodes.iter().any(|node|node["k"]=="ctor"&&node["data"]==owner&&node["ctor"]==constructor),"{label}: actual local qualified constructor {owner}.{constructor}");}}
                for called in row["singleCallOwners"].as_array().unwrap(){
                    let owner=row["singleCallFunction"].as_str().unwrap_or("step");
                    let function=nodes.iter().copied().find(|node|node["k"]=="fn"&&node["fullName"]==owner).expect("actual owning function");
                    let mut body=Vec::new();objects(&function["body"],&mut body);assert_eq!(body.iter().filter(|node|node["k"]=="call"&&node["fn"]==*called).count(),1,"{label}: exact single operand call");
                }
                for target_name in fixture["targets"].as_array().unwrap(){
                    let target_name=target_name.as_str().unwrap();let emitted=emit_typed_program_links(network,packet.root,target_name).unwrap();
                    assert_eq!(json!({"code":emitted.code,"language":emitted.language,"mappings":emitted.mappings,"assumptions":emitted.assumptions,"encodings":emitted.encodings,"theorems":emitted.theorems}),selection["emissions"][target_name],"{label}/{selected}/{transport}/{target_name}: every actual emission field");
                    if target_name=="JavaScript"{let output=emit_typed_program_links(network,packet.root,target_name).unwrap();assert_eq!(javascript_stdout(&output.code),expected,"{label}/{transport}: actual JavaScript stdout");}
                    if target_name=="Rust"{let output=emit_typed_program_links(network,packet.root,target_name).unwrap();assert_eq!(output.code,canonical.code,"{label}: exact compiled code association");assert_eq!(stdout(&target),expected,"{label}/{transport}: actual emitted native stdout");}
                }
            }
            assert_eq!(encode_link_network_binary(&graphs[2],BinaryLinksOptions::default()).unwrap(),binary);
        }
    }
    for row in fixture["refusals"].as_array().unwrap(){assert_concrete_source(row["source"].as_str().unwrap(),&row["concrete"],row["name"].as_str().unwrap());for selection in row["selections"].as_array().unwrap(){
        let source=row["source"].as_str().unwrap();let version=selection["version"].as_str().unwrap();
        let error=parse_rust_with_source_version(source,version).and_then(|surface|check_program(&surface)).expect_err("actual original source obligation");
        assert_eq!(error.kind.as_str(),selection["kind"].as_str().unwrap());assert_eq!(error.reason,selection["reason"].as_str().unwrap());assert_eq!(serde_json::to_value(error.span).unwrap(),selection["span"]);
        if let Some(fragment)=row["reasonIncludes"].as_str(){assert!(error.reason.contains(fragment));}
        assert!(create_typed_program_links_with_rust_source_version(source,version,None).is_err(),"all original typed-import refusals remain represented");
    }}
    for previous in fixture["previousRefusals"].as_array().unwrap(){assert!(fixture["cases"].as_array().unwrap().iter().any(|row|row["source"]==previous["source"]),"retain each exact formerly refused original input");}
}
