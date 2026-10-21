// Compose existing authenticated classifier history with one additive
// conversation source successor. Current discovery and requirements stay live.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readBootstrapStructuredDataLino} from '../../scripts/lib/structured-data-bootstrap.mjs';
import {tokenize} from '../src/translation/lexer.js';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {readOwnCodebaseTestRegistrationHistory,restoreOwnCodebaseTestRegistrationClassifier} from './own-codebase-test-registration-history.mjs';
const root=new URL('../../',import.meta.url);
export const TOOLING_FROZEN_SOURCE_REVISION='7881e1e6f83818e0b25b77e1b3ab15c2d51ca9f3';
export const CRITICAL_DELIVERY_SOURCE_HISTORY={path:'parity/self-translation/critical-delivery-source-history.lino',bytes:419191,
  sha256:'2ee6113331e91c67baad1e639d0dc34f1b1e408c214eb162ff7ec34b23842a83'};
export const CURRENT_CONVERSATION_SOURCE={path:'js/scripts/issue-195-conversation-requirements.mjs',bytes:72335,
  sha256:'fa1bda21f330724dce2bda4d068fbb7758e0dcb45d4494044171138721e088f2'};
const policyPath='parity/self-translation/tooling-frozen-source-history.lino';
const policySha256='83bbc27757f467f73c70f8dad5074c70a46414455a4602f6c25b48879017cf47';
const hostSuccessorPath='parity/self-translation/tooling-frozen-host-successors.lino';
const hostSuccessorRevision='8f2f6c483ec32a3ffc17afec694463965dc7551d';
const hostSuccessorFiles=['js/tests/ci-workflow-structure.test.js','js/tests/test-groups.test.js','rust/Cargo.toml'];
const hostPredecessorRevision='8bc0ef9725388c7927acf1854870bf46f5e365be';
let hostSuccessorSnapshot;
export const toolingFrozenSourcePin=value=>({bytes:Buffer.byteLength(value),sha256:createHash('sha256').update(value).digest('hex')});
const exactKeys=(value,keys)=>{assert.ok(value&&typeof value==='object'&&!Array.isArray(value));assert.deepEqual(Object.keys(value).sort(),keys.slice().sort());};

