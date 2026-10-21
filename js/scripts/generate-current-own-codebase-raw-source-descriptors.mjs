// Finite normal author: independently certify a8, then append one exact edge.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {checkProjectData,writeProjectData,serializeProjectData} from './project-data.mjs';
import {acceptCurrentRawPredecessorOutput,acceptCurrentRawPredecessorPacketSource} from './current-own-codebase-raw-source-history.mjs';
import {readCurrentOwnCodebaseRawDescriptorPolicy,deriveCurrentOwnCodebaseRawDescriptorSource,
  deriveCurrentOwnCodebaseRawDescriptorOutputs,verifyCurrentOwnCodebaseRawDescriptorRefresh} from './current-own-codebase-raw-source-descriptors.mjs';
const root=new URL('../../',import.meta.url);
const helper='js/scripts/current-own-codebase-raw-source-descriptors.mjs';
const pin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const once=(source,from,to)=>{assert.equal(source.split(from).length,2,'one exact original adapter anchor: '+from);return source.replace(from,()=>to);};

/** One automatically discovered UTF-8 interval, with complete whole-byte inverse. */
function intervals(previous,current){
  if(previous===current)return[];
  const before=Buffer.from(previous),after=Buffer.from(current);let start=0,suffix=0;
  while(start<Math.min(before.length,after.length)&&before[start]===after[start])start+=1;
  while(start>0&&((before[start]&0xc0)===0x80||(after[start]&0xc0)===0x80))start-=1;
  while(before.length-suffix>start&&after.length-suffix>start&&before[before.length-suffix-1]===after[after.length-suffix-1])suffix+=1;
  while(suffix>0&&((before[before.length-suffix]&0xc0)===0x80||(after[after.length-suffix]&0xc0)===0x80))suffix-=1;
  const end=before.length-suffix,currentEnd=after.length-suffix;
  return[{start,end,previousText:before.subarray(start,end).toString('utf8'),currentStart:start,currentEnd,currentText:after.subarray(start,currentEnd).toString('utf8')}];
}

function adapterSource(file,source){
  const prefix="\nconst root=new URL('../../',import.meta.url)";
  if(file.endsWith('/own-codebase-raw-source-descriptors.mjs')&&!file.includes('/generate-')){
    source=once(source,prefix,"\nimport {restoreCurrentOwnCodebaseRawSource,restoreCurrentOwnCodebaseRawText,restoreCurrentOwnCodebaseRawEdge} from './current-own-codebase-raw-source-descriptors.mjs';"+prefix);
    source=once(source,'export function restoreOwnCodebaseRawDescriptorText(source,file){return restoreText(source,file,loadPacket());}',
      'export function restoreOwnCodebaseRawDescriptorText(source,file){return restoreText(restoreCurrentOwnCodebaseRawText(source,file),file,loadPacket());}');
    source=once(source,'export function restoreOwnCodebaseRawDescriptorData(value,file){return readStructuredDataLino(restoreText(serializeProjectData(value,file),file,loadPacket()));}',
      'export function restoreOwnCodebaseRawDescriptorData(value,file){return readStructuredDataLino(restoreText(restoreCurrentOwnCodebaseRawText(serializeProjectData(value,file),file),file,loadPacket()));}');
    source=once(source,'export function restoreOwnCodebaseRawDescriptorEdge(artifact,identity,register){\n',
      'export function restoreOwnCodebaseRawDescriptorEdge(artifact,identity,register){\n  ({artifact,register}=restoreCurrentOwnCodebaseRawEdge(artifact,identity,register));\n');
    source=once(source,"pin(readFileSync(new URL(policy.sourceFile,root)))","pin(Buffer.from(restoreCurrentOwnCodebaseRawSource(readFileSync(new URL(policy.sourceFile,root),'utf8'),policy.sourceFile)))");
    return once(source,"'actual current frontend source remains independently pinned'","'the exact historical frontend view remains independently pinned; the successor checks live bytes'");
  }
  if(file.endsWith('/generate-own-codebase-raw-source-descriptors.mjs')){
    source=once(source,prefix,"\nimport {restoreCurrentOwnCodebaseRawSource,restoreCurrentOwnCodebaseRawText} from './current-own-codebase-raw-source-descriptors.mjs';"+prefix);
    source=once(source,"pin(readFileSync(new URL(policy.sourceFile,root)))","pin(Buffer.from(restoreCurrentOwnCodebaseRawSource(readFileSync(new URL(policy.sourceFile,root),'utf8'),policy.sourceFile)))");
    source=once(source,"'the actual reviewed frontend source, not an archived read view, is used'","'the original frontend assertion checks its exact historical view; live bytes belong to the successor'");
    return once(source,"for(const row of writes){const observed=readFileSync(new URL(row.file,root),'utf8');assert.ok(",
      "for(const row of writes){const physical=readFileSync(new URL(row.file,root),'utf8'),observed=check&&[policy.artifactFile,policy.registerFile,policy.loanFile].includes(row.file)?restoreCurrentOwnCodebaseRawText(physical,row.file):physical;assert.ok(");
  }
  assert.equal(file,'js/tests/translation-own-codebase-raw-source-descriptors.test.js');
  source=once(source,prefix,"\nimport {restoreCurrentOwnCodebaseRawSource,restoreCurrentOwnCodebaseRawData} from '../scripts/current-own-codebase-raw-source-descriptors.mjs';"+prefix);
  source=once(source,"assert.equal(source,readFileSync(new URL(policy.sourceFile,root),'utf8'));",
    "assert.equal(source,restoreCurrentOwnCodebaseRawSource(readFileSync(new URL(policy.sourceFile,root),'utf8'),policy.sourceFile));");
  return once(source,"const loan=readProjectData(new URL(packet.loan.file,root));assert.deepEqual(",
    "const loan=restoreCurrentOwnCodebaseRawData(readProjectData(new URL(packet.loan.file,root)),packet.loan.file);assert.deepEqual(");
}

