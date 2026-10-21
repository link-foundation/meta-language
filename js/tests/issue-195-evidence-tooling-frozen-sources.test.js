// Current obligations/classification remain live; old byte pins keep their
// own complete certified source scopes. No synthetic completion evidence.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {createHash} from 'node:crypto';
import {TEST_GROUPS,testFiles,partitionTests} from '../scripts/test-groups.mjs';
import {CRITICAL_CONVERSATION_REQUIREMENTS} from '../scripts/issue-195-conversation-requirements.mjs';
import {buildIssue195Manifest} from '../scripts/issue-195-acceptance-lib.mjs';
import {readProjectData} from '../scripts/project-data.mjs';
import {generateToolingFrozenSourceHistory} from '../scripts/generate-tooling-frozen-source-history.mjs';
import {CRITICAL_DELIVERY_SOURCE_HISTORY,CURRENT_CONVERSATION_SOURCE,readFrozenToolingTestGroups,
  readCriticalDeliveryFrozenSource,readCriticalDeliveryAppend,readToolingFrozenSourceHistory,
  verifyToolingFrozenSourceHistory,restoreCriticalDeliveryConversationSource,restoreToolingFrozenHostSource,toolingFrozenSourcePin} from '../scripts/tooling-frozen-source-history.mjs';
import {LinkNetwork} from '../src/network.js';
import {encodeStructuredDataLinks} from '../src/structured-data-links.js';
import {decodeStructuredDataLinks} from '../src/structured-data-graph.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const root=new URL('../../',import.meta.url);
const read=file=>readFileSync(new URL(file,root));
const digest=value=>createHash('sha256').update(value).digest('hex');
const packet=()=>readProjectData(new URL('parity/self-translation/tooling-frozen-source-history.lino',root));

test('active Node selects the explicit tooling historical-scope assertion range',()=>{
  const[major,minor]=process.versions.node.split('.').map(Number);
  assert.ok(major===22&&minor>=15||major===24,
    'uncovered Node '+process.versions.node+'; supported >=22.15.0 <23.0.0 || >=24.0.0 <25.0.0');
});

test('existing whole-classifier certificate preserves frozen hashes and every current discovered file plus all registered additions',()=>{
  const historical=readFrozenToolingTestGroups(),current=partitionTests(testFiles()),files=testFiles();
  assert.equal(digest(historical.groups.tooling.source.replace('|ci-rocq-toolchain-cache$','')),
    '8060de05e180631e6234aaaded7a603bf0c886cacee6a814df01675f0f4a80c2');
  assert.equal(digest(historical.groups.tooling.source),'970823e432915eaf5a5143c87d901b2be7b73c3e2f61bc23786bf2f8992f497e');
  assert.deepEqual(current.unassigned,[]);assert.deepEqual(current.ambiguous,[]);
  assert.equal(Object.values(current.groups).flat().length,files.length);
  for(const file of files){
    const before=Object.entries(historical.groups).filter(([,pattern])=>pattern.test(file.replace(/\.test\.js$/u,''))).map(([group])=>group);
    const actual=Object.entries(current.groups).filter(([,members])=>members.includes(file)).map(([group])=>group);
    const registered=historical.registrations.find(row=>row.file===file);
    assert.deepEqual(actual,registered?[registered.group]:before,file);
  }
  for(const row of historical.registrations){
    assert.ok(files.includes(row.file));assert.deepEqual(partitionTests([row.file]).groups[row.group],[row.file]);
    const stem=row.file.replace(/\.test\.js$/u,''),neighbors=[stem+'-extra.test.js',stem+'x.test.js'];
    assert.deepEqual(partitionTests(neighbors).unassigned,neighbors);
  }
  assert.throws(()=>readFrozenToolingTestGroups(Buffer.concat([read('js/scripts/test-groups.mjs'),Buffer.from('\n')])));
  const fresh=readFrozenToolingTestGroups();historical.groups.tooling=/^unknown$/u;
  assert.equal(fresh.groups.tooling.source,TEST_GROUPS.tooling.source.replace('|own-codebase-baseline-naming-history$',''));
});