export const TOOLING_FROZEN_HOSTS=[
  {path:'js/tests/ci-workflow-structure.test.js',previous:{bytes:199810,sha256:'7d21aa888965d15f17fb41c1b504c84b68a0916e9a1700b04d74ac43ecbddcba'},changes:[
    {before:"import { partitionGroupFiles, partitionTests, testFiles, TEST_GROUPS } from '../scripts/test-groups.mjs';\n",
      after:"import { partitionGroupFiles, partitionTests, testFiles, TEST_GROUPS } from '../scripts/test-groups.mjs';\nimport {readFrozenToolingTestGroups} from '../scripts/tooling-frozen-source-history.mjs';\n"},
    {before:"  const addition = '|ci-cancellation-investigation$', source = TEST_GROUPS.tooling.source;\n",
      after:"  const historicalRegistration = readFrozenToolingTestGroups();\n  const addition = '|ci-cancellation-investigation$', source = historicalRegistration.groups.tooling.source;\n"},
    {before:"    const before = Object.entries(TEST_GROUPS).filter(([name, pattern]) =>\n      (name === 'tooling' ? originalTooling : pattern).test(file.replace(/\\.test\\.js$/u, ''))).map(([name]) => name);\n",
      after:"    const before = Object.entries(historicalRegistration.groups).filter(([name, pattern]) =>\n      (name === 'tooling' ? originalTooling : pattern).test(file.replace(/\\.test\\.js$/u, ''))).map(([name]) => name);\n"},
    {before:"    assert.deepEqual(after, ['ci-cancellation-investigation', ...laterAdditions, rocqStem].some(stem => file === `${stem}.test.js`) ? ['tooling'] : before, file);\n",
      after:"    const registered = historicalRegistration.registrations.find(row => row.file === file);\n    assert.deepEqual(after, ['ci-cancellation-investigation', ...laterAdditions, rocqStem].some(stem => file === `${stem}.test.js`) ? ['tooling'] : registered ? [registered.group] : before, file);\n"},
    {before:"  assert.ok(files.includes(`${rocqStem}.test.js`), 'the new Rocq controls are an actual discovered test file');\n",
      after:"  for (const registration of historicalRegistration.registrations) {\n    assert.ok(files.includes(registration.file), 'each later registered test is an actual discovered file');\n    const actual = partitionTests([registration.file]);\n    assert.deepEqual(actual.unassigned, []);assert.deepEqual(actual.ambiguous, []);\n    assert.deepEqual(actual.groups[registration.group], [registration.file]);\n  }\n  assert.ok(files.includes(`${rocqStem}.test.js`), 'the new Rocq controls are an actual discovered test file');\n"},
  ]},
  {path:'experiments/critical-delivery/generate-source-history.mjs',previous:{bytes:2791,sha256:'f327f499381454eeb8180e31f67b55fcfb4f1aa622d9501080ba8152bc8aa68f'},changes:[
    {before:"import {verifyGitArtifactCertificate} from '../../js/scripts/git-object-certificate.mjs';\n",
      after:"import {verifyGitArtifactCertificate} from '../../js/scripts/git-object-certificate.mjs';\nimport {restoreCriticalDeliveryConversationSource} from '../../js/scripts/tooling-frozen-source-history.mjs';\n"},
    {before:"  const sourceText=original.blob.toString('utf8'),current=readFileSync(new URL(file,root),'utf8');\n",
      after:"  const sourceText=original.blob.toString('utf8'),current=restoreCriticalDeliveryConversationSource(readFileSync(new URL(file,root),'utf8'));\n"},
  ]},
  {path:'js/tests/issue-195-evidence-critical-delivery.test.js',previous:{bytes:9254,sha256:'af493eb796d7fe2384d23e348ed272affc0ffe136f2692c6e86516f8dc18538e'},changes:[
    {before:"import {generateCriticalDeliverySourceHistory} from '../../experiments/critical-delivery/generate-source-history.mjs';\n",
      after:"import {generateCriticalDeliverySourceHistory} from '../../experiments/critical-delivery/generate-source-history.mjs';\nimport {restoreCriticalDeliveryConversationSource} from '../scripts/tooling-frozen-source-history.mjs';\n"},
    {before:"  const history=generateCriticalDeliverySourceHistory({check:true}),previous=history.sources[0],current=readFileSync(new URL(previous.file,root),'utf8');\n",
      after:"  const history=generateCriticalDeliverySourceHistory({check:true}),previous=history.sources[0],current=restoreCriticalDeliveryConversationSource(readFileSync(new URL(previous.file,root),'utf8'));\n"},
  ]},
];

export function replayToolingFrozenHostChanges(source,changes){
  let current=source;const edits=[];
  for(const change of changes){
    assert.equal(current.split(change.before).length,2,'one exact reviewed scope transport interval');
    const start=current.indexOf(change.before);edits.push({start,removed:change.before,inserted:change.after});
    current=current.slice(0,start)+change.after+current.slice(start+change.before.length);
  }
  let inverse=current;
  for(const edit of edits.toReversed()){
    assert.equal(inverse.slice(edit.start,edit.start+edit.inserted.length),edit.inserted);
    inverse=inverse.slice(0,edit.start)+edit.removed+inverse.slice(edit.start+edit.inserted.length);
  }
  assert.equal(inverse,source,'all original inputs assertions guards and source bytes remain recoverable');
  return{source:current,edits};
}

/** Read exactly the certified object literal; do not evaluate source text or
 * guess which later regex alternatives can be removed. */
