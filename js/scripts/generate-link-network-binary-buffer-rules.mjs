// Complete source-first successor. Historical framing sources and styles stay
// unchanged; this producer packages every checked owner and both typed archives.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readProjectData,writeProjectData,checkProjectData} from './project-data.mjs';
import {writeRecordRuleLibrary} from './record-rule-library.mjs';
import {readBinaryBufferCompilerStyles} from './binary-buffer-compiler-styles.mjs';

const root=new URL('../../',import.meta.url);
const digest=bytes=>createHash('sha256').update(bytes).digest('hex');
const originalFile='js/src/link-network-binary-record-rules.js';
const sourceFile='js/src/link-network-binary-buffer-rules.js';
const primitiveFile='js/src/link-network-binary-rules.js';
const artifactFile=new URL('parity/self-translation/link-network-binary-buffer-rules-native.lino',root);
const imported="import { linkNetworkBinaryUnit, acceptLinkNetworkBinaryInteger, readLinkNetworkBinaryInteger,\n  linkNetworkBinaryIntegerByte, renderLinkNetworkBinaryField } from './link-network-binary-rules.js';\n";

export function readLinkNetworkBinaryBufferSourceModule() {
  const primitive=readFileSync(new URL(primitiveFile,root),'utf8');
  const source=readFileSync(new URL(sourceFile,root),'utf8');
  const boundary=primitive.indexOf('/** Validate and transcode');
  assert.ok(boundary>0,'the complete existing primitive declarations precede their historical framing entry');
  assert.equal(source.split(imported).length,2,'one actual primitive module edge is resolved');
  return primitive.slice(0,boundary)+source.replace(imported,'');
}

export async function generateLinkNetworkBinaryBufferRules({check=false}={}) {
  const [{parseJavaScript},{checkProgram},{emitRust},{encodeTypedProgramLinks,decodeTypedProgramLinks},
    {recheckLinkedTypedProgram},{LinkNetwork},{encodeLinkNetworkBinary,decodeLinkNetworkBinary}]=await Promise.all([
    import('../src/translation/javascript.js'),import('../src/translation/check.js'),import('../src/translation/emit-rust.js'),
    import('../src/typed-program-links.js'),import('../src/typed-program-checked-projection.js'),
    import('../src/network.js'),import('../src/link-network-binary.js'),
  ]);
  const source=readLinkNetworkBinaryBufferSourceModule();
  const program=checkProgram(parseJavaScript(source,{referenceSemantics:false}));
  const inventory=program.items.map((row,ordinal)=>({ordinal,kind:row.k,name:row.name,fullName:row.fullName,generated:row.generated===true}));
  const originalNames=['linkNetworkBinaryUnit','acceptLinkNetworkBinaryInteger','readLinkNetworkBinaryInteger',
    'linkNetworkBinaryIntegerByte','acceptLinkNetworkBinaryUnicode','renderLinkNetworkBinaryHexadecimalDigit',
    'renderLinkNetworkBinaryField','transcodeLinkNetworkBinary'];
  assert.deepEqual(inventory.filter(row=>!row.generated).map(row=>row.name),originalNames);
  assert.equal(new Set(inventory.map(row=>row.fullName)).size,program.declarations.size);
  const options={arrayConsumption:true};
  const emitted=emitRust(program,options);
  assert.equal(emitted.definitions.length,inventory.length);
  for(const owner of inventory) assert.equal(emitted.mappings.filter(row=>row.source===owner.fullName
    &&row.kind===(owner.kind==='fn'?'function':owner.kind)).length,1,'one complete mapping: '+owner.fullName);
  const packet=encodeTypedProgramLinks(program),recordCount=packet.network.links().length;
  assert.ok(recordCount>0&&recordCount<=1_000_000,'the complete source archive has an explicit bounded record census');
  const binaryLimits={recordLimit:recordCount,byteLimit:64*1024*1024};
  const text=packet.network.toLino(),binary=encodeLinkNetworkBinary(packet.network,binaryLimits);
  assert.ok(Buffer.byteLength(text)<=binaryLimits.byteLimit);
  const replays=[];
  for(const format of ['text','binary']) {
    const network=format==='text'?LinkNetwork.fromLino(text):decodeLinkNetworkBinary(binary,binaryLimits);
    assert.equal(network.links().length,recordCount);
    const restored=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});
    assert.deepEqual(restored.items.map(row=>row.fullName),inventory.map(row=>row.fullName));
    const replay=emitRust(restored,options);
    for(const field of ['text','definitions','preludes','mappings','assumptions','encodings'])assert.deepEqual(replay[field],emitted[field],
      'the entire source-free emitted '+field+' agrees after '+format);
    replays.push({format,owners:restored.declarations.size,emittedBytes:Buffer.byteLength(replay.text),emittedSha256:digest(replay.text)});
  }
  const files=writeRecordRuleLibrary(emitted,'link_network_binary_buffer_rules',{check,sourceStyles:readBinaryBufferCompilerStyles});
  const artifact={schemaVersion:1,model:'owned binary framing field buffers 1',
    inputs:[sourceFile,primitiveFile,originalFile].map(path=>{const bytes=readFileSync(new URL(path,root));return{path,bytes:bytes.length,sha256:digest(bytes)};}),
    sourceClosure:{source,bytes:Buffer.byteLength(source),sha256:digest(source),removedImport:imported},
    options,inventory,definitionCount:inventory.length,emitted:{text:emitted.text,bytes:Buffer.byteLength(emitted.text),sha256:digest(emitted.text),
      mappings:emitted.mappings,assumptions:emitted.assumptions,encodings:emitted.encodings},
    library:files.map(row=>({path:row.path??'link_network_binary_buffer_rules.rs',bytes:Buffer.byteLength(row.source),sha256:digest(row.source)})),
    archive:{recordCount,binaryLimits,textBytes:Buffer.byteLength(text),textSha256:digest(text),binaryBytes:binary.byteLength,binarySha256:digest(binary)},
    replays,nativeCompilation:false,nativeExecution:false,
    boundary:'All complete primitive and framing owners, actual final-use array moves and both complete checked-program archives. Native runtime parity and full-grammar timing remain CI evidence.'};
  if(check)checkProjectData(artifactFile,artifact);else writeProjectData(artifactFile,artifact);
  assert.deepEqual(readProjectData(artifactFile),artifact);
  return artifact;
}

if(process.argv[1]===fileURLToPath(import.meta.url)) {
  assert.ok(process.argv.slice(2).every(argument=>argument==='--check'),'only read-only --check is also admitted');
  const artifact=await generateLinkNetworkBinaryBufferRules({check:process.argv.includes('--check')});
  console.log('Binary field buffers: '+artifact.definitionCount+' complete checked owners; both typed archives agree; native execution not selected');
}
