// Normal writer: complete real shared decisions, both semantic codecs and native ABI.
import assert from 'node:assert/strict';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import {checkProjectData,readProjectData,writeProjectData} from './project-data.mjs';
import {writeRecordRuleLibrary} from './record-rule-library.mjs';
import {tokenize} from '../src/translation/lexer.js';
const root=new URL('../../',import.meta.url),sourcePath='js/src/translation/rust-shared-array-handle-rules.js';
const digest=source=>createHash('sha256').update(source).digest('hex');
const imports=["import {acceptRustSourceArrayElement} from './rust-array-rules.js';\n","import {acceptReferenceIdentityComparison} from './reference-identity-rules.js';\n"];
const decisions=['acceptRustSharedArrayHandleSourceModel','acceptRustSharedArrayHandleSourceSize','acceptRustSharedArrayHandleLayer','acceptRustSharedArrayHandlePayload','acceptRustSharedArrayHandleTransfer','readRustSharedArrayHandleSourceForm','acceptRustSharedArrayHandleValueObservation','readRustSharedArrayHandleComparisonProjection'];
export function rustSharedArrayHandleRulesPolicy(){
  const paths=['js/src/translation/rust-array-rules.js','js/src/translation/reference-identity-rules.js',sourcePath];
  const inputs=paths.map(path=>{const source=readFileSync(new URL(path,root),'utf8');return{path,source,bytes:Buffer.byteLength(source),sha256:digest(source)};});
  let own=inputs.at(-1).source;
  for(const edge of imports){assert.equal(own.split(edge).length-1,1,'one complete actual required module edge');own=own.replace(edge,'');}
  assert.ok(inputs.slice(0,-1).every(input=>!/^\s*import\b/mu.test(input.source)));assert.doesNotMatch(own,/^\s*import\b/mu);
  assert.deepEqual([...inputs.at(-1).source.matchAll(/^export function (\w+)\(/gmu)].map(match=>match[1]),decisions);
  const source=inputs.slice(0,-1).map(input=>input.source).concat([own]).join('\n');
  return{schemaVersion:1,model:'standard Rust shared array identity handles 1',sourcePath,decisions,
    sourceClosure:{source,bytes:Buffer.byteLength(source),sha256:digest(source),inputs,removedImports:imports},
    assertionVersions:{Node:'>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0'},
    retainedIdentity:{file:'js/tests/translation-module-reference.test.js',owner:'identity',source:readFileSync(new URL('js/tests/translation-module-reference.test.js',root),'utf8')},
    cases:[
      {name:'one fresh handle and its explicit alias',source:'function probe(){const value=[true];const alias=value;return value===alias;}',call:'probe',expected:true},
      {name:'separate equal Boolean payloads keep distinct identities',source:'function probe(){const left=[true];const right=[true];return left===right;}',call:'probe',expected:false},
      {name:'fresh calls are distinct allocations',source:'/** @returns {boolean[]} */function fresh(){return [true];}function probe(){return fresh()===fresh();}',call:'probe',expected:false},
      ...[
        {domain:'numeric',annotation:'number',literal:'[1.25,-0]'},
        {domain:'Unicode string',annotation:'string',literal:'["hello","🌍"]'},
      ].flatMap(({domain,annotation,literal})=>[
        {name:domain+' payload retains an explicit shared alias',source:'function probe(){const value='+literal+';const alias=value;return value===alias;}',call:'probe',expected:true},
        {name:domain+' equal payloads remain distinct allocations',source:'function probe(){const left='+literal+';const right='+literal+';return left===right;}',call:'probe',expected:false},
        {name:domain+' fresh calls retain distinct allocations',source:'/** @returns {'+annotation+'[]} */function fresh(){return '+literal+';}function probe(){return fresh()===fresh();}',call:'probe',expected:false},
      ]),
    ],
    boundary:'Closed flat Rust source modules/programs with actual fully qualified standard Rc and RefCell owners and direct scalar Vec payloads. Fresh nonempty literal allocation, explicit handle clone and actual ptr_eq borrow are represented by existing checked nodes. Ordinary Rc equality, payload methods and observations, bare value Vec, escaping borrows/moves and alternate source interpretations refuse. Complete portable executable runner composition remains a separate obligation. Native parser and module adapters require actual CI compilation and execution.'};
}
export async function generateRustSharedArrayHandleRules({check=false,native=false}={}){
  const policy=rustSharedArrayHandleRulesPolicy();
  for(const path of ['js/src/translation/data/rust-shared-array-handles.lino','rust/src/data/rust-shared-array-handles.lino']){
    if(check)checkProjectData(new URL(path,root),policy);else writeProjectData(new URL(path,root),policy);
  }
  let artifact=null;
  if(native){
    const {parseJavaScript}=await import('../src/translation/javascript.js');
    const {checkProgram}=await import('../src/translation/check.js');
    const {emitRust}=await import('../src/translation/emit-rust.js');
    const {encodeTypedProgramLinks,decodeTypedProgramLinks}=await import('../src/typed-program-links.js');
    const {recheckLinkedTypedProgram}=await import('../src/typed-program-checked-projection.js');
    const {LinkNetwork}=await import('../src/network.js');
    const {encodeLinkNetworkBinary,decodeLinkNetworkBinary}=await import('../src/link-network-binary.js');
    const surface=parseJavaScript(policy.sourceClosure.source,{referenceSemantics:false});
    const inventory=surface.items.map((item,ordinal)=>({ordinal,name:item.name,kind:item.k,generated:item.generated===true}));
    const sourceNames=[...policy.sourceClosure.source.matchAll(/^export function (\w+)\(/gmu)].map(row=>row[1]);
    assert.deepEqual(inventory.filter(row=>!row.generated).map(row=>row.name),sourceNames,'every dependency and current decision is retained');
    const checked=checkProgram(surface),emitted=emitRust(checked,{arrayConsumption:true}),packet=encodeTypedProgramLinks(checked);
    const owners=program=>inventory.map(row=>{const declaration=program.declarations.get(row.name);assert.ok(declaration&&declaration.k===row.kind);return{...row,fullName:declaration.fullName};});
    const checkedOwners=owners(checked);assert.equal(checkedOwners.length,checked.declarations.size);
    const text=packet.network.toLino(),programPaths=['js/src/data/rust-shared-array-handle-program.lino','rust/src/data/rust-shared-array-handle-program.lino'];
    for(const path of programPaths){if(check)assert.equal(readFileSync(new URL(path,root),'utf8'),text);else writeFileSync(new URL(path,root),text);}
    const replays=[];
    for(const [format,network]of [['text',LinkNetwork.fromLino(text)],['binary',decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]]){
      const program=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});assert.deepEqual(owners(program),checkedOwners);
      const replay=emitRust(program,{arrayConsumption:true});for(const field of ['text','definitions','preludes','mappings','assumptions','encodings'])assert.deepEqual(replay[field],emitted[field]);
      replays.push({format,owners:owners(program),sha256:digest(replay.text)});
    }
    for(const owner of checkedOwners)assert.equal(emitted.mappings.filter(row=>row.source===owner.fullName&&row.kind==='function').length,1);
    const files=writeRecordRuleLibrary(emitted,'rust_shared_array_handle_rules',{check}),libraryInterface=[];
    for(const file of files){const tokens=tokenize(file.source,'Rust').tokens;
      for(let at=0;at<tokens.length;at+=1){if(tokens[at].value!=='fn'||tokens[at+1]?.kind!=='identifier')continue;
        const target=tokens[at+1].value,mapping=emitted.mappings.find(row=>row.target===target&&row.kind==='function');if(!mapping)continue;
        let body=at+2;while(body<tokens.length&&tokens[body].value!=='{')body+=1;assert.ok(body<tokens.length);
        libraryInterface.push({source:mapping.source,target,path:file.path??'rust_shared_array_handle_rules.rs',signature:file.source.slice(tokens[at].start,tokens[body].start).trim()});
      }}
    for(const name of policy.decisions)assert.equal(libraryInterface.filter(row=>row.source===name).length,1);
    artifact={schemaVersion:1,model:policy.model,sourceClosure:policy.sourceClosure,inventory,checkedOwners,replays,libraryInterface,
      archive:{root:packet.root.asU64(),paths:programPaths,bytes:Buffer.byteLength(text),sha256:digest(text)},
      emitted:{text:emitted.text,definitions:emitted.definitions,preludes:emitted.preludes,mappings:emitted.mappings,assumptions:emitted.assumptions,encodings:emitted.encodings},
      files:files.map(file=>({path:file.path??'rust_shared_array_handle_rules.rs',source:file.source,bytes:Buffer.byteLength(file.source),sha256:digest(file.source)})),
      boundary:'Complete normal source translation and actual generated library only. No native host or executed native evidence is supplied by this artifact.'};
    const path=new URL('parity/self-translation/rust-shared-array-handle-rules-native.lino',root);if(check)checkProjectData(path,artifact);else writeProjectData(path,artifact);
  }
  const featurePath=new URL('parity/language-features.lino',root),catalog=readProjectData(featurePath);
  const feature={id:'rust-standard-shared-array-handle-source-identity',required:true,
    javascript:{status:'implemented',evidence:[sourcePath,'js/src/translation/rust-shared-array-handles.js','js/src/translation/rust-shared-array-handle-modules.js','js/src/translation/rust.js','js/src/translation/check.js','js/src/self-translation.js','js/src/typed-program-links.js','js/scripts/generate-rust-shared-array-handle-rules.mjs','js/src/translation/data/rust-shared-array-handles.lino','js/src/data/rust-shared-array-handle-program.lino','js/tests/translation-rust-shared-array-handles.test.js','js/tests/translation-rust-shared-array-handle-modules.test.js']},
    rust:{status:'implemented',evidence:['rust/src/translation/rust/shared_array_handles.rs','rust/src/self_translation/shared_array_handles.rs','rust/src/translation/check/expressions.rs','rust/src/typed_program_links/projection.rs','rust/src/translation/rust_shared_array_handle_rules.rs','rust/src/translation/rust_shared_array_handle_rules/part_1.rs','rust/src/data/rust-shared-array-handles.lino','rust/src/data/rust-shared-array-handle-program.lino','rust/tests/unit/translation_rust_shared_array_handles.rs','parity/self-translation/rust-shared-array-handle-rules-native.lino']}};
  const rows=catalog.features.filter(row=>row.id===feature.id);assert.ok(rows.length<=1);
  if(check)assert.deepEqual(rows,[feature]);
  else if(rows.length)Object.assign(rows[0],feature);else catalog.features.push(feature);
  if(!check)writeProjectData(featurePath,catalog);
  return{policy,artifact};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  assert.ok(process.argv.slice(2).every(value=>value==='--check'||value==='--native'));
  await generateRustSharedArrayHandleRules({check:process.argv.includes('--check'),native:process.argv.includes('--native')});
}
