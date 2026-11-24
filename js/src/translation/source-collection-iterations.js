import {unsupported} from './diagnostics.js';
import {constructStandardCollectionItems} from './collection-items.js';
import {acceptDisjointCollectionIterationScope,readDisjointCollectionIterationItemForm,readDisjointCollectionIterationSourceOperation,readDisjointCollectionIterationMarker,readDisjointCollectionSpreadSourceOperation,readDisjointCollectionSpreadMarker,acceptDisjointCollectionSpreadReceiver} from './disjoint-collection-iteration-rules.js';

export function readClosedCollectionIterationSourceScope(known,externals,literals,bindings,directory,tokens){return acceptDisjointCollectionIterationScope(known,externals,literals,bindings,directory,tokens.some(token=>token.value==='import'||token.value==='export'));}

export function wrapClosedCollectionIteration(parser,iterable){
 const value=iterable.object;
 if(!parser.closedCollectionIterationScope||!parser.referenceSemantics||value.k!=='method'||!readDisjointCollectionIterationItemForm('map',value.op)||value.args.length!==0)return iterable;
 return {k:'math',op:readDisjointCollectionIterationSourceOperation(),name:'closed standard collection iteration',args:[{value:iterable,spread:false}],span:iterable.span};
}
export function wrapClosedCollectionSpread(parser,iterable){
 const wrapped=wrapClosedCollectionIteration(parser,iterable);
 if(wrapped===iterable||!acceptDisjointCollectionSpreadReceiver(iterable.object.object.k))return iterable;
 return {...wrapped,op:readDisjointCollectionSpreadSourceOperation(),name:'closed standard collection spread'};
}

export function readClosedCollectionIterationOperand(node){
 if(node.args.length!==1||node.args[0].spread||node.args[0].value?.k!=='forOfIterable')throw unsupported('closed collection iteration','one actual eager for-of source operand is required',node.span);
 const iterable=node.args[0].value,value=iterable.object;
 if(value.k!=='method'||!readDisjointCollectionIterationItemForm('map',value.op)||value.args.length!==0)throw unsupported('closed collection iteration','only a direct zero-argument standard keys, values or entries source is admitted',node.span);
 if(node.op===readDisjointCollectionSpreadSourceOperation()&&!acceptDisjointCollectionSpreadReceiver(value.object.k))throw unsupported('closed collection spread','the eager source needs an actual owned name or field receiver',node.span);
 return {iterable,value};
}

export function checkClosedCollectionIteration(checker,node,env,path,expected){
 if(checker.language!=='JavaScript'||checker.checkedProjection)throw unsupported('closed collection iteration','private source operations cannot enter checked projections',node.span);
 const {value}=readClosedCollectionIterationOperand(node),receiver=checker.expr(value.object,env,path,undefined);
 if(!readDisjointCollectionIterationItemForm(receiver.type.kind,value.op))return checker.expr(node.args[0].value,env,path,expected);
 const snapshot=constructStandardCollectionItems(checker.referenceValue(receiver),value.op);
 if(receiver.type.reference!==true)return snapshot;
 const marker=node.op===readDisjointCollectionSpreadSourceOperation()?readDisjointCollectionSpreadMarker(checker.closedCollectionIterationMarkers.length,value.op):readDisjointCollectionIterationMarker(checker.closedCollectionIterationMarkers.length,value.op);
 checker.closedCollectionIterationMarkers.push({marker,operation:value.op});
 return {...snapshot,op:marker};
}