export function readFrozenToolingTestGroups(classifier=readFileSync(new URL('js/scripts/test-groups.mjs',root))){
  const history=readOwnCodebaseTestRegistrationHistory();
  const source=restoreOwnCodebaseTestRegistrationClassifier(classifier).toString('utf8');
  const tokens=tokenize(source,'JavaScript').tokens;
  const owners=tokens.flatMap((token,index)=>token.kind==='identifier'&&token.value==='TEST_GROUPS'
    &&tokens[index-1]?.value==='const'?[index]:[]);
  assert.equal(owners.length,1);let at=owners[0]+1;
  for(const value of ['=','Object','.','freeze','(','{'])assert.equal(tokens[at++].value,value);
  const groups={};
  while(tokens[at]?.value!=='}'){
    const key=tokens[at++];assert.ok(key.kind==='identifier'||key.kind==='string');
    assert.equal(tokens[at++].value,':');const literal=tokens[at++];assert.equal(literal.kind,'regex');
    const end=literal.raw.lastIndexOf('/');assert.ok(end>0);assert.equal(Object.hasOwn(groups,key.value),false);
    groups[key.value]=new RegExp(literal.raw.slice(1,end),literal.raw.slice(end+1));
    if(tokens[at]?.value===',')at+=1;else assert.equal(tokens[at]?.value,'}');
  }
  assert.ok(groups.tooling instanceof RegExp);
  return{source,groups,registrations:structuredClone(history.registrations)};
}

export function readCriticalDeliveryFrozenSource(){
  const bytes=readFileSync(new URL(CRITICAL_DELIVERY_SOURCE_HISTORY.path,root));
  assert.deepEqual(toolingFrozenSourcePin(bytes),{bytes:CRITICAL_DELIVERY_SOURCE_HISTORY.bytes,sha256:CRITICAL_DELIVERY_SOURCE_HISTORY.sha256});
  const history=readBootstrapStructuredDataLino(bytes.toString('utf8'));
  assert.equal(history.schemaVersion,1);assert.equal(history.revision,'e5d0ff8265bcd3136cffd42d53942c2e4b4eb2ea');assert.equal(history.sources.length,1);
  const original=history.sources[0];assert.equal(original.file,CURRENT_CONVERSATION_SOURCE.path);
  verifyGitArtifactCertificate(original.certificate,Buffer.from(original.sourceText),{revision:history.revision,path:original.file});
  assert.equal(original.sourceText.split(history.anchor).length,2);
  const source=original.sourceText.replace(history.anchor,history.anchor+history.addition);
  assert.deepEqual(toolingFrozenSourcePin(source),original.current);
  return{history,source};
}

/** Discover one exact array-list insertion automatically from complete sources.
 * Existing obligations may not be replaced, reordered or edited. */
export function readCriticalDeliveryAppend(previous,current){
  let start=0;while(start<previous.length&&start<current.length&&previous[start]===current[start])start+=1;
  assert.equal(previous.slice(start),']);\n','new obligations append only at the original requirement-list closing delimiter');
  const end=current.length-(previous.length-start);
  assert.ok(end>start);assert.equal(current.slice(0,start)+current.slice(end),previous);
  const source=current.slice(start,end);
  const ids=tokenize(source,'JavaScript').tokens.filter(token=>token.kind==='string'&&token.value.startsWith('I195-')).map(token=>token.value);
  assert.ok(ids.length>0);assert.equal(ids.length,new Set(ids).size);
  return{start,source,ids};
}

