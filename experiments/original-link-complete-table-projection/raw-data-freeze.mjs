// Retain the original enumerable-value freeze while avoiding repeated DAG walks.
import {types} from 'node:util';

const completelyFrozen=new WeakSet();
let objects=0,reused=0;

function freezeOriginalLinkRawValue(value) {
  if(!value||typeof value!=='object')return true;
  if(completelyFrozen.has(value)){reused++;return true;}
  objects++;
  // Proxies keep the original trap order. Accessors and every ancestor that
  // reaches them remain uncached, so repeated getters retain their effects.
  let reusable=!types.isProxy(value);
  if(reusable)for(const key of Object.keys(value)){
    const descriptor=Object.getOwnPropertyDescriptor(value,key);
    if(!Object.hasOwn(descriptor,'value'))reusable=false;
  }
  for(const child of Object.values(value))if(!freezeOriginalLinkRawValue(child))reusable=false;
  Object.freeze(value);
  if(reusable)completelyFrozen.add(value);
  return reusable;
}

/** Preserve object identities and freeze every original enumerable descendant. */
export function freezeOriginalLinkRawData(value) {
  freezeOriginalLinkRawValue(value);
  return value;
}

/** Diagnostic work counters only; no graph data or execution evidence is stored. */
export function readOriginalLinkRawFreezeWork() {
  return{objects,reused};
}
