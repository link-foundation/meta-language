//! Deferred full original/frontend/transport/panic controls. Never run locally
//! during authoring. Each selected native test provisions only its own snapshot.
#[path="../../src/translation/rust_standard_sum_method_rules.rs"]
mod method_rules;
#[path="../../src/translation/rust_panic_value_rules.rs"]
mod panic_rules;
use super::issue_195_translation_pairs::{TemporaryDirectory,run};
use meta_language::translation::check::check_program;
use meta_language::translation::rust::parse_rust_with_source_version;
use meta_language::{BinaryLinksOptions,LinkNetwork,ParseConfiguration,
 create_typed_program_links_with_rust_source_version,decode_link_network_binary,
 decode_typed_program_links,emit_typed_program_links,encode_link_network_binary};
use serde_json::{Value,json};
use std::fs;
use std::path::{Path,PathBuf};
use std::process::Command;

fn text(args:&Value,index:usize)->&str{args[index].as_str().expect("literal source decision text")}
fn decisions(fixture:&Value){
    let rows=fixture["decisions"].as_array().unwrap();assert_eq!(rows.len(),24);
    for row in rows{
        let args=&row["arguments"];
        let actual=match row["operation"].as_str().unwrap(){
            "acceptRustStandardSumOwnedPanicReceiver"=>json!(method_rules::accept_rust_standard_sum_owned_panic_receiver(text(args,0),text(args,1),text(args,2))),
            "readRustStandardSumMethodArity"=>json!(method_rules::read_rust_standard_sum_method_arity(text(args,0),text(args,1))),
            "readRustStandardSumMethodResult"=>json!(method_rules::read_rust_standard_sum_method_result(text(args,0),text(args,1))),
            "readRustStandardSumMethodBranch"=>json!(method_rules::read_rust_standard_sum_method_branch(text(args,0),text(args,1),text(args,2))),
            "readRustStandardSumMethodPanicMessageRole"=>json!(method_rules::read_rust_standard_sum_method_panic_message_role(text(args,0),text(args,1))),
            "readRustStandardSumMethodSourceVersionRange"=>json!(method_rules::read_rust_standard_sum_method_source_version_range(text(args,0),text(args,1))),
            "readRustStandardSumMethodSourceVersionError"=>json!(method_rules::read_rust_standard_sum_method_source_version_error(text(args,0),text(args,1),args[2].as_f64().unwrap())),
            "readRustOptionPanicMessage"=>json!(panic_rules::read_rust_option_panic_message(text(args,0),text(args,1))),
            other=>panic!("uncovered independent Option panic policy {other}"),
        };
        assert_eq!(actual,row["expected"],"actual native shared decision: {row}");
        assert_eq!(row["actual"],row["expected"],"actual future JavaScript shared decision: {row}");
    }
}
fn provision_selected(version:&str){
    assert_eq!(std::env::var("CI").as_deref(),Ok("true"),"native Option panic toolchains and workloads are CI-only");
    assert!(["1.90.0","1.99.0"].contains(&version),"uncovered selected source snapshot {version}");
    let setup=run(Command::new("rustup").args(["toolchain","install",version,"--profile","minimal"]));
    eprintln!("selected Option panic Rust {version} setup stdout:\n{}\nstderr:\n{}",setup.stdout,setup.stderr);
    assert!(setup.ok,"selected source snapshot must install: {}",setup.stderr);
    let identity=run(Command::new("rustup").args(["run",version,"rustc","--version"]));
    assert!(identity.ok,"{}",identity.stderr);assert_eq!(identity.stdout.split_whitespace().nth(1),Some(version));
}
fn producer(version:&str)->Value{
    let result=run(Command::new("node").current_dir(env!("CARGO_MANIFEST_DIR")).args([
        "--max-old-space-size=352","--max-semi-space-size=1","../js/scripts/produce-rust-standard-option-panic.mjs","--source-version",version]));
    assert!(result.ok,"actual Option panic frontend producer failed: {}",result.stderr);
    serde_json::from_str(&result.stdout).expect("runtime-only producer IPC JSON")
}
fn compile(directory:&Path,name:&str,source:&str,version:&str)->PathBuf{
    let input=directory.join(format!("{name}.rs"));let binary=directory.join(format!("{name}{}",std::env::consts::EXE_SUFFIX));
    fs::write(&input,source).unwrap();assert_eq!(fs::read_to_string(&input).unwrap(),source,"the compiled source is exact");
    let output=run(Command::new("rustup").args(["run",version,"rustc","--edition","2024","--crate-type","bin"]).arg(&input).arg("-o").arg(&binary));
    assert!(output.ok,"{name}/{version}: {}",output.stderr);binary
}
fn native_outcome(binary:&Path,panic_expected:bool)->Value{
    let output=run(Command::new(binary).env("RUST_BACKTRACE","0"));
    if !panic_expected{assert!(output.ok,"actual native return failed: {}",output.stderr);return json!({"stdout":output.stdout,"panic":null});}
    assert!(!output.ok,"the declared source runtime panic must occur");
    // Keep the exact complete original main unchanged. For these declared
    // single-line payloads, std snapshots1.90/1.99 print the raw payload on the
    // line after the default main panic-hook location. Unknown framing refuses.
    let lines=output.stderr.lines().collect::<Vec<_>>();
    let header=lines.iter().position(|line|line.starts_with("thread 'main' panicked at ")).expect("actual main panic hook location");
    assert!(lines[header].ends_with(':'),"uncovered panic-hook framing: {}",output.stderr);
    let payload=*lines.get(header+1).expect("actual raw native panic payload");
    assert!(!payload.is_empty(),"the declared panic payload is not empty");
    assert!(lines.iter().skip(header+2).all(|line|line.is_empty()||line.starts_with("note: run with ")),"uncovered native panic-hook tail: {}",output.stderr);
    json!({"stdout":output.stdout,"panic":payload})
}
fn javascript_outcome(code:&str)->Value{
    let script=format!("const lines=[];let failure=null;try{{require('node:vm').runInNewContext({},{{console:{{log:(...v)=>lines.push(v.map(String).join(' '))}},process:{{argv:[]}}}},{{timeout:1000}})}}catch(error){{if(error.name!=='Error'||typeof error.message!=='string')throw error;failure=error.message}}process.stdout.write(JSON.stringify({{stdout:lines.length?lines.join('\\n')+'\\n':'',panic:failure}}));",serde_json::to_string(code).unwrap());
    let result=run(Command::new("node").args(["-e",&script]));assert!(result.ok,"{}",result.stderr);
    serde_json::from_str(&result.stdout).expect("actual JavaScript runtime outcome")
}
fn bytes(value:&Value)->Vec<u8>{value.as_array().unwrap().iter().map(|byte|u8::try_from(byte.as_u64().unwrap()).unwrap()).collect()}
fn assert_no_private(root:&Value){
    let mut pending=vec![root];let mut visited=0usize;
    while let Some(value)=pending.pop(){
        if !value.is_array()&&!value.is_object(){continue;}
        visited+=1;assert!(visited<=100000,"finite complete source traversal");
        assert_ne!(value["k"],"rustStandardSumTry");assert_ne!(value["op"],"rustStandardSumTry");assert_ne!(value["k"],"rustStandardSumSourcePattern");
        if let Some(path)=value["path"].as_array(){assert!(path.iter().all(|part|!part.as_str().is_some_and(|part|part.starts_with("#rust-standard-sum-source"))));}
        match value{Value::Object(fields)=>pending.extend(fields.values()),Value::Array(values)=>pending.extend(values),_=>{}}
    }
}

