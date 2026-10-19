// Physical cache representations. Logical entries and cell accounting stay
// unchanged; all cached computations still return their complete results.
const RESULT_FIELDS = new Set(['end','state','children','dynamic','precedence','tail','ambiguous','cost','before','open']);
const PADDING_RUN = Symbol('padding run');

export function packLexicalCacheResults(results) {
  if (!Array.isArray(results) || Object.getPrototypeOf(results)!==Array.prototype || results.length < 16
    || !Object.getOwnPropertyDescriptor(results,'length').writable
    || Reflect.ownKeys(results).length !== results.length + 1) return results;
  for (let index=0;index<results.length;index++) {
    const descriptor=Object.getOwnPropertyDescriptor(results,index);
    if (!descriptor || !Object.hasOwn(descriptor,'value') || !descriptor.enumerable || !descriptor.writable || !descriptor.configurable
      || descriptor.value===null || typeof descriptor.value!=='object') return results;
  }
  const template=results[0],keys=Reflect.ownKeys(template);
  if (!keys.includes('end') || keys.some(key=>!RESULT_FIELDS.has(key))) return results;
  const fields=new Set(keys),orders=new Map(),templates=[],shapes=[];
  for (const result of results) {
    if (Object.getPrototypeOf(result)!==Object.prototype) return results;
    const names=Reflect.ownKeys(result);
    if (names.length!==keys.length || names.some(name=>!fields.has(name))) return results;
    for (const key of keys) {
      const descriptor=Object.getOwnPropertyDescriptor(result,key);
      if (!Object.hasOwn(descriptor,'value') || !descriptor.enumerable || !descriptor.writable || !descriptor.configurable
        || (key!=='end'&&!Object.is(result[key],template[key]))) return results;
    }
    if (!Number.isSafeInteger(result.end)) return results;
    const order=names.join('\u0000');
    if (!orders.has(order)){orders.set(order,templates.length);templates.push(result);}
    shapes.push(orders.get(order));
  }
  const ends=Float64Array.from(results,result=>result.end);
  return templates.length===1?{template,ends}:{templates,ends,shapes:Uint32Array.from(shapes)};
}

export function readLexicalCacheResults(stored) {
  return Array.isArray(stored)?stored:Array.from(stored.ends,(end,index)=>({...(stored.template??stored.templates[stored.shapes[index]]),end}));
}

function readPaddingLeaves() {
  const {start,length,pool}=this[PADDING_RUN];
  return Array.from({length},(_,index)=>pool.get(start+index)
    ?? {type:'token',kind:null,start:start+index,end:start+index+1,trivia:true});
}

function readPaddingStates() {
  const {length,state}=this[PADDING_RUN];
  return Array(length+1).fill(state);
}

export function packTriviaCacheResult(skipped,position,pool,limit) {
  if (skipped===null || typeof skipped!=='object' || Object.getPrototypeOf(skipped)!==Object.prototype) return skipped;
  const fields=['end','leaves','state','states'],names=Reflect.ownKeys(skipped);
  if (names.length!==fields.length || names.some((name,index)=>name!==fields[index])) return skipped;
  for (const field of fields) {
    const descriptor=Object.getOwnPropertyDescriptor(skipped,field);
    if (!Object.hasOwn(descriptor,'value') || !descriptor.enumerable || !descriptor.writable || !descriptor.configurable) return skipped;
  }
  const {leaves,states,state,end}=skipped;
  if (!Array.isArray(leaves) || !Array.isArray(states)
    || Object.getPrototypeOf(leaves)!==Array.prototype || Object.getPrototypeOf(states)!==Array.prototype
    || !Object.getOwnPropertyDescriptor(leaves,'length').writable || !Object.getOwnPropertyDescriptor(states,'length').writable) return skipped;
  const length=leaves.length;
  if (length<16 || end!==position+length || states.length!==length+1
    || Reflect.ownKeys(leaves).length!==length+1 || Reflect.ownKeys(states).length!==states.length+1) return skipped;
  for (const values of [leaves,states]) for (let index=0;index<values.length;index++) {
    const descriptor=Object.getOwnPropertyDescriptor(values,index);
    if (!descriptor || !Object.hasOwn(descriptor,'value') || !descriptor.enumerable || !descriptor.writable || !descriptor.configurable) return skipped;
  }
  if (!states.every(value=>Object.is(value,state))) return skipped;
  const keys=['type','kind','start','end','trivia'];
  for (let index=0;index<length;index++) {
    const leaf=leaves[index];
    if (leaf===null || typeof leaf!=='object' || Object.getPrototypeOf(leaf)!==Object.prototype) return skipped;
    const names=Reflect.ownKeys(leaf);
    if (names.length!==keys.length || names.some((name,index)=>name!==keys[index])) return skipped;
    for (const key of keys) {
      const descriptor=Object.getOwnPropertyDescriptor(leaf,key);
      if (!Object.hasOwn(descriptor,'value') || !descriptor.enumerable || !descriptor.writable || !descriptor.configurable) return skipped;
    }
    if (leaf.type!=='token' || leaf.kind!==null || leaf.trivia!==true
      || !Object.is(leaf.start,position+index) || leaf.end!==position+index+1) return skipped;
  }
  for (const leaf of leaves) if (!pool.has(leaf.start)&&pool.size<limit) pool.set(leaf.start,leaf);
  const packed={end};
  Object.defineProperty(packed,'leaves',{get:readPaddingLeaves,enumerable:true,configurable:true});
  packed.state=state;
  Object.defineProperty(packed,'states',{get:readPaddingStates,enumerable:true,configurable:true});
  Object.defineProperty(packed,PADDING_RUN,{value:{start:position,length,pool,state}});
  return packed;
}
