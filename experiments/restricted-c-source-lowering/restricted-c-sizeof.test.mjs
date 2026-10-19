// Authored size-query controls for the source-only C lowering experiment.
import assert from 'node:assert/strict';
import test from 'node:test';
import {createHash} from 'node:crypto';
import {parseProgrammingLanguage} from '../../js/src/programming-language-parser.js';
import {LinkNetwork} from '../../js/src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../../js/src/link-network-binary.js';
import {lowerRestrictedCst,archiveRestrictedCLowering,readRestrictedCLowering} from './restricted-c-cst.mjs';
import {readProjectData} from '../../js/scripts/project-data.mjs';
import {generateRestrictedCSizeQueryPublication,checkRestrictedCSizeQueryPublication} from './sizeof-source-publication.mjs';

const abi={intBits:32,sizeBits:64,charSigned:true};
const lower=(source,options={})=>lowerRestrictedCst(parseProgrammingLanguage(source,'C'),source,{abi,...options});
const queries=fn=>fn.body.filter(item=>['C size query','unresolved C size query'].includes(item.operation));

test('sizeof source aliases retain declaration and ABI owners without folding enum representation into int width',()=>{
 for(const name of['Block','OtherSourceType']){
  const source=`typedef enum { FIRST, SECOND } ${name}; size_t extent(void) { return sizeof(${name}); }`;
  const unit=lower(source),query=queries(unit.functions[0])[0];
  assert.equal(query.operation,'C size query');
  assert.equal(query.form,'type');
  assert.equal(query.operandEvaluation,'unevaluated');
  assert.equal(query.byteSize,null);
  assert.equal(Object.hasOwn(query,'value'),false);
  assert.deepEqual(query.result.type,{kind:'C integer',bits:64,signed:false});
  assert.deepEqual(query.operand.type,{kind:'C integer',bits:32,signed:true});
  assert.equal(query.resultTypeBasis.abiOwner,unit.abi);
  assert.equal(unit.abi,unit.loweringOptions.abi);
  const declaration=unit.sourceCst.tree.children.find(node=>node.term==='type_definition');
  assert.equal(query.operand.owner.source,declaration);
  assert.equal(query.operand.owner.boundType,query.operand.type);
  assert.equal(query.operand.source,query.source.children.find(node=>node.field===(query.sourceForm==='type'?'type':'value')));
  if(query.sourceForm==='expression') {
   assert.equal(query.refinement.owner,query.operand.owner);
   assert.equal(query.refinement.rule,'visible type without value binding');
   assert.ok(unit.obligations.some(item=>item.category==='C sizeof namespace refinement'));
  }
  assert.equal(Buffer.from(source).subarray(query.span.start,query.span.end).toString(),`sizeof(${name})`);
  assert.ok(unit.obligations.some(item=>item.category==='C sizeof layout'));
  assert.ok(unit.obligations.some(item=>item.category==='C syntax derivation'));
  assert.ok(unit.obligations.some(item=>item.category==='C execution boundary'));
  assert.equal(unit.checkedPortableProgram,null);
  const packet=archiveRestrictedCLowering(unit),restored=readRestrictedCLowering(packet.network,packet.root);
  assert.deepEqual(restored,unit);
  const again=queries(restored.functions[0])[0];
  assert.equal(again.operand.owner.source,restored.sourceCst.tree.children.find(node=>node.term==='type_definition'));
  assert.equal(again.resultTypeBasis.abiOwner,restored.abi);
  const textNetwork=LinkNetwork.fromLino(packet.network.toLino());
  for(const network of[textNetwork,decodeLinkNetworkBinary(encodeLinkNetworkBinary(textNetwork))]) {
   const decoded=readRestrictedCLowering(network,packet.root),actual=queries(decoded.functions[0])[0];
   assert.deepEqual(decoded,unit);
   assert.equal(actual.operand.owner.source,decoded.sourceCst.tree.children.find(node=>node.term==='type_definition'));
   assert.equal(actual.operand.owner.boundType,actual.operand.type);
   assert.equal(actual.resultTypeBasis.abiOwner,decoded.loweringOptions.abi);
   assert.equal(actual.refinement?.owner,actual.refinement?actual.operand.owner:undefined);
   assert.equal(decoded.sourceCst.tokens.map(token=>token.text).join(''),source);
   assert.equal(decoded.checkedPortableProgram,null);
  }
 }
});

