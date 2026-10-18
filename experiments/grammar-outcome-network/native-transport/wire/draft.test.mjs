// PRIVATE controls; these do NOT execute native Rust or a concrete direct sink.
import assert from 'node:assert/strict';
import test from 'node:test';
import {encodeStructuredDataLinks,decodeStructuredDataLinks} from '../../../../js/src/structured-data-links.js';
import {readOutcomeInventory,checkOutcomeInventory,buildOrderedWirePlan,renderBorrowedVisitors} from './generate.mjs';
import {outcomeWireEvents,preflightOutcome} from './events.mjs';

// One actual complete-CST inventory for this bounded file; no canonical registry.
let inventory,plan;
const current=()=>{inventory??=readOutcomeInventory();plan??=buildOrderedWirePlan(inventory);return plan;};
const limits={nodeLimit:1_000_000,depthLimit:256,byteLimit:128*1024*1024};
const owned=name=>({kind:'owned',name});
const trace=(name,value)=>[...outcomeWireEvents(current(),value,owned(name))];
const keys=(name,value)=>trace(name,value).filter(event=>event.key&&event.depth===3).map(event=>event.value);
const leaf=(variant,payload)=>({$variant:variant,payload});
const token=(extra={})=>({$variant:'Token',trivia:false,start:0,end:1,text:leaf('Text','a'),...extra});

test('current complete native CST ownership supplies nine exhaustive declarations and only authenticated root paths',()=>{
  const source=renderBorrowedVisitors((current(),inventory),plan);
  assert.equal(inventory.declarations.length,9);assert.equal(inventory.inputs.length,4);
  assert.equal(inventory.declarations.find(item=>item.name==='SyntaxAttributes').alias,'IndexMap<String,OperationValue>');
  assert.ok(inventory.rootReexport.source.includes('ParseOutcome'));
  for(const edit of [
    value=>value.declarations.find(item=>item.name==='ParseOutcome').fields.push({name:'future',type:'bool'}),
    value=>value.declarations.find(item=>item.name==='SyntaxTree').variants.push({name:'Future',unit:true}),
    value=>{value.declarations.find(item=>item.name==='SyntaxAttributes').alias='BTreeMap<String,OperationValue>';},
    value=>{value.declarations.find(item=>item.name==='SyntaxTree').variants.find(item=>item.name==='Embed').fields.find(item=>item.name==='selection').type='Option<Box<foreign::GrammarSelection>>';},
    value=>value.inputs[0].imports.push({source:'use foreign::SyntaxTree;'}),
    value=>{value.rootReexport.source=value.rootReexport.source.replace('LeafText,','LeafText as OtherText,');},
  ]) {const forged=structuredClone(inventory);edit(forged);assert.throws(()=>checkOutcomeInventory(forged));}
  for(const name of ['parse_outcome','parse_rejection','ambiguity','grammar_selection','grammar_selection_alternative','syntax_tree','leaf_text','operation_value','syntax_attributes','json'])
    assert.ok(source.includes('fn wire_'+name+'<S: OutcomeWireSink>'));
  assert.ok(source.includes('type Node;')&&source.includes('FnOnce(&mut Self, usize)'));
  for(const forbidden of ['..','_ =>','.clone(', 'from_json','to_value',' as f64']) assert.ok(!source.includes(forbidden),forbidden);
  // These are borrowed input uses, never allocated intermediate values.
  assert.ok(source.includes('value: &serde_json::Value')&&source.includes('value: &crate::SyntaxAttributes'));
  assert.deepEqual(buildOrderedWirePlan(inventory),plan);
});

test('ordered events preserve option presence flattening flags and current JavaScript construction order',()=>{
  const ordinary={ok:false,ambiguities:[]};
  assert.deepEqual(keys('ParseOutcome',ordinary),['ok','tree','ambiguities','rejection']);
  assert.deepEqual(keys('ParseOutcome',{...ordinary,source_planes:[]}),['ok','tree','ambiguities','rejection','sourcePlanes']);
  const rejection={reason:'syntax',offset:0,line:1,column:1,expected:[],limit:0,diagnostics:[],offset_semantics:''};
  assert.deepEqual(keys('ParseRejection',rejection),['reason','offset','line','column','expected','limit','diagnostics','offsetSemantics']);
  const selection={mode:'any-grammar',mapping:'field-ranges',start:0,end:1,candidates:[],provenance:{},alternatives:[],source_mapping:null,ambiguous:false};
  assert.deepEqual(keys('GrammarSelection',selection),['mode','mapping','start','end','candidates','provenance','alternatives','sourceMapping','ambiguous']);
  assert.ok(trace('GrammarSelection',selection).some(event=>event.kind==='boolean'&&event.value===false));
  const ambiguity={rule:'r',start:0,end:1,languages:[],source:0,source_start:0,source_end:0,offset_semantics:'',source_route:[]};
  assert.deepEqual(keys('Ambiguity',ambiguity),['rule','start','end','languages','source','sourceStart','sourceEnd','offsetSemantics','sourceRoute']);
  assert.deepEqual(keys('SyntaxTree',token({field:'',attributes:new Map(),lexical:null,text:leaf('Hex','ff')})),['type','kind','field','start','end','attributes','lexical','text','hex']);
  assert.deepEqual(keys('SyntaxTree',{$variant:'Error',start:0,end:0,text:leaf('Text',''),reason:''}),['type','start','end','text','reason']);
  assert.deepEqual(keys('SyntaxTree',{$variant:'Missing',start:0,end:0,literal:false}),['type','kind','start','end']);
  assert.deepEqual(keys('SyntaxTree',{$variant:'Embed',language:'Json',start:0,end:1,root:token(),source_children:[],selection}),['type','language','start','end','root','sourceChildren','selection']);
  assert.deepEqual(keys('SyntaxTree',{$variant:'Node',kind:'root',trivia:true,start:0,end:1,children:[token()],attributes:new Map()}),['type','kind','trivia','start','end','children','attributes']);
  // Plain control fields encode native None by absence; present null is legal
  // for Option<Value>, not a fabricated Some(null SyntaxTree).
  assert.throws(()=>trace('ParseOutcome',{...ordinary,tree:null}));
  assert.throws(()=>trace('SyntaxTree',token({future:true})),/unknown native control field/);
});

