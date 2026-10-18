// PRIVATE future CI fixture author. No native process, repository write or observation ledger.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';

const args=process.argv.slice(2);
assert.equal(args.length,4);
assert.equal(args[0],'--repository');assert.equal(args[2],'--output');
const repository=resolve(args[1]),output=resolve(args[3]);
assert.ok(output!==repository&&!output.startsWith(repository+'/'),'fixture output is outside the original checkout');
const load=path=>import(pathToFileURL(resolve(repository,path)).href);
const {compileGrammar}=await load('js/src/grammar.js');
const {parseGrammarLinks,percentEncodeLinksText}=await load('js/src/grammar-links.js');
const {checkGrammarSelectionMappedTree}=await load('js/src/grammar-runtime.js');
const {encodeStructuredDataLinks,decodeStructuredDataLinks}=await load('js/src/structured-data-links.js');
const {LinkNetwork}=await load('js/src/network.js');
const {encodeLinkNetworkBinary,decodeLinkNetworkBinary}=await load('js/src/link-network-binary.js');
const {readProjectData,writeProjectData}=await load('js/scripts/project-data.mjs');
const {grammarSelectionFixture}=await load('js/tests/support/grammar-selection-cases.js');

// Counts expanded references of the independent, existing JS codec graph.
// Scalar spelling is read from that codec, never reimplemented here.
function metrics({network,root}) {
  const pending=[{id:root,depth:0}];let nodes=0,bytes=0,depth=0;
  while(pending.length){
    const item=pending.pop(),link=network.link(item.id);
    nodes+=1;bytes+=Buffer.byteLength(link.metadata().definition??'');depth=Math.max(depth,item.depth);
    for(const id of link.references())pending.push({id,depth:item.depth+1});
  }
  assert.ok(depth<=256);
  return {nodes,bytes,depth};
}
function capture(expected) {
  const packet=encodeStructuredDataLinks(expected),text=packet.network.toLino();
  const canonical=LinkNetwork.fromLino(text),binary=encodeLinkNetworkBinary(canonical);
  const restored=decodeLinkNetworkBinary(binary);
  assert.equal(restored.toLino(),canonical.toLino(),'only this actual graph is compared by identity');
  assert.deepEqual(decodeStructuredDataLinks(canonical,packet.root),expected);
  assert.deepEqual(decodeStructuredDataLinks(restored,packet.root),expected);
  return {expected,limits:metrics(packet),jsPhysicalNodes:packet.network.links().length};
}
const literal=text=>`(grammar (start document))\n(rule document normal (literal ${percentEncodeLinksText(text)}))\n`;
const pair='(grammar (start document))\n(rule document normal (seq (literal a) (literal b)))\n';
const selection=grammarSelectionFixture();
const grammars=Object.fromEntries(selection.grammars.map(row=>[row.name,row.listing]));
const attributes=readProjectData(resolve(repository,'parity/fixtures/grammar-attribute-order.lino')).cases[0];
const host='(grammar (start document))\n(rule document normal (any-grammar (ref physical) (grammar-range-candidate Inline included-ranges (source-range 0 1) (source-range 2 3))))\n(rule physical normal (literal a%2Bb))\n';
const definitions=[
  {id:'accepted-utf8',listing:literal('π'),source:'π',ok:true},
  {id:'rejected-literal',listing:literal('π'),source:'x',ok:false,reason:'syntax'},
  {id:'recovered-refused',listing:pair,source:'a',options:{errorRecovery:true},ok:false,reason:'recovered'},
  {id:'recovered-accepted',listing:pair,source:'a',options:{errorRecovery:true,recovery:'accept'},ok:true},
  {id:'attribute-order',listing:attributes.grammar,source:attributes.source,ok:true,attributeOrder:attributes.order},
  {id:'any-alternatives',listing:selection.any.listing,source:selection.any.source,grammars,ok:true},
  {id:'ambiguity-refused',listing:selection.any.listing,source:selection.any.source,grammars,options:{ambiguity:'reject'},ok:false,reason:'ambiguity'},
  {id:'nested-selection',listing:selection.consuming.listing,source:'{{x}}',grammars:{recursive:selection.consuming.listing},options:{grammarName:'recursive'},ok:true},
  {id:'included-source-planes',listing:host,source:'a+b',grammars:{Inline:'(grammar (start decoded))\n(rule decoded normal (literal ab))\n'},ok:true,mapped:true},
  {id:'unknown-selection',listing:selection.unknown.listing,source:selection.unknown.source,grammars,ok:false,reason:'unknownGrammar'},
  {id:'invalid-utf8-byte',listing:'(grammar (start document))\n(rule document normal any)\n',sourceBytes:[255],ok:true},
];
const cases=definitions.map(row=>{
  const linked=new Map(Object.entries(row.grammars??{}).map(([name,listing])=>[name,parseGrammarLinks(listing)]));
  const outcome=compileGrammar(parseGrammarLinks(row.listing),{...row.options,resolveGrammar:name=>linked.get(name)})
    .parseTree(row.sourceBytes?Uint8Array.from(row.sourceBytes):row.source);
  assert.equal(outcome.ok,row.ok,row.id);
  if(row.reason)assert.equal(outcome.rejection.reason,row.reason,row.id);
  if(row.attributeOrder)assert.deepEqual(Object.keys(outcome.tree.attributes),row.attributeOrder);
  if(row.mapped){assert.ok(outcome.sourcePlanes.length>0);assert.equal(checkGrammarSelectionMappedTree(outcome),outcome);}
  return {...row,origin:'fresh JavaScript public grammar outcome',...capture(outcome)};
});
// Independent constructed wire values, not outcomes claimed reachable from a grammar.
const opaque=()=>({negativeZero:-0,empty:[],nothing:null,flag:false,zero:0,text:'é'});
const token={type:'token',kind:null,start:0,end:1,attributes:{},lexical:opaque(),text:null,hex:'ff'};
const missing={type:'missing',kind:null,start:1,end:1};
const alternative={language:'alternative',root:missing,provenance:[]};
const selectionValue={mode:'boundary',mapping:'identity',start:0,end:4,candidates:[opaque()],provenance:null,
  alternatives:[alternative],sourceMapping:null,ambiguous:true};