test('sizeof expression queries preserve update and call source without emitting operand effects',()=>{
 const source='int effect(void) { return 7; } size_t measure(int value) { return sizeof(value++) + sizeof(effect()); }';
 const unit=lower(source),fn=unit.functions.find(fn=>fn.name==='measure'),found=queries(fn);
 assert.equal(found.length,2);
 assert.ok(found.every(query=>query.operation==='C size query'&&query.form==='expression'&&query.operandEvaluation==='unevaluated'));
 assert.equal(found[0].operand.owner,fn.parameters[0]);
 assert.equal(found[1].operand.owner,unit.sourceCst.tree.children.find(node=>node.term==='function_definition'));
 assert.equal(fn.body.some(item=>['C call','store C lvalue','load C lvalue','address C binding'].includes(item.operation)),false);
 assert.ok(unit.obligations.some(item=>item.category==='C sizeof operand constraints'));
 assert.ok(unit.functions.some(fn=>fn.name==='effect'),'ordinary source closure remains a conservative declaration inventory');
 assert.equal(unit.sourceCst.tokens.map(token=>token.text).join(''),source);
 assert.equal(unit.checkedPortableProgram,null);
 const shadow=lower('int value; size_t measure(int value) { return sizeof(value); }');
 const localQuery=queries(shadow.functions[0])[0];
 assert.equal(localQuery.operand.owner,shadow.functions[0].parameters[0]);
 assert.notEqual(localQuery.operand.owner,shadow.globalInitializers.find(item=>item.operation==='declare C binding').binding);
});

test('variably modified array bounds and aliased qualified pointer or record shapes remain unresolved',()=>{
 const source='size_t measure(int count) { return sizeof(int[count++]); }';
 const unit=lower(source),query=queries(unit.functions[0])[0];
 assert.equal(query.operation,'unresolved C size query');
 assert.equal(query.operandEvaluation,'unproved');
 assert.equal(query.operand.arrays.length,1);
 assert.ok(unit.obligations.some(item=>item.category==='C sizeof array extent'));
 assert.ok(unit.obligations.some(item=>item.category==='C sizeof evaluation mode'));
 assert.equal(unit.functions[0].body.some(item=>['C call','store C lvalue','load C lvalue'].includes(item.operation)),false);
 const array={kind:'C array',element:{kind:'C integer',bits:32,signed:true},sizeSource:null};
 for(const declared of[array,{kind:'C pointer',to:array},{kind:'C qualified',qualifiers:['const'],type:{kind:'C pointer',to:array}},{kind:'C record',identity:'unproved:record'}]){
  const candidate=lower('size_t measure(void) { return sizeof(Alias); }',{types:{Alias:declared}});
  const unresolved=queries(candidate.functions[0])[0];
  assert.equal(unresolved.operation,'unresolved C size query');
  assert.equal(unresolved.operandEvaluation,'unproved');
  assert.equal(unresolved.operand.owner,candidate.loweringOptions.types.Alias);
  assert.equal(unresolved.byteSize,null);
  assert.equal(candidate.checkedPortableProgram,null);
 }
});

test('missing invalid conflicting and source-owned size_t bases are handled explicitly',()=>{
 const source='int width(void) { return sizeof(int); }';
 for(const candidateAbi of[{intBits:32,charSigned:true},{intBits:32,sizeBits:0,charSigned:true}]){
  const unit=lower(source,{abi:candidateAbi}),query=queries(unit.functions[0])[0];
  assert.equal(query.operation,'unresolved C size query');
  assert.equal(query.result.type.kind,'unresolved C type');
  assert.equal(query.byteSize,null);
  assert.ok(unit.obligations.some(item=>item.category==='C sizeof size type'));
 }
 const conflict=lower('typedef uint16_t size_t; int width(void) { return sizeof(int); }');
 assert.equal(queries(conflict.functions[0])[0].result.type.kind,'unresolved C type');
 assert.ok(conflict.obligations.some(item=>item.category==='C sizeof size type'&&item.detail.includes('conflicts')));
 const owned=lower('typedef uint64_t size_t; int width(void) { return sizeof(int); }');
 const query=queries(owned.functions[0])[0];
 assert.equal(query.operation,'C size query');
 assert.equal(query.result.type.bits,64);
 assert.equal(query.resultTypeBasis.declaration.source,owned.sourceCst.tree.children.find(node=>node.term==='type_definition'));
 assert.equal(query.byteSize,null);
 assert.ok(owned.obligations.some(item=>item.category==='C sizeof layout'));
 const declarationOnly=lower('typedef uint64_t size_t; int width(void) { return sizeof(int); }',{abi:{intBits:32,charSigned:true}});
 assert.equal(queries(declarationOnly.functions[0])[0].operation,'unresolved C size query');
 assert.ok(declarationOnly.obligations.some(item=>item.category==='C sizeof size type'));
});