test('the complete frozen conversation source and its exact additive successor preserve all current new critical manifest rows',async()=>{
  const history=readToolingFrozenSourceHistory(),actual=read(CURRENT_CONVERSATION_SOURCE.path).toString('utf8');
  const frozen=readCriticalDeliveryFrozenSource();
  assert.equal(restoreCriticalDeliveryConversationSource(actual),frozen.source);
  assert.equal(restoreCriticalDeliveryConversationSource(frozen.source),frozen.source);
  assert.deepEqual(readCriticalDeliveryAppend(frozen.source,actual),history.conversation.append);
  assert.equal(history.conversation.append.ids.length,3);
  const manifest=await buildIssue195Manifest(new URL('../../',import.meta.url).pathname);
  for(const id of history.conversation.append.ids){
    const declarations=CRITICAL_CONVERSATION_REQUIREMENTS.filter(row=>row.id===id);assert.equal(declarations.length,1,id);
    const rows=manifest.atomicRequirements.filter(row=>row.id===id);assert.equal(rows.length,1,id);
    const declaration=declarations[0],row=rows[0];
    assert.equal(row.critical,true,id);assert.equal(row.fullScopeReleaseBlocker,true,id);
    assert.equal(row.expectedBehavior,declaration.expectedBehavior,id);
    assert.ok(row.requiredRuntimes.length&&row.verifications.length,id);
    for(const assertion of declaration.assertions)assert.ok(row.verifications.some(cell=>cell.assertions.includes(assertion)),id+': '+assertion);
    assert.ok(row.verifications.every(cell=>cell.fixtureIds.length&&cell.assertions.length));
    assert.equal(Object.hasOwn(row,'status'),false);
  }
  assert.equal(digest(read(CRITICAL_DELIVERY_SOURCE_HISTORY.path)),CRITICAL_DELIVERY_SOURCE_HISTORY.sha256);
});

test('unknown sources replacement edits coherent caller repins forged membership and source-scope omission refuse',()=>{
  const actual=read(CURRENT_CONVERSATION_SOURCE.path).toString('utf8'),frozen=readCriticalDeliveryFrozenSource().source;
  for(const source of [actual+'\n',actual.replace('Use as much automation','Use less automation'),frozen+'\n']){
    assert.throws(()=>restoreCriticalDeliveryConversationSource(source));
  }
  assert.throws(()=>readCriticalDeliveryAppend(frozen,frozen.replace('Missing implementations','Hidden implementations')+'\n'));
  const changes=[
    value=>{value.revision='0'.repeat(40);},value=>{value.criticalHistory.sha256='0'.repeat(64);},
    value=>{value.conversation.source+='\n';Object.assign(value.conversation,toolingFrozenSourcePin(value.conversation.source));},
    value=>{value.conversation.previous.source+='\n';Object.assign(value.conversation.previous,toolingFrozenSourcePin(value.conversation.previous.source));},
    value=>{value.conversation.append.start+=1;},value=>{value.conversation.append.source+='\n';},
    value=>{value.conversation.append.ids.pop();},value=>{value.conversation.certificate.path='foreign.mjs';},
    value=>{value.hosts.pop();},value=>{value.hosts.push(structuredClone(value.hosts[0]));},value=>{value.hosts.reverse();},
    value=>{value.hosts[0].source+='\n';Object.assign(value.hosts[0],toolingFrozenSourcePin(value.hosts[0].source));},
    value=>{value.hosts[0].edits[0].inserted+='\n';},value=>{value.hosts[0].certificate.revision='0'.repeat(40);},
    value=>{value.futureSourcePermission=true;},
  ];
  for(const[index,mutate]of changes.entries()){
    const value=packet();mutate(value);assert.throws(()=>verifyToolingFrozenSourceHistory(value),assert.AssertionError,'tamper '+index);
  }
});

test('the complete source proof reloads through both Links codecs without changing sources obligations or registrations',()=>{
  const value=readToolingFrozenSourceHistory(),saved=encodeStructuredDataLinks(value);
  const canonical=LinkNetwork.fromLino(saved.network.toLino());
  for(const network of [canonical,decodeLinkNetworkBinary(encodeLinkNetworkBinary(canonical))]){
    const restored=decodeStructuredDataLinks(network,saved.root);assert.deepEqual(restored,value);
    assert.deepEqual(verifyToolingFrozenSourceHistory(restored),verifyToolingFrozenSourceHistory(value));
  }
});

