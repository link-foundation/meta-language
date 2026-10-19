import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {selfTranslate} from '../../../../js/src/self-translation.js';
import {readRustSourceItems} from '../../../../js/scripts/generate-private-output-test-model.mjs';
import {readProjectData} from '../../../../js/scripts/project-data.mjs';
import {encodeStructuredDataLinks} from '../../../../js/src/structured-data-links.js';
import {captureOrdinaryInversePlan,extendRecursiveInversePlan} from './plan.mjs';
import {reconstructOrdinaryOutcome} from './owned.mjs';
import {checkOwnedOutcomePreorder} from './preorder-rules.mjs';
import {checkOwnedOutcomeSourceTypes} from './source-type-rules.mjs';
import {deriveOwnedOutcomeTypeSchema,readOwnedOutcomeTypeColumns} from './source-types.mjs';
import {generateOwnedOutcomeSourceTypes} from './generate-source-types.mjs';

const check=(schema,row)=>checkOwnedOutcomeSourceTypes(row.types,row.selectors,row.offsets,row.children,
  schema.modes,schema.patterns,schema.patternOffsets,schema.patternTypes,schema.variantOwners);
const readFixture=()=>readProjectData(new URL('../../../../parity/fixtures/grammar-outcome-source-type-rules.lino',import.meta.url));

test('complete source type generation retains every source owner, original projection and executable loop',()=>{
  const {fixture,files}=generateOwnedOutcomeSourceTypes({check:true});
  assert.equal(fixture.cases.length,33);assert.equal(fixture.declarations.length,16);
  assert.equal(fixture.sourceSchema.sourceRoots.length,9);assert.equal(fixture.sourceSchema.sourcePins.length,13);
  assert.deepEqual(fixture.sourceSchema.sourceTypeReferences,fixture.ordinarySourceSchema.sourceTypeReferences);
  assert.notDeepEqual(fixture.sourceSchema.variantReferences,fixture.ordinarySourceSchema.variantReferences);
  for(const row of fixture.cases)assert.equal(check(fixture.schema,row),row.expected,row.name);
  assert.equal(fixture.projections.length,14);
  for(const row of fixture.projections){
    const count=row.types.length;
    assert.equal(check(fixture.sourceSchema,row),0,row.name);
    assert.equal(checkOwnedOutcomePreorder(row.offsets,row.children,count,row.children.length,count)[0],0,row.name);
  }
  const parsed=readRustSourceItems(Buffer.from(files.map(file=>file.source).join('\n')));
  for(const suffix of ['loop1','loop2','loop3','loop4','loop5','loop6','loop8']){
    const name='ml_check_owned_outcome_source_types_'+suffix;
    const owners=parsed.items.filter(item=>item.kind==='function_item'&&item.name===name);assert.equal(owners.length,1);
    const owner=parsed.text(owners[0].node);assert.match(owner,/\bloop\s*\{/u);
    assert.equal(owner.split(name+'(').length,2,'every complete lifted loop executes without a recursive self call');
  }
});

test('source type columns refuse every nonfinite channel and retain deep iterative resources',()=>{
  const fixture=readFixture(),row=fixture.cases.find(row=>row.name==='fixed fields');
  const columns=[row.types,row.selectors,row.offsets,row.children,fixture.schema.modes,fixture.schema.patterns,
    fixture.schema.patternOffsets,fixture.schema.patternTypes,fixture.schema.variantOwners];
  const expected=[-3,-4,-2,-6,-1,-1,-1,-1,-1];
  for(const value of [NaN,Infinity,-Infinity])for(let index=0;index<columns.length;index++){
    const altered=structuredClone(columns);altered[index][0]=value;
    assert.equal(checkOwnedOutcomeSourceTypes(...altered),expected[index],'nonfinite channel '+index);
  }
  const count=8192,offsets=Array.from({length:count+1},(_,index)=>Math.min(index,count-1));
  const children=Array.from({length:count-1},(_,index)=>index+2);
  const deep={types:Array(count).fill(7),selectors:Array(count).fill(4),offsets,children};deep.selectors[count-1]=0;
  assert.deepEqual(checkOwnedOutcomePreorder(offsets,children,count,count-1,count),[0,count,count-1,count]);
  assert.equal(check(fixture.schema,deep),0);
  assert.deepEqual(checkOwnedOutcomePreorder(offsets,children,count,count-1,count-1),[-5,0,0,0]);
  deep.types[count-1]=0;assert.equal(check(fixture.schema,deep),-6);
});

test('source type adapters authenticate captured plans before reading caller data and isolate schema copies',()=>{
  const ordinary=captureOrdinaryInversePlan(),recursive=extendRecursiveInversePlan(ordinary);
  let invoked=false;const forged={get plan(){invoked=true;throw new Error('must not invoke');}};
  assert.throws(()=>deriveOwnedOutcomeTypeSchema(forged));assert.equal(invoked,false);
  const schema=deriveOwnedOutcomeTypeSchema(recursive),altered=deriveOwnedOutcomeTypeSchema(recursive);
  altered.modes.fill(-1);altered.sourcePins.pop();assert.deepEqual(deriveOwnedOutcomeTypeSchema(recursive),schema);
  const value={ok:true,tree:{type:'token',kind:'identifier',start:0,end:1,text:'a'},ambiguities:[],rejection:null};
  const packet=encodeStructuredDataLinks(value);
  const model=reconstructOrdinaryOutcome(recursive,packet.network,packet.root,{usizeBits:64});
  const columns=readOwnedOutcomeTypeColumns(recursive,model,{usizeBits:64});assert.equal(check(columns.schema,columns),0);
  const hostile=structuredClone(model);Object.defineProperty(hostile,'nodes',{get(){invoked=true;throw new Error('must not invoke');}});
  assert.throws(()=>readOwnedOutcomeTypeColumns(recursive,hostile,{usizeBits:64}));assert.equal(invoked,false);
  const foreign=structuredClone(model);foreign.nodes[0].typeReference='foreign';
  assert.throws(()=>readOwnedOutcomeTypeColumns(recursive,foreign,{usizeBits:64}));
});

test('the complete source type algorithm translates without carried items and restores every source byte',()=>{
  const source=readFileSync(new URL('./source-type-rules.mjs',import.meta.url),'utf8');
  const translated=selfTranslate(source,'JavaScript','Rust');
  assert.equal(translated.items.filter(item=>item.status==='carried').length,0);
  assert.ok(translated.items.some(item=>item.status==='translated'));
  assert.equal(selfTranslate(translated.code,'Rust','JavaScript').code,source);
});