export function verifyToolingFrozenSourceHistory(packet,{hostSuccessors=false}={}){
  if(hostSuccessors){
    exactKeys(packet,['schemaVersion','model','revision','sources','boundary']);
    assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'certified tooling host successors 1');
    assert.equal(packet.revision,hostSuccessorRevision);assert.equal(typeof packet.boundary,'string');
    assert.ok(Array.isArray(packet.sources));assert.deepEqual(packet.sources.map(row=>row.path),hostSuccessorFiles);
    const oldBytes=readFileSync(new URL(policyPath,root));assert.equal(toolingFrozenSourcePin(oldBytes).sha256,policySha256);
    const predecessors=verifyToolingFrozenSourceHistory(readBootstrapStructuredDataLino(oldBytes.toString('utf8')));
    return packet.sources.map(row=>{
      exactKeys(row,['path','source','bytes','sha256','certificate','previous','edit']);
      assert.deepEqual(toolingFrozenSourcePin(row.source),{bytes:row.bytes,sha256:row.sha256});
      verifyGitArtifactCertificate(row.certificate,Buffer.from(row.source),{revision:hostSuccessorRevision,path:row.path});
      exactKeys(row.previous,['revision','source','bytes','sha256','certificate']);
      const previous=row.previous.source;
      assert.equal(row.previous.revision,hostPredecessorRevision);
      assert.deepEqual(toolingFrozenSourcePin(previous),{bytes:row.previous.bytes,sha256:row.previous.sha256});
      verifyGitArtifactCertificate(row.previous.certificate,Buffer.from(previous),{revision:hostPredecessorRevision,path:row.path});
      const existing=predecessors.find(candidate=>candidate.path===row.path);
      if(existing)assert.equal(previous,existing.current);
      exactKeys(row.edit,['start','removed','inserted']);
      const {start,removed,inserted}=row.edit;
      assert.ok(Number.isSafeInteger(start)&&start>=0);assert.equal(typeof removed,'string');assert.equal(typeof inserted,'string');
      assert.ok(removed.length||inserted.length);assert.equal(previous.slice(start,start+removed.length),removed);
      assert.equal(previous.slice(0,start)+inserted+previous.slice(start+removed.length),row.source);
      assert.equal(row.source.slice(start,start+inserted.length),inserted);
      assert.equal(row.source.slice(0,start)+removed+row.source.slice(start+inserted.length),previous);
      return{path:row.path,previous,current:row.source};
    });
  }
  exactKeys(packet,['schemaVersion','model','revision','coordinateUnit','criticalHistory','conversation','hosts','boundary']);
  assert.equal(packet.schemaVersion,1);assert.equal(packet.model,'composed CI tooling historical scope 1');
  assert.equal(packet.revision,TOOLING_FROZEN_SOURCE_REVISION);assert.equal(packet.coordinateUnit,'UTF-16 code units');
  assert.deepEqual(packet.criticalHistory,CRITICAL_DELIVERY_SOURCE_HISTORY);
  exactKeys(packet.conversation,['path','source','bytes','sha256','certificate','previous','append']);
  assert.deepEqual({path:packet.conversation.path,bytes:packet.conversation.bytes,sha256:packet.conversation.sha256},CURRENT_CONVERSATION_SOURCE);
  assert.deepEqual(toolingFrozenSourcePin(packet.conversation.source),{bytes:CURRENT_CONVERSATION_SOURCE.bytes,sha256:CURRENT_CONVERSATION_SOURCE.sha256});
  verifyGitArtifactCertificate(packet.conversation.certificate,Buffer.from(packet.conversation.source),{revision:TOOLING_FROZEN_SOURCE_REVISION,path:CURRENT_CONVERSATION_SOURCE.path});
  const frozen=readCriticalDeliveryFrozenSource();assert.equal(packet.conversation.previous.source,frozen.source);
  assert.deepEqual(packet.conversation.previous,{source:frozen.source,...toolingFrozenSourcePin(frozen.source)});
  assert.deepEqual(packet.conversation.append,readCriticalDeliveryAppend(frozen.source,packet.conversation.source));
  assert.deepEqual(packet.hosts.map(row=>row.path),TOOLING_FROZEN_HOSTS.map(row=>row.path));
  const outputs=[];
  for(const[index,row]of packet.hosts.entries()){
    const expected=TOOLING_FROZEN_HOSTS[index];exactKeys(row,['path','source','bytes','sha256','certificate','current','edits']);
    assert.deepEqual({bytes:row.bytes,sha256:row.sha256},expected.previous);
    assert.deepEqual(toolingFrozenSourcePin(row.source),expected.previous);
    verifyGitArtifactCertificate(row.certificate,Buffer.from(row.source),{revision:TOOLING_FROZEN_SOURCE_REVISION,path:row.path});
    const replay=replayToolingFrozenHostChanges(row.source,expected.changes);
    assert.deepEqual(row.edits,replay.edits);assert.deepEqual(row.current,toolingFrozenSourcePin(replay.source));
    outputs.push({path:row.path,previous:row.source,current:replay.source});
  }
  return outputs;
}

