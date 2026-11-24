// Host archive validation; no metadata accessor or represented body executes.
export function assertPlainSourceReceipt(value){
 const active=new Set(),seen=new Set();let visits=0,bytes=0;
 function walk(item,depth){
  if(++visits>262144||depth>256)throw new RangeError('source receipts exceed their explicit structural budget');
  if(typeof item==='string'){bytes+=Buffer.byteLength(item);if(bytes>128*1024*1024)throw new RangeError('source receipts exceed their explicit byte budget');return;}
  if(item===null||typeof item==='boolean'||typeof item==='number'&&Number.isFinite(item))return;
  if(typeof item!=='object'||![Object.prototype,Array.prototype,null].includes(Object.getPrototypeOf(item))||active.has(item))throw new TypeError('source receipts require acyclic plain data');
  if(seen.has(item))return;active.add(item);const keys=Reflect.ownKeys(item);
  if(Array.isArray(item)&&(keys.length!==item.length+1||!Array.from({length:item.length},(_,index)=>String(index)).every(key=>Object.hasOwn(item,key))))throw new TypeError('source receipt arrays must retain every actual dense data element');
  for(const key of keys){const descriptor=Object.getOwnPropertyDescriptor(item,key);if(typeof key!=='string'||!descriptor||!Object.hasOwn(descriptor,'value'))throw new TypeError('source receipts cannot evaluate accessor or symbol metadata');if(Array.isArray(item)&&key==='length')continue;if(!descriptor.enumerable)throw new TypeError('source receipts cannot hide nonenumerable metadata');walk(descriptor.value,depth+1);}
  active.delete(item);seen.add(item);
 }
 walk(value,0);
}