test('size query source declaration ABI and numeric corruption fail complete archive replay',()=>{
 const unit=lower('typedef enum { FIRST, SECOND } Block; size_t extent(void) { return sizeof(Block); }');
 for(const mutate of[
  query=>{query.source=structuredClone(query.source);},
  query=>{query.operand.owner.source=structuredClone(query.operand.owner.source);},
  query=>{query.resultTypeBasis.abiOwner=structuredClone(query.resultTypeBasis.abiOwner);},
  query=>{query.byteSize='4';},
  query=>{query.operandEvaluation='unproved';},
  query=>{query.result.type.bits=32;},
 ]){
  const changed=structuredClone(unit);mutate(queries(changed.functions[0])[0]);
  const packet=archiveRestrictedCLowering(changed);
  assert.throws(()=>readRestrictedCLowering(packet.network,packet.root),TypeError);
 }
});

test('sizeof supplied CSTs require exactly one real operand field and unknown names remain unresolved',()=>{
 const source='int width(void) { return sizeof(int); }';
 for(const mode of['missing','both']){
  const parsed=parseProgrammingLanguage(source,'C'),pending=[parsed.tree];let query;
  while(pending.length){const node=pending.pop();if(['sizeof_expression','size_of_expression'].includes(node.term)){query=node;break;}pending.push(...(node.children??[]));}
  assert.ok(query);
  const operand=query.children.find(node=>node.field==='type'||node.field==='value');
  assert.ok(operand);
  if(mode==='missing')operand.field=null;
  else query.children.push({...structuredClone(operand),field:operand.field==='type'?'value':'type'});
  assert.throws(()=>lowerRestrictedCst(parsed,source,{abi}),/exactly one actual type or value CST field/u);
 }
 const unknown=lower('size_t width(void) { return sizeof(unknown()); }');
 const query=queries(unknown.functions[0])[0];
 assert.equal(query.operation,'unresolved C size query');
 assert.equal(query.operandEvaluation,'unproved');
 assert.ok(unknown.obligations.some(item=>item.category==='C sizeof operand type'));
 assert.equal(unknown.functions[0].body.some(item=>item.operation==='C call'),false);
 assert.equal(unknown.checkedPortableProgram,null);
});

test('sizeof namespace refinement retains the original CST and respects value shadowing and declaration order',()=>{
 const source='typedef int Alias; size_t measure(int Alias) { return sizeof(Alias); }';
 const parsed=parseProgrammingLanguage(source,'C'),unit=lowerRestrictedCst(parsed,source,{abi});
 const query=queries(unit.functions[0])[0];
 assert.deepEqual(unit.sourceCst,structuredClone(parsed),'lowering retains complete original CST data, independent of host wrapper prototypes');
 assert.equal(query.form,'expression');
 assert.equal(query.operand.owner,unit.functions[0].parameters[0]);
 assert.equal(query.operandEvaluation,'unevaluated');
 assert.equal(query.sourceForm,query.source.children.some(node=>node.field==='type')?'type':'expression');
 for(const text of[
  'size_t measure(void) { return sizeof(Later); } typedef int Later;',
  'typedef int Alias; typedef int Alias; size_t measure(void) { return sizeof(Alias); }',
 ]) {
  const unresolved=lower(text),actual=queries(unresolved.functions[0])[0];
  assert.equal(actual.operation,'unresolved C size query');
  assert.equal(actual.operandEvaluation,'unproved');
  assert.equal(actual.byteSize,null);
  assert.ok(unresolved.obligations.some(item=>['C sizeof operand type','C sizeof type visibility'].includes(item.category)));
  assert.equal(unresolved.checkedPortableProgram,null);
 }
});