export function readToolingFrozenSourceHistory({hostSuccessors=false}={}){
  if(hostSuccessors){
    return structuredClone(readToolingHostSuccessorSnapshot().packet);
  }
  const bytes=readFileSync(new URL(policyPath,root));assert.equal(toolingFrozenSourcePin(bytes).sha256,policySha256);
  const packet=readBootstrapStructuredDataLino(bytes.toString('utf8'));
  for(const row of verifyToolingFrozenSourceHistory(packet))assert.equal(restoreToolingHostSuccessor(readFileSync(new URL(row.path,root),'utf8'),row.path),row.current);
  return packet;
}

/** Compose only the three complete source hosts already authenticated by
 * this packet. A selected unknown successor cannot acquire an older guard view. */
export function restoreToolingFrozenHostSource(source,file,{hostSuccessorsOnly=false,preserveUnknownSource=false}={}){
  assert.equal(typeof source,'string');assert.equal(typeof file,'string');
  if(hostSuccessorsOnly)return restoreToolingHostSuccessor(source,file,preserveUnknownSource);
  if(!TOOLING_FROZEN_HOSTS.some(row=>row.path===file))return source;
  const packet=readToolingFrozenSourceHistory(),row=packet.hosts.find(candidate=>candidate.path===file);
  assert.ok(row);if(source===row.source)return source;
  source=restoreToolingHostSuccessor(source,file);
  assert.deepEqual(toolingFrozenSourcePin(source),row.current,'an unknown tooling host successor has no complete historical source authority');
  const expected=TOOLING_FROZEN_HOSTS.find(candidate=>candidate.path===file);
  assert.equal(source,replayToolingFrozenHostChanges(row.source,expected.changes).source);
  return row.source;
}

function restoreToolingHostSuccessor(source,file,preserveUnknownSource=false){
  if(!hostSuccessorFiles.includes(file))return source;
  const row=readToolingHostSuccessorSnapshot().views.find(candidate=>candidate.path===file);
  if(source===row.previous)return source;
  // Earlier guards still authenticate their own complete older epochs. An
  // unrecognized input gains no authority and remains visible to that guard.
  if(preserveUnknownSource&&source!==row.current)return source;
  assert.equal(source,row.current,'an unknown complete tooling host has no historical source authority');
  return row.previous;
}

function readToolingHostSuccessorSnapshot(){
  const bytes=readFileSync(new URL(hostSuccessorPath,root));
  // A dependency mutation remains visible even when the successor is cached.
  assert.equal(toolingFrozenSourcePin(readFileSync(new URL(policyPath,root))).sha256,policySha256);
  if(hostSuccessorSnapshot?.bytes.equals(bytes))return hostSuccessorSnapshot;
  hostSuccessorSnapshot=undefined;
  const text=bytes.toString('utf8');assert.deepEqual(Buffer.from(text),bytes);
  const packet=readBootstrapStructuredDataLino(text),views=verifyToolingFrozenSourceHistory(packet,{hostSuccessors:true});
  hostSuccessorSnapshot={bytes,packet,views};return hostSuccessorSnapshot;
}

/** Only the complete certified current source gets its older source view;
 * actual current requirements are still imported by the real manifest. */
export function restoreCriticalDeliveryConversationSource(source){
  assert.equal(typeof source,'string');
  const frozen=readCriticalDeliveryFrozenSource();if(source===frozen.source)return source;
  assert.deepEqual(toolingFrozenSourcePin(source),{bytes:CURRENT_CONVERSATION_SOURCE.bytes,sha256:CURRENT_CONVERSATION_SOURCE.sha256},'an unknown conversation source cannot acquire historical scope authority');
  const packet=readToolingFrozenSourceHistory();assert.equal(source,packet.conversation.source);
  assert.equal(source.slice(0,packet.conversation.append.start)+source.slice(packet.conversation.append.start+packet.conversation.append.source.length),frozen.source);
  return frozen.source;
}