const full={ok:false,tree:{type:'node',kind:'boundary',field:'',trivia:true,start:0,end:4,
  children:[token,{type:'error',start:1,end:3,text:'π',reason:''},
    {type:'missing',kind:'word',start:3,end:3,literal:true},
    {type:'embed',language:'child',start:0,end:4,root:{type:'token',kind:'word',field:'value',trivia:true,start:0,end:1,text:'x'},sourceChildren:[],selection:selectionValue}],
  attributes:{'2':2,'10':10,z:'last',a:'first',wide:2**53,minimum:-(2**63)}},
  ambiguities:[{rule:'boundary',start:0,end:4,languages:[],source:0,sourceStart:0,sourceEnd:4,offsetSemantics:'boundary',sourceRoute:[opaque()]}],
  rejection:{reason:'boundary',offset:0,line:1,column:2,expected:['x','π'],limit:9,diagnostics:[opaque()],offsetSemantics:'boundary'},sourcePlanes:[opaque()]};
for(const [id,expected]of [
  ['minimal',{ok:true,tree:null,ambiguities:[],rejection:null}],
  ['all-native-variants',full],
  ['repeated-scalars',{ok:true,tree:null,ambiguities:[],rejection:null,sourcePlanes:Array(32).fill(0)}],
])cases.push({id,origin:'independent constructed value; not a parser/mapping validity claim',...capture(expected)});
const inputs=['js/src/grammar-runtime.js','js/src/grammar-runtime/syntax-tree.js','js/src/grammar-runtime/grammar-selection.js',
  'js/src/structured-data-encode.js','js/tests/support/grammar-selection-cases.js','parity/fixtures/grammar-attribute-order.lino']
  .map(path=>{const bytes=readFileSync(resolve(repository,path));return {path,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(output,{schemaVersion:1,profile:'private borrowed native outcome probe 1',inputs,cases,
  boundary:'Generated expected values and graph-derived limits only. No native observations, cross-runtime node identity, default CST alias, source association or general language support claim.'});
console.log('Authored '+cases.length+' private expected rows; native observations remain pending.');
