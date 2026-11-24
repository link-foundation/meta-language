// Deferred controls: never imported or executed by the source author.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {planProjectTranslationDependencyGaps} from '../src/project-translation-rules.js';
import {draftProjectTranslation,projectTranslationDraftToLinks} from '../src/project-translation.js';
import {decodeStructuredDataLinks} from '../src/structured-data-links.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';
const fixture=readProjectData(new URL('../../parity/fixtures/project-dependency-gaps.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.node);
for(const row of fixture.cases)test(row.name,()=>assert.deepEqual(planProjectTranslationDependencyGaps(row.count,row.failed,row.importers,row.providers,row.limit),row.expected));
const inputs=()=>[
 {identifier:'a.js',language:'JavaScript',source:fixture.source,sourceIssues:[fixture.sourceIssue]},
 {identifier:'b.js',language:'JavaScript',source:fixture.source,imports:{'./a.js':'a.js'}},
 {identifier:'c.js',language:'JavaScript',source:fixture.source,imports:{'./b.js':'b.js'}},
 {identifier:'d.js',language:'JavaScript',source:fixture.source}
];
test('source gaps propagate through actual complete project output and both codecs',()=>{
 const draft=draftProjectTranslation(inputs(),{targetLanguage:'JavaScript'}),byName=new Map(draft.modules.map(module=>[module.identifier,module]));
 assert.deepEqual(byName.get('a.js').sourceIssues,[fixture.sourceIssue]);assert.ok(byName.get('a.js').gaps.some(gap=>gap.diagnostic.name==='SourceInventoryGap'&&gap.diagnostic.message===fixture.sourceIssue));
 for(const [identifier,provider]of [['b.js','a.js'],['c.js','b.js']]){const module=byName.get(identifier);assert.equal(module.emittedCode,null);assert.equal(typeof module.draftCode,'string');assert.ok(module.gaps.some(gap=>gap.diagnostic.name==='DependencySourceGap'&&gap.diagnostic.provider===provider&&gap.diagnostic.root==='a.js'));}
 assert.equal(typeof byName.get('d.js').emittedCode,'string');
 const packet=projectTranslationDraftToLinks(draft);
 for(const network of [packet.network,LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))])assert.deepEqual(decodeStructuredDataLinks(network,packet.root),draft);
});
test('dependency budget exhaustion withholds every uncertified executable',()=>{
 const draft=draftProjectTranslation(inputs(),{targetLanguage:'JavaScript',dependencyWorkLimit:1});
 for(const module of draft.modules){assert.equal(module.emittedCode,null);assert.ok(module.gaps.some(gap=>gap.diagnostic.name==='DependencyWorkLimitGap'));}
});
test('invalid issue domains and budgets refuse without dropping diagnostics',()=>{
 for(const sourceIssues of [[''],[1],'issue'])assert.throws(()=>draftProjectTranslation([{...inputs()[0],sourceIssues}],{targetLanguage:'JavaScript'}),/invalid source inventory issues/u);
 assert.throws(()=>draftProjectTranslation(inputs(),{targetLanguage:'JavaScript',dependencyWorkLimit:0}),/invalid drafting resource limit/u);
});
test('actual provider translation failures also block transitive consumers',()=>{
 const sources=inputs();sources[0]={identifier:'a.js',language:'Rust',source:'pub fn value()->u8{unknown_operation()}'};
 const draft=draftProjectTranslation(sources,{targetLanguage:'JavaScript'}),modules=new Map(draft.modules.map(module=>[module.identifier,module]));
 assert.ok(modules.get('a.js').gaps.length>0);assert.deepEqual(modules.get('a.js').sourceIssues,[]);
 for(const identifier of ['b.js','c.js']){assert.equal(modules.get(identifier).emittedCode,null);assert.ok(modules.get(identifier).gaps.some(gap=>gap.diagnostic.name==='DependencySourceGap'&&gap.diagnostic.root==='a.js'));}
 assert.equal(typeof modules.get('d.js').emittedCode,'string');
});
test('duplicate source issues remain distinct and issue bytes have their own bound',()=>{
 const sources=[{...inputs()[3],sourceIssues:[fixture.sourceIssue,fixture.sourceIssue]}];
 const draft=draftProjectTranslation(sources,{targetLanguage:'JavaScript'});assert.deepEqual(draft.modules[0].sourceIssues,[fixture.sourceIssue,fixture.sourceIssue]);assert.equal(draft.modules[0].gaps.filter(gap=>gap.diagnostic.name==='SourceInventoryGap').length,2);
 assert.throws(()=>draftProjectTranslation([{...inputs()[3],sourceIssues:['é'.repeat(40)]}],{targetLanguage:'JavaScript',byteLimit:64}),/source issue byte resource limit exceeded/u);
});
