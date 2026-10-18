// PRIVATE executable plan controls. Native fixtures are plain tagged data here;
// this does not execute native visitors or replace any production codec.
import assert from 'node:assert/strict';
import {wellFormedString} from '../../../../js/src/structured-data-model.js';

const owned = name => ({kind:'owned',name});
const json = {kind:'json'};
const string = {kind:'string'};
const has = (object,key) => Object.hasOwn(object,key);
const text = value => {assert.equal(typeof value,'string');assert.ok(wellFormedString(value),'native UTF-8 string cannot contain an unpaired code unit');return value;};
const fields = (value,names,variant=false) => {
  assert.ok(value&&Object.getPrototypeOf(value)===Object.prototype,'native record control must be plain tagged data');
  assert.ok(Reflect.ownKeys(value).every(key=>typeof key==='string'&&(names.includes(key)||(variant&&key==='$variant'))),'unknown native control field');
};

function* orderedEntries(plan,rows,sourceFields,value,variant=false) {
  fields(value,sourceFields,variant);
  for(const row of rows) {
    if(row.literal) {yield {key:row.key,type:typeof row.value==='string'?string:{kind:'null'},value:row.value};continue;}
    if(row.flatten) {
      const leaf=value[row.native],shape=plan.shapes.find(shape=>shape.native==='LeafText');
      fields(leaf,['payload'],true);
      const selected=shape.variants.find(variant=>variant.name===leaf.$variant);
      assert.ok(selected,'unknown LeafText variant');text(leaf.payload);
      for(const entry of selected.flatten) yield {key:entry.key,type:entry.literal?{kind:'null'}:string,value:entry.literal?entry.value:leaf.payload};
      continue;
    }
    const present=has(value,row.native);
    if(row.presence==='some'&&!present) continue;
    if(row.presence==='nullable'&&!present) {yield {key:row.key,type:{kind:'null'},value:null};continue;}
    assert.ok(present,'native control lacks required '+row.native);
    if(row.presence==='true') {assert.equal(typeof value[row.native],'boolean');if(!value[row.native]) continue;}
    yield {key:row.key,type:['some','nullable'].includes(row.presence)?row.type.item:row.type,value:value[row.native]};
  }
}

function classify(plan,type,value) {
  if(type.kind==='Box') return classify(plan,type.item,value);
  if(type.kind==='Vec') {assert.ok(Array.isArray(value));return {kind:'array',identity:value,values:value,item:type.item};}
  if(type.kind==='owned') {
    if(type.name==='SyntaxAttributes') {
      assert.ok(value instanceof Map,'IndexMap control uses ordered entries, not a JS object that reorders numeric keys');
      return {kind:'object',identity:value,entries:(function*(){for(const [key,item] of value) yield {key:text(key),type:owned('OperationValue'),value:item};})()};
    }
    const shape=plan.shapes.find(shape=>shape.native===type.name);assert.ok(shape,'unknown owned declaration');
    if(shape.entries) return {kind:'object',identity:value,entries:orderedEntries(plan,shape.entries,shape.sourceFields,value)};
    const variant=shape.variants.find(variant=>variant.name===value?.$variant);assert.ok(variant,'unknown native variant');
    if(variant.entries) return {kind:'object',identity:value,entries:orderedEntries(plan,variant.entries,variant.sourceFields,value,true)};
    assert.equal(variant.transparent,0,'LeafText is flattened only inside its owning object');
    fields(value,['payload'],true);assert.ok(has(value,'payload'));
    return classify(plan,variant.payloadType,value.payload);
  }
  if(type.kind==='json') {
    if(value===null) return {kind:'null'};
    if(typeof value==='boolean') return {kind:'boolean',value};
    if(typeof value==='number') return {kind:'number',value};
    if(typeof value==='string') return {kind:'string',value:text(value)};
    if(Array.isArray(value)) return {kind:'array',identity:value,values:value,item:json};
    assert.ok(value&&Object.getPrototypeOf(value)===Object.prototype,'native JSON control must be representable plain data');
    return {kind:'object',identity:value,entries:(function*(){for(const key of Object.keys(value)) yield {key:text(key),type:json,value:value[key]};})()};
  }
  if(type.kind==='null') {assert.equal(value,null);return {kind:'null'};}
  if(type.kind==='string') return {kind:'string',value:text(value)};
  if(type.kind==='bool') {assert.equal(typeof value,'boolean');return {kind:'boolean',value};}
  if(type.kind==='usize'||type.kind==='i64') {
    assert.ok(typeof value==='bigint'||typeof value==='number','integer fixture must retain its original integer');
    if(typeof value==='number') assert.ok(Number.isSafeInteger(value),'use BigInt for large native integer controls');
    const integer=BigInt(value);
    if(type.kind==='i64') assert.ok(integer>=-(1n<<63n)&&integer<(1n<<63n));
    else assert.ok(integer>=0n,'usize cannot be negative');
    const represented=Number(integer);
    assert.ok(Number.isFinite(represented)&&BigInt(represented)===integer,'native integer is not exactly representable as binary64');
    return {kind:'number',value:represented};
  }
  throw new TypeError('unsupported control type '+type.kind);
}

