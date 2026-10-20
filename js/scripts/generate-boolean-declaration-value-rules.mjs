// Preserve and translate the complete direct-entry policy. No native Boolean
// operation, selector or source-qualification decision is authored here.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {checkProjectData,writeProjectData} from './project-data.mjs';

const root=new URL('../../',import.meta.url);
const sourcePath='js/src/boolean-declaration-value-rules.js';
const digest=value=>createHash('sha256').update(value).digest('hex');

/** Translate every original policy owner and retain both checked archive replays. */
export async function generateBooleanDeclarationValueRules({check=false,native=false}={}){
  const source=readFileSync(new URL(sourcePath,root),'utf8');
  assert.doesNotMatch(source,/^\s*import\b/mu);
  const decisions=[...source.matchAll(/^export function (\w+)\(/gmu)].map(match=>match[1]);
  assert.deepEqual(decisions,['acceptBooleanDeclarationValueContext','acceptBooleanDeclarationValueEntry','acceptBooleanDeclarationValueSelector']);
  const sourceClosure={source,bytes:Buffer.byteLength(source),sourceSha256:digest(source),
    inputs:[{path:sourcePath,source,bytes:Buffer.byteLength(source),sourceSha256:digest(source),removedImports:[]}]};
  const policy={schemaVersion:1,model:'closed Boolean declaration value selection 1',sourcePath,sourceClosure,decisions,
    basis:'closed boolean function declaration value',selector:'selected function declaration',
    ownership:'Exactly one formal-root selector targets an actual original flat nongenerated function declaration in the unchanged complete checked typed-program directory.',
    context:'Ordinary checked JavaScript; Rust only after the unchanged complete original const proof succeeds. The same existing closed Boolean operation walk validates every declaration.',
    qualification:'Only authenticated original owned source-signature and Rust const-qualification fields are excluded from this runtime callable observation. They and their original declaration IDs remain in the complete archive.',
    boundary:'No generated const wrapper, generic const-call relaxation, typed-program schema extension or fabricated source proof.'};
  for(const path of ['js/src/data/boolean-declaration-value-rules.lino','rust/src/data/boolean-declaration-value-rules.lino']){
    if(check)checkProjectData(new URL(path,root),policy);else writeProjectData(new URL(path,root),policy);
  }
  let artifact=null;
  if(native){
    const {parseJavaScript}=await import('../src/translation/javascript.js');
    const {checkProgram}=await import('../src/translation/check.js');
    const {emitRust}=await import('../src/translation/emit-rust.js');
    const {writeRecordRuleLibrary}=await import('./record-rule-library.mjs');
    const {encodeTypedProgramLinks,decodeTypedProgramLinks}=await import('../src/typed-program-links.js');
    const {recheckLinkedTypedProgram}=await import('../src/typed-program-checked-projection.js');
    const {LinkNetwork}=await import('../src/network.js');
    const {encodeLinkNetworkBinary,decodeLinkNetworkBinary}=await import('../src/link-network-binary.js');
    const surface=parseJavaScript(source,{referenceSemantics:false});
    const inventory=surface.items.map((item,sourceOrdinal)=>({sourceOrdinal,kind:item.k,name:item.name,generated:item.generated===true}));
    assert.deepEqual(inventory.filter(item=>!item.generated).map(item=>item.name),decisions);
    const checked=checkProgram(surface);
    const owners=program=>inventory.map(owner=>{
      const declaration=program.declarations.get(owner.name);assert.ok(declaration&&declaration.k===owner.kind);
      return {...owner,fullName:declaration.fullName};
    });
    const checkedNamedSourceInventory=owners(checked);
    assert.equal(checkedNamedSourceInventory.length,checked.declarations.size);
    const output=emitRust(checked,{arrayConsumption:true});
    const emitted={text:output.text,definitions:output.definitions,preludes:output.preludes,mappings:output.mappings,
      assumptions:output.assumptions,encodings:output.encodings};
    for(const owner of checkedNamedSourceInventory)assert.equal(emitted.mappings.filter(mapping=>mapping.source===owner.fullName&&mapping.kind==='function').length,1);
    const packet=encodeTypedProgramLinks(checked),replays=[];
    for(const [format,network]of [['text',LinkNetwork.fromLino(packet.network.toLino())],['binary',decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]]){
      const restored=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});
      assert.deepEqual(owners(restored),checkedNamedSourceInventory);
      const replay=emitRust(restored,{arrayConsumption:true});
      for(const field of Object.keys(emitted))assert.deepEqual(replay[field],emitted[field]);
      replays.push({format,checkedNamedSourceInventory:owners(restored),emittedTextSha256:digest(replay.text)});
    }
    const files=writeRecordRuleLibrary(emitted,'boolean_declaration_value_rules',{check});
    artifact={schemaVersion:1,model:policy.model,sourceClosure,inventory,checkedNamedSourceInventory,emitted,replays,
      files:files.map(file=>({path:file.path??'boolean_declaration_value_rules.rs',bytes:Buffer.byteLength(file.source),sourceSha256:digest(file.source)})),
      boundary:'Complete source-only checked translation with both honest source-free archive replays; native compilation and runtime checks remain CI-only.'};
    const file=new URL('parity/self-translation/boolean-declaration-value-rules-native.lino',root);
    if(check)checkProjectData(file,artifact);else writeProjectData(file,artifact);
  }
  return {policy,artifact};
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2);assert.ok(new Set(args).size===args.length&&args.every(arg=>['--check','--native'].includes(arg)));
  const result=await generateBooleanDeclarationValueRules({check:args.includes('--check'),native:args.includes('--native')});
  console.log(JSON.stringify({decisions:result.policy.decisions,checkedOwners:result.artifact?.checkedNamedSourceInventory.length??null,nativeCompilation:false,nativeExecution:false}));
}
