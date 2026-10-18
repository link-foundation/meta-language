// Translate the complete ownership planner; hosts interpret its selected IDs.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {checkProjectData,writeProjectData} from './project-data.mjs';
import {writeRecordRuleLibrary} from './record-rule-library.mjs';
import {readGuardedPrivateArrayWriteLibraryStyles} from './guarded-private-array-write-library-style.mjs';

const root=new URL('../../',import.meta.url);
const sourcePath='js/src/translation/guarded-private-array-write-rules.js';
const hash=source=>createHash('sha256').update(source).digest('hex');

export function guardedPrivateArrayWriteRulesPolicy(){
  const source=readFileSync(new URL(sourcePath,root),'utf8');
  assert.doesNotMatch(source,/^\s*import\b/mu,'the whole planner is a closed source module');
  return {schemaVersion:1,model:'guarded private primitive array write 1',sourcePath,source,
    sourceBytes:Buffer.byteLength(source),sourceSha256:hash(source),
    selection:'Flat triples identify the actual conditional, indexed write and fresh allocation. JavaScript statement and native expression wrappers retain their actual identities.',
    admission:'A nonempty dense primitive literal has no alias or escaping incoming ownership edge. A direct single-write conditional retains integer, lower and upper guards; pure operands preserve source evaluation order. Unsupported facts retain the original write.',
    materialization:'The host saves the actual index and right operand once, then rebinds the private array through the existing immutable map operation. The original guard remains in place.',
    archivePath:'src/data/guarded-private-array-write-program.lino',
    boundary:'The original nonempty-literal profile excludes aliases, captures, blocks, loops, unknown calls and empty literals. The joint construction profile below is separate. Neither makes a linear-time or general indexed-mutation claim. Native behavior requires CI execution.',
    construction:{selection:'Flat four-field rows identify operation code, actual statement, actual operation and fresh empty allocation. Every discarded primitive-literal append and direct guarded write is jointly proved before either is lowered.',
      admission:'The original guard proof applies to direct top-level writes. A fresh empty private array has at least one discarded primitive-literal append, including in builder loops. Every incoming owner use is checked; aliases, captures, rebinding, observed push results, unknown arguments and nested writes refuse.',
      materialization:'Actual append arguments become immutable concatenation; guarded writes use the original immutable map rebinding. Original source order and both original ownership planners remain.',
      boundary:'This conservative extension requires explicit integer and array bounds guards. It does not admit general mutable builders or claim linear complexity.'}};
}

export async function generateGuardedPrivateArrayWriteRules({check=false,native=false}={}){
  const policy=guardedPrivateArrayWriteRulesPolicy();
  for(const path of ['js/src/translation/data/guarded-private-array-write-rules.lino','rust/src/data/guarded-private-array-write-rules.lino']){
    if(check)checkProjectData(new URL(path,root),policy);else writeProjectData(new URL(path,root),policy);
  }
  let artifact=null;
  if(native){
    const {parseJavaScript}=await import('../src/translation/javascript.js');
    const {checkProgram}=await import('../src/translation/check.js');
    const {emitRust}=await import('../src/translation/emit-rust.js');
    const {emitJavaScript}=await import('../src/translation/emit-javascript.js');
    const {encodeTypedProgramLinks,decodeTypedProgramLinks}=await import('../src/typed-program-links.js');
    const {recheckLinkedTypedProgram}=await import('../src/typed-program-checked-projection.js');
    const {LinkNetwork}=await import('../src/network.js');
    const {encodeLinkNetworkBinary,decodeLinkNetworkBinary}=await import('../src/link-network-binary.js');
    const surface=parseJavaScript(policy.source,{referenceSemantics:false}),checked=checkProgram(surface);
    const inventory=surface.items.map((item,ordinal)=>({ordinal,kind:item.k,name:item.name,generated:item.generated===true}));
    const owners=program=>inventory.map(item=>{
      const declaration=program.declarations.get(item.name);
      assert.ok(declaration&&declaration.k===item.kind,'every original and generated declaration remains');
      return {...item,fullName:declaration.fullName,...(declaration.modulePath===undefined?{}:{modulePath:declaration.modulePath})};
    });
    const checkedNamedSourceInventory=owners(checked);
    assert.equal(inventory.length,checked.declarations.size);
    const emitted=emitRust(checked,{arrayConsumption:true}),javascript=emitJavaScript(checked);
    const packet=encodeTypedProgramLinks(checked),text=packet.network.toLino(),binary=encodeLinkNetworkBinary(packet.network),replays=[];
    for(const [format,network]of [['text',LinkNetwork.fromLino(text)],['binary',decodeLinkNetworkBinary(binary)]]){
      const restored=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});
      assert.deepEqual(owners(restored),checkedNamedSourceInventory);
      const replay=emitRust(restored,{arrayConsumption:true});
      for(const field of ['text','definitions','preludes','mappings','assumptions','encodings'])assert.deepEqual(replay[field],emitted[field]);
      assert.deepEqual(emitJavaScript(restored),javascript,'whole JavaScript emission and provenance survive source-free reload');
      replays.push({format,checkedNamedSourceInventory:owners(restored),emittedTextBytes:Buffer.byteLength(replay.text),emittedTextSha256:hash(replay.text)});
    }
    for(const owner of checkedNamedSourceInventory)assert.equal(emitted.mappings.filter(mapping=>mapping.source===owner.fullName&&mapping.kind===(owner.kind==='fn'?'function':owner.kind)).length,1);
    for(const path of ['js/src/data/guarded-private-array-write-program.lino','rust/src/data/guarded-private-array-write-program.lino']){
      const output=new URL(path,root);
      if(check)assert.equal(readFileSync(output,'utf8'),text,'the actual inspectable linked program is fresh');else writeFileSync(output,text);
    }
    const files=writeRecordRuleLibrary(emitted,'guarded_private_array_write_rules',{check,sourceStyles:readGuardedPrivateArrayWriteLibraryStyles});
    artifact={schemaVersion:1,model:policy.model,source:{path:sourcePath,text:policy.source,bytes:policy.sourceBytes,sha256:policy.sourceSha256},
      inventory,checkedNamedSourceInventory,replays,
      emitted:{text:emitted.text,definitions:emitted.definitions,preludes:emitted.preludes,mappings:emitted.mappings,assumptions:emitted.assumptions,encodings:emitted.encodings},
      emittedTextBytes:Buffer.byteLength(emitted.text),emittedTextSha256:hash(emitted.text),
      archive:{root:packet.root.asU64(),textBytes:Buffer.byteLength(text),textSha256:hash(text),binaryBytes:binary.byteLength,binarySha256:hash(binary)},
      library:{files:files.map(file=>({path:file.path??'guarded_private_array_write_rules.rs',bytes:Buffer.byteLength(file.source),sourceSha256:hash(file.source)}))},boundary:policy.boundary};
    const output=new URL('parity/self-translation/guarded-private-array-write-rules-native.lino',root);
    if(check)checkProjectData(output,artifact);else writeProjectData(output,artifact);
  }
  return {policy,artifact};
}

if(process.argv[1]===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2);
  assert.ok(new Set(args).size===args.length&&args.every(argument=>['--check','--native'].includes(argument)));
  await generateGuardedPrivateArrayWriteRules({check:args.includes('--check'),native:args.includes('--native')});
}