export function generateCurrentOwnCodebaseRawSourceDescriptors({check=false}={}){
  const policy=readCurrentOwnCodebaseRawDescriptorPolicy();
  const capture=file=>{const{blob,certificate}=captureGitArtifactCertificate(policy.revision,file);assert.equal(Buffer.from(blob.toString('utf8')).equals(blob),true);return{file,commit:policy.revision,...pin(blob),sourceText:blob.toString('utf8'),certificate,current:null};};
  const[artifact,register,loan]=[policy.artifactFile,policy.registerFile,policy.loanFile].map(capture);
  const old=readStructuredDataLino(artifact.sourceText),proof=old.predecessor.proof.producerApplicability;
  const sources=policy.sourcePaths.map((file,index)=>{
    const row=capture(file),sourceText=readFileSync(new URL(file,root),'utf8');
    const result={...row,current:{...pin(sourceText),sourceText},artifactPrevious:structuredClone(old.decisionSources[policy.decisionIndices[index]]),edits:intervals(row.sourceText,sourceText)};
    assert.equal(result.artifactPrevious.file,file);assert.notEqual(result.current.sha256,result.artifactPrevious.sha256,'each selected actual descriptor needs this edge: '+file);
    deriveCurrentOwnCodebaseRawDescriptorSource(result);return result;
  });
  const adapters=policy.adapterPaths.map(file=>{const row=capture(file),sourceText=adapterSource(file,row.sourceText);return{...row,current:{...pin(sourceText),sourceText},edits:intervals(row.sourceText,sourceText)};});
  const inferenceDescriptorAlias=proof.sources.find(row=>row.file===policy.sourcePaths[1])===proof.sourceHistory.current;
  const packet={schemaVersion:policy.schemaVersion,model:policy.model,revision:policy.revision,artifact,register,loan,sources,adapters,inferenceDescriptorAlias,edge:null,
    boundary:'One additive provenance edge updates three actual source descriptors and both logical inference-proof descriptor paths. Complete emitted definitions, archives, fixtures, mappings, prior receipts and policies retain exact whole-byte inverses. This supplies no semantic-equivalence or native-execution evidence.'};
  const result=deriveCurrentOwnCodebaseRawDescriptorOutputs(packet);packet.edge=result.edge;
  for(const row of[artifact,register,loan])row.current=pin(result.outputs.find(output=>output.file===row.file).source);
  verifyCurrentOwnCodebaseRawDescriptorRefresh(packet);
  const writes=[...result.outputs.map(row=>({...row,previous:[artifact,register,loan].find(input=>input.file===row.file).sourceText})),
    ...adapters.map(row=>({file:row.file,source:deriveCurrentOwnCodebaseRawDescriptorSource(row),previous:row.sourceText}))];
  // Validate every input before the first write, and keep repeated authors exact.
  for(const row of writes){const actual=readFileSync(new URL(row.file,root),'utf8');assert.ok(actual===row.previous||actual===row.source||acceptCurrentRawPredecessorOutput(actual,row.file),'unreviewed whole input: '+row.file);if(check)assert.equal(actual,row.source);}
  const helperPath=new URL(helper,root),source=readFileSync(helperPath,'utf8'),pattern=/^const policySha256='(?:PENDING|[a-f0-9]{64})';$/mu;
  assert.equal(source.split(pattern).length,2);
  const packetSource=readFileSync(new URL(policy.output,root),'utf8');
  assert.ok(acceptCurrentRawPredecessorPacketSource(packetSource)||packetSource===serializeProjectData(packet,policy.output),'an unknown current packet is never overwritten');
  if(check)checkProjectData(new URL(policy.output,root),packet);else writeProjectData(new URL(policy.output,root),packet);
  const expected=source.replace(pattern,"const policySha256='"+pin(readFileSync(new URL(policy.output,root))).sha256+"';");
  if(check)assert.equal(source,expected);else{for(const row of writes)writeFileSync(new URL(row.file,root),row.source);writeFileSync(helperPath,expected);}
  return packet;
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
  const args=process.argv.slice(2);assert.ok(args.length<=1&&args.every(arg=>arg==='--check'));
  generateCurrentOwnCodebaseRawSourceDescriptors({check:args.includes('--check')});
  console.log('One exact current raw source provenance edge authored; all whole predecessors retained, without compilation or native execution.');
}
