// Complete source helper surface, shared with the native frontend by generation.
import {rustFixedType} from './types.js';
const reference=name=>({k:'name',path:[name]});
const project=(name,index)=>({k:'tupleIndex',object:reference(name),index});
const tuple=items=>({k:'tuple',items});

export function buildRustExclusiveSourceVectorHelperBody(operation,bindings,elementType,packetType,span){
 const names=bindings.parameters,empty={k:'array',element:elementType,items:[],span};
 let body;
 if(operation==='clear')body=tuple([{k:'unit',span},empty]);
 else{
  const {accumulator,item,prior,held,constructors}=bindings,ret=operation==='pop'?packetType:{kind:'tuple',elements:[rustFixedType('usize'),packetType.elements[1]]};let initial,step;
  if(operation==='pop'){
   const some=value=>({k:'app',fn:{k:'name',path:constructors.Some},args:[value],span});
   initial=tuple([{k:'app',fn:{k:'name',path:constructors.None},args:[],span},empty]);
   step={k:'match1',scrutinee:project(accumulator,0),cases:[{pattern:{k:'ctor',path:constructors.None,binds:[]},body:tuple([some(reference(item)),project(accumulator,1)])},{pattern:{k:'ctor',path:constructors.Some,binds:[prior]},body:tuple([some(reference(item)),{k:'array',element:elementType,items:[{value:project(accumulator,1),spread:true},{value:reference(prior),spread:false}],span}])}],span};
  }else{
   const number=value=>({k:'num',value,type:rustFixedType('usize'),span});initial=tuple([reference(names[1]),empty]);
   step={k:'if',cond:{k:'binary',op:'gt',left:project(accumulator,0),right:number('0'),span},then:tuple([{k:'binary',op:'sub',left:project(accumulator,0),right:number('1'),span},{k:'array',element:elementType,items:[{value:project(accumulator,1),spread:true},{value:reference(item),spread:false}],span}]),else:reference(accumulator),span};
  }
  const fold={k:'method',object:reference(names[0]),op:'reduce',args:[{k:'lambda',params:[{name:accumulator,type:ret},{name:item,type:elementType}],ret,captures:[],body:step,span},initial],span};
  if(operation==='pop')body=fold;else{body={k:'let',name:held,type:ret,value:fold,body:tuple([{k:'unit',span},project(held,1)]),span};}
 }
 return body;
}