test('attribute order opaque source references exact integers and signed zero remain observable',()=>{
  const attributes=new Map([
    ['2',leaf('Integer',2n)],['10',leaf('Integer',10n)],['z',leaf('Text','last')],['a',leaf('Text','')],['😀',leaf('Integer',0n)],
  ]);
  attributes.set('z',leaf('Text','overwritten'));
  const events=trace('SyntaxAttributes',attributes);
  assert.deepEqual(events.filter(event=>event.key).map(event=>event.value),['2','10','z','a','😀']);
  assert.ok(events.some(event=>event.value==='overwritten'));
  const value={ok:false,ambiguities:[],source_planes:[{kind:'grammar-source',identity:0,derivation:{input:0,negativeZero:-0},extra:[]}]};
  const sourceEvents=trace('ParseOutcome',value);
  assert.ok(sourceEvents.some(event=>event.kind==='number'&&Object.is(event.value,-0)));
  assert.equal(sourceEvents.filter(event=>event.kind==='number'&&Object.is(event.value,0)).length,2);
  assert.ok(preflightOutcome(current(),value,limits).scalarBytes>0);
  assert.doesNotThrow(()=>trace('OperationValue',leaf('Integer',1n<<60n)));
  assert.throws(()=>trace('OperationValue',leaf('Integer',(1n<<53n)+1n)),/exactly representable/);
  assert.throws(()=>preflightOutcome(current(),{...value,source_planes:[{invalid:Infinity}]},limits),/finite binary64/);
  assert.throws(()=>trace('SyntaxTree',token({text:leaf('Text','\ud800')})),/unpaired/);
});

test('expanded preflight matches an independent existing codec traversal and refuses at exact resource boundaries',()=>{
  current();
  const native={ok:false,ambiguities:[]};
  const expected={ok:false,tree:null,ambiguities:[],rejection:null};
  const encoded=encodeStructuredDataLinks(expected);
  const frames=[{id:encoded.root,depth:0}];
  let occurrences=0,scalarBytes=0,maximumDepth=0;
  while(frames.length) {
    const {id,depth}=frames.pop(),link=encoded.network.link(id);
    occurrences++;scalarBytes+=Buffer.byteLength(link.metadata().definition??'');maximumDepth=Math.max(maximumDepth,depth);
    for(const child of link.references()) frames.push({id:child,depth:depth+1});
  }
  const actual=preflightOutcome(plan,native,limits);
  assert.deepEqual(actual,{occurrences,scalarBytes,maximumDepth,references:occurrences-1});
  assert.deepEqual(decodeStructuredDataLinks(encoded.network,encoded.root,{nodeLimit:occurrences,depthLimit:maximumDepth,byteLimit:scalarBytes}),expected);
  assert.deepEqual(preflightOutcome(plan,native,{nodeLimit:occurrences,depthLimit:maximumDepth,byteLimit:scalarBytes}),actual);
  for(const [name,count] of [['nodeLimit',occurrences],['depthLimit',maximumDepth],['byteLimit',scalarBytes]])
    assert.throws(()=>preflightOutcome(plan,native,{...limits,[name]:count-1}),/resource limit/);
  // Repeated primitive references decode repeatedly; never relax the existing
  // encoder defaults or existing callers' intentionally larger decoder limits.
  const repeated={ok:false,ambiguities:[],source_planes:Array.from({length:30},()=>0)};
  const once=preflightOutcome(plan,{ok:false,ambiguities:[],source_planes:[0]},limits);
  const many=preflightOutcome(plan,repeated,limits);
  assert.equal(many.occurrences-once.occurrences,29);assert.equal(many.scalarBytes-once.scalarBytes,29);
  assert.throws(()=>preflightOutcome(plan,repeated,{...limits,nodeLimit:once.occurrences}),/resource limit/);
  assert.throws(()=>preflightOutcome(plan,{ok:false,ambiguities:[],source_planes:['x'.repeat(1024)]},{...limits,byteLimit:64}),/resource limit/);
  const nested={};nested.loop=nested;
  assert.throws(()=>preflightOutcome(plan,{ok:false,ambiguities:[],source_planes:[nested]},limits),/cyclic/);
});
