// Structural type/term construction shared by ordinary immutable collection
// access and the separately proof-gated eager source loop transport.
import {array} from './types.js';
import {typeError} from './diagnostics.js';
import {readCollectionAccessResult} from './collection-access-rules.js';
export function constructStandardCollectionItems(object,operation){
 const form=readCollectionAccessResult(object.type.kind,operation);
 if(!['keys','values','entries'].includes(form))throw typeError('a collection snapshot requires actual standard keys, values or entries',object.span);
 const key=object.type.kind==='map'?object.type.key:object.type.element;
 const value=object.type.kind==='map'?object.type.value:object.type.element;
 const element=form==='keys'?key:form==='values'?value:{kind:'tuple',elements:[key,value]};
 return {k:'collectionItems',object,op:operation,type:array(element)};
}