test('sizeof callable refinements retain actual owners and refuse a function declared only afterward',()=>{
 const source='int effect(void) { return 7; } size_t measure(void) { return sizeof(effect()); }';
 const parsed=parseProgrammingLanguage(source,'C'),unit=lowerRestrictedCst(parsed,source,{abi});
 const fn=unit.functions.find(item=>item.name==='measure'),query=queries(fn)[0];
 assert.deepEqual(unit.sourceCst,structuredClone(parsed));
 assert.equal(query.form,'expression');
 assert.equal(query.operation,'C size query');
 assert.equal(query.operand.owner,unit.sourceCst.tree.children.find(node=>node.term==='function_definition'));
 assert.equal(query.operandEvaluation,'unevaluated');
 assert.equal(fn.body.some(item=>['C call','store C lvalue','load C lvalue'].includes(item.operation)),false);
 assert.ok(unit.obligations.some(item=>item.category==='C sizeof operand constraints'));
 const late=lower('size_t measure(void) { return sizeof(effect()); } int effect(void) { return 7; }');
 const unresolved=queries(late.functions.find(item=>item.name==='measure'))[0];
 assert.equal(unresolved.operation,'unresolved C size query');
 assert.equal(unresolved.operandEvaluation,'unproved');
 assert.ok(late.obligations.some(item=>item.category==='C sizeof operand type'));
 assert.equal(late.checkedPortableProgram,null);
 for(const mutate of[
  item=>{item.form='type';},
  item=>{item.sourceForm=item.sourceForm==='type'?'expression':'type';},
  item=>{item.refinement=item.refinement?structuredClone(item.refinement):{source:item.source,owner:item.operand.owner,rule:'forged refinement'};},
 ]) {
  const changed=structuredClone(unit),actual=queries(changed.functions.find(item=>item.name==='measure'))[0];
  mutate(actual);const packet=archiveRestrictedCLowering(changed);
  assert.throws(()=>readRestrictedCLowering(packet.network,packet.root),TypeError);
 }
});

test('size-query source publication reconstructs current files and reverses every original byte',()=>{
 const archive=readProjectData(new URL('./data/sizeof-source-publication.lino',import.meta.url));
 const sources=generateRestrictedCSizeQueryPublication(archive);
 assert.deepEqual(checkRestrictedCSizeQueryPublication(),sources.map(source=>source.path));
 assert.equal(sources[0].source.slice(0,-archive.moduleAppend.length).replace(archive.edits[0].after,archive.edits[0].before),archive.originals[0].source);
 assert.equal(sources[1].source.replace(archive.edits[1].after,archive.edits[1].before),archive.originals[1].source);
 assert.equal(sources[2].source,archive.addedTest.source);
 for(const mutate of[
  value=>{value.originals[0].source+='\n';},
  value=>{value.originals[1].sha256='0'.repeat(64);},
  value=>{value.edits[0].path='../outside.mjs';},
  value=>{value.edits[0].after=value.edits[0].after.replace('SizeQueryLowering','OtherLowering');},
  value=>{value.edits[1].before=value.originals[1].source;},
  value=>{value.edits[1].after+='\n';},
  value=>{value.moduleAppend+='\n';},
  value=>{value.addedTest.source+='\n';},
  value=>{value.addedTest.path='../outside.test.mjs';},
  value=>{value.published.pop();},
  value=>{value.originals={...value.originals,length:2};},
 ]){
  const changed=structuredClone(archive);mutate(changed);
  assert.throws(()=>generateRestrictedCSizeQueryPublication(changed),TypeError);
 }
 for(const altered of sources){
  assert.throws(()=>checkRestrictedCSizeQueryPublication({archive,readSource:path=>{
   const source=sources.find(source=>source.path===path);assert.ok(source);
   return Buffer.from(source.source+(path===altered.path?' ':''));
  }}),/source publication is stale/u);
 }
 // UTF-8 encoding alone would collapse a lone surrogate onto replacement
 // bytes. Even matching byte receipts cannot authenticate that source text.
 for(const surrogate of[String.fromCharCode(0xd800),String.fromCharCode(0xdc00)]) {
  for(const index of[0,2]) {
   const changed=structuredClone(archive),suffix='\n// '+surrogate+'\n';
   const source=sources[index].source+suffix;
   const pin={path:sources[index].path,bytes:Buffer.byteLength(source),sha256:createHash('sha256').update(source).digest('hex')};
   if(index===0)changed.moduleAppend+=suffix;
   else changed.addedTest={...pin,source};
   changed.published[index]=pin;
   assert.throws(()=>generateRestrictedCSizeQueryPublication(changed),TypeError);
  }
 }
});