fn selected_snapshot(version:&str){
    provision_selected(version);let fixture=producer(version);decisions(&fixture);
    assert_eq!(fixture["versions"],json!([version]));assert_eq!(fixture["targets"],json!(["JavaScript","Rust","Lean","Rocq"]));assert_eq!(fixture["transports"],json!(["original","text","binary"]));
    let originals=fixture["cases"].as_array().unwrap();assert_eq!(originals.len(),9);
    assert_eq!(originals.iter().filter(|row|row["panic"].is_string()).count(),5);
    for(index,row)in originals.iter().enumerate(){
        let label=row["name"].as_str().unwrap();let source=row["source"].as_str().unwrap();let panic_expected=row["panic"].is_string();
        let expected=json!({"stdout":row["stdout"],"panic":row.get("panic").cloned().unwrap_or(Value::Null)});
        assert_eq!(row["sourceVersionRange"],">=1.90.0 <1.100.0");assert!(source.len()<=65536);
        if let Some(message)=row["panic"].as_str(){assert!(!message.contains('\n'),"an added multiline fixture requires its complete panic-hook framing control");}
        let directory=TemporaryDirectory::new(&format!("standard_option_panic_{}_{}",version.replace('.',"_"),index));
        let original=compile(&directory.0,"original",source,version);
        assert_eq!(native_outcome(&original,panic_expected),expected,"{label}/{version}: actual unchanged original Rust");
        let concrete=LinkNetwork::parse(source,"Rust",ParseConfiguration::default());
        assert_eq!(concrete.reconstruct_text(),source);assert_eq!(concrete.to_lino(),row["concrete"]["links"].as_str().unwrap());
        let concrete_bytes=encode_link_network_binary(&concrete,BinaryLinksOptions::default()).unwrap();assert_eq!(concrete_bytes,bytes(&row["concrete"]["binary"]));
        for graph in [concrete.clone(),LinkNetwork::from_lino(&concrete.to_lino()).unwrap(),decode_link_network_binary(&concrete_bytes,BinaryLinksOptions::default()).unwrap()]{assert_eq!(graph.to_lino(),concrete.to_lino());assert_eq!(graph.reconstruct_text(),source);}
        assert_eq!(row["selections"].as_array().unwrap().len(),1);let selected=&row["selections"][0];assert_eq!(selected["version"],version);
        // A source refusal is an error here, even for rows declaring a runtime
        // panic. Every case must enter the actual checker and typed graph.
        let surface=parse_rust_with_source_version(source,version).unwrap_or_else(|error|panic!("{label}: {error}"));let raw=serde_json::to_value(&surface).unwrap();assert_no_private(&raw);assert_eq!(raw,selected["surface"]);
        let checked=check_program(&surface).unwrap();
        let packet=create_typed_program_links_with_rust_source_version(source,version,Some(&json!({"fixture":label}))).unwrap();
        assert_eq!(packet.root.as_u64(),selected["root"].as_u64().unwrap());assert_eq!(packet.network.to_lino(),selected["links"].as_str().unwrap());
        let binary=encode_link_network_binary(&packet.network,BinaryLinksOptions::default()).unwrap();assert_eq!(binary,bytes(&selected["binary"]));
        let canonical=emit_typed_program_links(&packet.network,packet.root,"Rust").unwrap();let emitted_binary=compile(&directory.0,"emitted",&canonical.code,version);
        assert_eq!(selected["direct"],expected,"{label}: actual future JavaScript frontend result");assert_eq!(selected["observed"].as_array().unwrap().len(),3);
        let graphs=[packet.network.clone(),LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),decode_link_network_binary(&binary,BinaryLinksOptions::default()).unwrap()];
        for(transport,graph)in graphs.iter().enumerate(){
            assert_eq!(graph.to_lino(),packet.network.to_lino());let restored=decode_typed_program_links(graph,packet.root).unwrap();assert_eq!(restored,checked);assert_no_private(&serde_json::to_value(&restored).unwrap());
            assert_eq!(selected["observed"][transport],expected,"{label}: JavaScript transport observation versus literal");
            for target in ["JavaScript","Rust","Lean","Rocq"]{
                let emitted=emit_typed_program_links(graph,packet.root,target).unwrap();
                assert_eq!(json!({"code":emitted.code,"language":emitted.language,"mappings":emitted.mappings,"assumptions":emitted.assumptions,"encodings":emitted.encodings,"theorems":emitted.theorems}),selected["emissions"][target],"{label}/{version}/{transport}/{target}: all native emission fields");
                if target=="JavaScript"{let code=emit_typed_program_links(graph,packet.root,target).unwrap().code;assert_eq!(javascript_outcome(&code),expected,"{label}/{transport}: actual JavaScript stdout/panic");}
                if target=="Rust"{let code=emit_typed_program_links(graph,packet.root,target).unwrap().code;assert_eq!(code,canonical.code);assert_eq!(native_outcome(&emitted_binary,panic_expected),expected,"{label}/{transport}: actual emitted Rust stdout/panic");}
            }
        }
        assert_eq!(encode_link_network_binary(&graphs[2],BinaryLinksOptions::default()).unwrap(),binary);
    }
    let refusals=fixture["refusals"].as_array().unwrap();assert_eq!(refusals.len(),6);
    for row in refusals{
        assert_eq!(row["selections"].as_array().unwrap().len(),1);let selected=&row["selections"][0];assert_eq!(selected["version"],version);
        let source=row["source"].as_str().unwrap();let error=parse_rust_with_source_version(source,version).and_then(|surface|check_program(&surface)).expect_err("all actual source refusals remain separate from runtime panic cases");
        assert_eq!(error.kind.as_str(),selected["kind"].as_str().unwrap());assert_eq!(error.reason,selected["reason"].as_str().unwrap());assert_eq!(serde_json::to_value(error.span).unwrap(),selected["span"]);
        assert!(create_typed_program_links_with_rust_source_version(source,version,None).is_err());
    }
}

#[test]
fn standard_option_panic_source_rust_1_90(){selected_snapshot("1.90.0");}
#[test]
fn standard_option_panic_source_rust_1_99(){selected_snapshot("1.99.0");}