test('complete certified tooling host predecessors compose with old guards while unknown selected source successors refuse',()=>{
  const history=readToolingFrozenSourceHistory();
  const successors=readToolingFrozenSourceHistory({hostSuccessors:true});
  const views=verifyToolingFrozenSourceHistory(successors,{hostSuccessors:true});
  for(const row of history.hosts){
    const current=read(row.path).toString('utf8'),previous=row.source;
    assert.notEqual(current,previous);
    const successor=successors.sources.find(candidate=>candidate.path===row.path);
    if(successor){
      assert.deepEqual(toolingFrozenSourcePin(current),{bytes:successor.bytes,sha256:successor.sha256});
      const view=views.find(candidate=>candidate.path===row.path);
      assert.equal(current,view.current);assert.deepEqual(toolingFrozenSourcePin(view.previous),row.current);
    }else assert.deepEqual(toolingFrozenSourcePin(current),row.current);
    assert.equal(restoreToolingFrozenHostSource(current,row.path),previous);
    assert.equal(restoreToolingFrozenHostSource(previous,row.path),previous);
    for(const source of [current+'\n',previous+'\n',current.replace('import assert','import alteredAssert')]){
      assert.throws(()=>restoreToolingFrozenHostSource(source,row.path),assert.AssertionError);
    }
    for(const path of ['unregistered.js','js/tests/issue-195-evidence-ci-node-runtime.test.js',row.path.toUpperCase()]){
      assert.equal(restoreToolingFrozenHostSource(current,path),current);
      assert.equal(restoreToolingFrozenHostSource(current+'\n',path),current+'\n');
    }
  }
  const detached=readToolingFrozenSourceHistory();detached.hosts[0].source='caller mutation';
  const original=history.hosts[0];assert.equal(restoreToolingFrozenHostSource(read(original.path).toString('utf8'),original.path),original.source);
});

test('the certified workflow host successor retains the whole original inverse and refuses altered source authority',()=>{
  const history=readToolingFrozenSourceHistory({hostSuccessors:true});
  assert.deepEqual(generateToolingFrozenSourceHistory({hostSuccessors:true,check:true}),history);
  const [view]=verifyToolingFrozenSourceHistory(history,{hostSuccessors:true});
  const prior=readToolingFrozenSourceHistory().hosts.find(row=>row.path===view.path);
  assert.deepEqual(toolingFrozenSourcePin(view.previous),prior.current);
  assert.equal(view.current,read(view.path).toString('utf8'));
  assert.equal(restoreToolingFrozenHostSource(view.previous,view.path),prior.source);
  assert.equal(restoreToolingFrozenHostSource(view.current,view.path),prior.source);
  for(const source of verifyToolingFrozenSourceHistory(history,{hostSuccessors:true})){
    assert.equal(restoreToolingFrozenHostSource(source.current,source.path,{hostSuccessorsOnly:true}),source.previous);
    assert.equal(restoreToolingFrozenHostSource(source.previous,source.path,{hostSuccessorsOnly:true}),source.previous);
    assert.throws(()=>restoreToolingFrozenHostSource(source.current+'\n',source.path,{hostSuccessorsOnly:true}));
  }
  for(const mutate of [
    value=>{value.revision='0'.repeat(40);},value=>{value.sources=[];},
    value=>{value.sources.push(structuredClone(value.sources[0]));},
    value=>{value.sources[0].path='unregistered.js';},
    value=>{const row=value.sources[0];row.source+='\n';Object.assign(row,toolingFrozenSourcePin(row.source));},
    value=>{value.sources[0].certificate.path='unregistered.js';},
    value=>{value.sources[0].certificate.revision='0'.repeat(40);},
    value=>{value.sources[0].previous.sha256='0'.repeat(64);},
    value=>{value.sources[0].edit.start++;},
    value=>{value.sources[0].edit.removed+='\n';},
    value=>{value.sources[0].edit.inserted+='\n';},
    value=>{value.futureSourcePermission=true;},
  ]){const changed=structuredClone(history);mutate(changed);assert.throws(()=>verifyToolingFrozenSourceHistory(changed,{hostSuccessors:true}));}
  assert.throws(()=>restoreToolingFrozenHostSource(view.current+'\n',view.path));
  assert.throws(()=>generateToolingFrozenSourceHistory({hostSuccessors:true,apply:true}));
});

test('normal capture leaves the current classifier requirements and old critical source history untouched',()=>{
  const paths=['js/scripts/test-groups.mjs',CURRENT_CONVERSATION_SOURCE.path,CRITICAL_DELIVERY_SOURCE_HISTORY.path];
  const before=paths.map(read),history=generateToolingFrozenSourceHistory({check:true});
  assert.deepEqual(history,packet());paths.forEach((path,index)=>assert.deepEqual(read(path),before[index]));
});
