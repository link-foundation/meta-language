// Native consumers are generated from the existing, authenticated graph test
// and process transport. All semantic behavior stays in the real package APIs.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {writeProjectData,checkProjectData} from './project-data.mjs';
import {generateConcreteProgramGraphFixtures} from './generate-concrete-program-graph-fixtures.mjs';
const root=new URL('../../',import.meta.url),revision='d322fcd4da01c243d5f76fe631d78db2d850106b';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});

export function applyNativeConsumerChanges(source,changes) {
  for(const change of changes){
    assert.equal(source.split(change.before).length-1,change.occurrences,'complete recorded native consumer template input');
    source=source.replaceAll(change.before,change.after);
  }
  return source;
}

export function generateConcreteProgramNativeTemplate({check=false}={}) {
  const sources=['rust/tests/unit/translation_typescript_typed_graph.rs','experiments/typescript-typed-graph/run-rust-emissions.mjs'].map(file=>{
    const original=captureGitArtifactCertificate(revision,file);
    verifyGitArtifactCertificate(original.certificate,original.blob,{revision,path:file});
    const sourceText=original.blob.toString('utf8');assert.deepEqual(Buffer.from(sourceText),original.blob);
    return{file,commit:revision,...pin(original.blob),sourceText,certificate:original.certificate};
  });
  const rustChanges=[
    ['//! CI-only consumers of the exact JavaScript-produced TypeScript program graphs.',
      '//! CI-only consumers of the CST-preserving priority-language program graphs.',1],
    ['fn check_type_script_graph(', 'fn check_concrete_program_graph(',1],
    ['../../../parity/fixtures/typescript-typed-graph/manifest.lino',
      '../../../parity/fixtures/concrete-program-graph-native/manifest.lino',1],
    ['assert_eq!(cases.len(), 4);','assert_eq!(cases.len(), 5);',1],
    ['issue-195-artifacts/typescript-typed-graph-native',
      'issue-195-artifacts/typescript-typed-graph-native/concrete-program-graphs',1],
    ['experiments/typescript-typed-graph/run-rust-emissions.mjs',
      'experiments/concrete-program-graphs/run-rust-emissions.mjs',1],
    ['native TypeScript graph execution:','native concrete program graph execution:',1],
    ['    let artifacts = repository.join("issue-195-artifacts/typescript-typed-graph-native/concrete-program-graphs");',
      '    let artifacts = repository\n        .join("issue-195-artifacts/typescript-typed-graph-native/concrete-program-graphs");',1],
    ["    let digest = format!(\"{:x}\", Sha256::digest(source.as_bytes()));","    let digest =\n        Sha256::digest(source.as_bytes())\n            .iter()\n            .fold(String::new(), |mut hexadecimal, byte| {\n                write!(hexadecimal, \"{byte:02x}\").expect(\"writing into a String succeeds\");\n                hexadecimal\n            });",1],
    ["        let digest = format!(\"{:x}\", Sha256::digest(code.as_bytes()));","        let digest =\n            Sha256::digest(code.as_bytes())\n                .iter()\n                .fold(String::new(), |mut hexadecimal, byte| {\n                    write!(hexadecimal, \"{byte:02x}\").expect(\"writing into a String succeeds\");\n                    hexadecimal\n                });",1],
  ].map(([before,after,occurrences])=>({before,after,occurrences}));
  const javascriptChanges=[
    ['../../parity/fixtures/typescript-typed-graph/manifest.lino',
      '../../parity/fixtures/concrete-program-graph-native/manifest.lino',1],
    ["'typescript-typed-graph-native'","'typescript-typed-graph-native/concrete-program-graphs'",1],
    ['runRustRuntimeTypeScriptGraphCase','runRustRuntimeConcreteProgramGraphCase',2],
    ['native TypeScript graph execution is GitHub CI-only','native concrete program graph execution is GitHub CI-only',1],
    ['native package TypeScript graph execution failed','native package concrete program graph execution failed',1],
  ].map(([before,after,occurrences])=>({before,after,occurrences}));
  const value={schemaVersion:1,revision,sources,rustChanges,javascriptChanges,
    boundary:'Complete certified native test and host process transport templates with explicit mechanical edits. Existing real Rust-package graph decoders and both emitters supply semantics. Generated files are source inputs, not native execution evidence.'};
  const file=new URL('parity/self-translation/concrete-program-native-consumer-template.lino',root);
  if(check)checkProjectData(file,value);else writeProjectData(file,value);
  return value;
}