/** Iterate ordered graph events without constructing a projected value tree.
 * Explicit JS control stack; the generated native draft remains recursive.
 */
export function* outcomeWireEvents(plan,value,type=owned('ParseOutcome'),options={}) {
  const maximumFrames=options.maximumFrames??1_000_000;
  assert.ok(Number.isSafeInteger(maximumFrames)&&maximumFrames>0);
  const stack=[],active=new Set();
  const push=frame=>{assert.ok(stack.length<maximumFrames,'control frame reservation exceeds resource limit');stack.push(frame);};
  push({kind:'value',type,value,depth:1});
  while(stack.length) {
    const frame=stack.pop(),depth=frame.depth;
    if(frame.kind==='end') {active.delete(frame.identity);yield {kind:frame.array?'endArray':'endObject',depth};continue;}
    if(frame.kind==='fields') {
      const next=frame.entries.next();
      if(next.done) continue;
      push(frame);
      yield {kind:'pair',depth:depth+1};
      yield {kind:'string',value:text(next.value.key),depth:depth+2,key:true};
      push({kind:'endPair',depth:depth+1});
      push({kind:'value',type:next.value.type,value:next.value.value,depth:depth+2});
      continue;
    }
    if(frame.kind==='items') {
      if(frame.index===frame.values.length) continue;
      assert.ok(has(frame.values,frame.index),'sparse native vector control');
      const value=frame.values[frame.index++];push(frame);
      push({kind:'value',type:frame.item,value,depth:depth+1});continue;
    }
    if(frame.kind==='endPair') {yield {kind:'endPair',depth};continue;}
    const result=classify(plan,frame.type,frame.value);
    if(result.kind!=='object'&&result.kind!=='array') {yield {...result,depth};continue;}
    assert.ok(!active.has(result.identity),'cyclic native control fixture');
    yield {kind:result.kind,depth}; // Budget sink sees this BEFORE child frames.
    active.add(result.identity);
    push({kind:'end',identity:result.identity,array:result.kind==='array',depth});
    if(result.kind==='array') push({kind:'items',values:result.values,item:result.item,index:0,depth});
    else push({kind:'fields',entries:result.entries,depth});
  }
  yield {kind:'root',depth:0}; // Existing Encoder inserts schema root last.
}

/** Count EXPANDED graph occurrences and scalar bytes; no scalar interner/graph.
 * This contract applies only to the proposed NEW API, not existing encoders.
 */
export function preflightOutcome(plan,value,limits,type=owned('ParseOutcome')) {
  for(const key of ['nodeLimit','depthLimit','byteLimit']) assert.ok(Number.isSafeInteger(limits[key])&&limits[key]>0,'positive explicit limits required');
  let occurrences=0,scalarBytes=0,maximumDepth=0,references=0;
  const add=(left,right)=>{assert.ok(right<=Number.MAX_SAFE_INTEGER-left,'resource counter overflow');return left+right;};
  for(const event of outcomeWireEvents(plan,value,type,{maximumFrames:limits.nodeLimit})) {
    if(['endArray','endObject','endPair'].includes(event.kind)) continue;
    occurrences=add(occurrences,1);assert.ok(occurrences<=limits.nodeLimit,'expanded occurrence resource limit');
    maximumDepth=Math.max(maximumDepth,event.depth);assert.ok(event.depth<=limits.depthLimit,'graph depth resource limit');
    let bytes=0;
    if(event.kind==='string') bytes=Buffer.byteLength(event.value,'utf8');
    else if(event.kind==='boolean') bytes=event.value?4:5;
    else if(event.kind==='number') {
      assert.ok(Number.isFinite(event.value),'finite binary64 required');
      // JS itself supplies its Number spelling for this private control. Native
      // MUST call the existing formatter, with its bounded scratch reservation.
      bytes=Buffer.byteLength(Object.is(event.value,-0)?'-0':String(event.value));
    } else if(event.kind==='root') bytes=1;
    scalarBytes=add(scalarBytes,bytes);assert.ok(scalarBytes<=limits.byteLimit,'expanded scalar byte resource limit');
    references=add(references,event.kind==='pair'?2:event.kind==='root'?1:0);
    // Array edges: count each child when read; object edges: each pair.
    // Total edges of this expanded tree are occurrences-1, independently of
    // whether scalar encoding later interns actual graph nodes.
  }
  assert.ok(references<=occurrences-1);
  return {occurrences,scalarBytes,maximumDepth,references:occurrences-1};
}