export async function generateConcreteProgramNativeInputs({check=false}={}) {
  const {fixture}=generateConcreteProgramGraphFixtures({check:true}),template=generateConcreteProgramNativeTemplate({check});
  const [{createConcreteProgramLinks},{TYPED_PROGRAM_LINKS_SCHEMA}]=await Promise.all([
    import('../../experiments/concrete-program-graphs/projection.mjs'),import('../src/typed-program-links.js'),
  ]);
  const cases=[...fixture.cases.map(row=>({...row,edit:false})),...fixture.cases.filter(row=>row.language!=='TypeScript')
    .map(row=>({...row,edit:true,expectedLines:['6']}))];assert.equal(cases.length,5);
  const directory='parity/fixtures/concrete-program-graph-native';
  if(!check)mkdirSync(new URL(directory+'/',root),{recursive:true});
  const rows=[];
  for(const [index,row]of cases.entries()){
    const packet=createConcreteProgramLinks(row.source,row.language,{utf16Strings:true});
    if(row.edit){
      const operation=packet.network.fieldRelations().find(field=>field.label===TYPED_PROGRAM_LINKS_SCHEMA.fields.op
        &&packet.network.link(field.child).metadata().definition==='add');assert.ok(operation);
      const link=packet.network.link(operation.child);link.setMetadata(link.metadata().withDefinition('mul'));
    }
    const source=packet.network.toLino(),path=directory+'/case-'+index+'.lino';
    if(check)assert.equal(readFileSync(new URL(path,root),'utf8'),source);else writeFileSync(new URL(path,root),source);
    rows.push({index,language:row.language,variant:row.edit?'edited checked program':'original checked program',
      root:String(packet.root.asU64()),path,...pin(source),expectedLines:row.expectedLines});
  }
  const rustTemplate=template.sources.find(row=>row.file.endsWith('.rs')).sourceText;
  const marker='\n#[test]\n';assert.equal(rustTemplate.split(marker).length,5);
  const helper=rustTemplate.slice(0,rustTemplate.indexOf(marker));
  let rust=applyNativeConsumerChanges(helper,template.rustChanges)+'\n';
  const names=['original_java_script','original_rust','original_type_script','edited_java_script','edited_rust'];
  for(const[index,name]of names.entries())rust+='#[test]\nfn '+name+'_concrete_graph_executes_in_both_targets() {\n    check_concrete_program_graph('+index+');\n}\n\n';
  rust=rust.slice(0,-1);
  const javascript=applyNativeConsumerChanges(template.sources.find(row=>row.file.endsWith('.mjs')).sourceText,template.javascriptChanges);
  const outputs=[['rust/tests/unit/translation_concrete_program_graphs.rs',rust],
    ['experiments/concrete-program-graphs/run-rust-emissions.mjs',javascript]];
  for(const[path,source]of outputs){
    if(check)assert.equal(readFileSync(new URL(path,root),'utf8'),source);else writeFileSync(new URL(path,root),source);
  }
  const manifest={schemaVersion:1,cases:rows,template:'parity/self-translation/concrete-program-native-consumer-template.lino',
    generatedConsumers:outputs.map(([path,source])=>({path,...pin(source)})),
    boundary:'Five actual CST-preserving JavaScript/Rust/TypeScript program graphs. Original and edited expected lines are independent fixture inputs. Native execution remains CI-only and must be observed after real package decoding and emission.'};
  const path=new URL(directory+'/manifest.lino',root);
  if(check)checkProjectData(path,manifest);else writeProjectData(path,manifest);
  return manifest;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'));
  await generateConcreteProgramNativeInputs({check:process.argv.includes('--check')});
  console.log('Prepared five native graph inputs and mechanically generated both actual native consumer sources.');
}
