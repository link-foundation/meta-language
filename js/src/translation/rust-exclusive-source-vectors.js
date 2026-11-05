// Actual source helper bodies over shared checked folds, tuples and standard
// Option constructors. No float slice bound or native Vec body is substituted.
import {unsupported,typeError} from './diagnostics.js';
import {UNIT,rustFixedType} from './types.js';
import {createRustStandardSumSourceOrigin} from './rust-standard-sum-source.js';
import {readRustStandardSumOwnershipFacts} from './rust-standard-sum-ownership.js';
import {readRustRecordSourceType} from './rust-records.js';
import {acceptRustPrimitiveFromSource} from './rust-primitive-from-rules.js';
import {readRustExclusiveSourceVectorMethodArity,acceptRustExclusiveSourceVectorReceiver} from './rust-exclusive-source-vector-rules.js';
import {attachRustVectorMutationInterface} from './rust-vector-mutation-interfaces.js';
const fail=(reason,span)=>{throw unsupported('Rust exclusive Vec source mutation',reason,span);};
const reference=name=>({k:'name',path:[name]}),project=(name,index)=>({k:'tupleIndex',object:reference(name),index}),tuple=items=>({k:'tuple',items});
export function readRustExclusiveSourceVectorMethod(parser,origin,method,path){
 if(parser.exclusiveSourceModel!=='Rust exclusive source loans 1'||origin.type?.kind!=='array')return null;
 const arity=readRustExclusiveSourceVectorMethodArity(method.value);if(arity<0)return null;
 const owner=origin.expandedSourceOwner??origin.sourceOwner;
 if(!parser.origins.collecting&&!acceptRustExclusiveSourceVectorReceiver(origin.type.kind,origin.arraySourceModel??'',origin.wrappers.length,origin.proven===true,owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1))fail('the actual receiver must retain its direct standard owned Vec origin',method);
 const usize=rustFixedType('usize'),parameter={...parser.origins.typeOrigin(usize,path),proven:true,copyEvidence:true,wrappers:[],sourceLifetimes:[],sourceOwner:{kind:'primitive',path:['usize'],arguments:[]}};
 return {method:method.value,parameters:arity===1?[parameter]:[]};
}
export function createRustExclusiveSourceVectorPacket(parser,place,selected,args,path,span){
 if(args.length!==selected.parameters.length)throw typeError('Vec mutation requires its actual complete argument list',span);
 const field=place.field,sourceOwner=field.expandedSourceOwner??field.sourceOwner,vectorType=readRustRecordSourceType(parser,field.type,field.path),elementType=readRustRecordSourceType(parser,field.type.element,field.path),element={...parser.origins.typeOrigin(elementType,[]),sourceOwner:structuredClone(sourceOwner.arguments[0]),wrappers:[],sourceLifetimes:[]};
 if(element.type.kind==='string'){element.textSourceModel='owned string';element.copyEvidence=false;}
 if(!parser.origins.collecting&&!['copy','owned'].includes(readRustStandardSumOwnershipFacts(parser.origins,element).profile))fail('Vec mutation needs its actual complete destructor-free owning element origin',span);
 if(selected.method==='truncate'){
  const value=args[0],expected=selected.parameters[0];if(value.k==='num'&&!value.type){value.type=structuredClone(expected.type);parser.origins.mark(value,expected);}
  const actual=parser.origins.origin(value,path),owner=actual.expandedSourceOwner??actual.sourceOwner;
  if(!parser.origins.collecting&&(!acceptRustPrimitiveFromSource(actual.type?.kind??'',String(actual.type?.bits??''),Boolean(actual.type?.signed),owner?.kind??'',owner?.path.join('::')??'',owner?.arguments.length??-1,actual.proven===true,actual.wrappers.length)||owner.path.join('::')!=='usize'))fail('truncate retains its actual direct usize operand without a floating-point index conversion',span);
 }
 const result=selected.method==='pop'?{...createRustStandardSumSourceOrigin(parser,'std::option::Option',[element],span),sourceOwnedResultProof:true}:{...parser.origins.typeOrigin(UNIT,path),proven:true,copyEvidence:true,sourceOwner:{kind:'unit',path:[],arguments:[]}},vector={...field,type:vectorType,path:[],wrappers:[],sourceLifetimes:[],sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLexicalBindingIdentity:null,sourceBorrowedPlace:false,sourceExclusiveParameter:false,sourceOwnedResultProof:true},parameters=[vector,...selected.parameters],helper=parser.freshProductBinding(),names=parameters.map(()=>parser.freshProductBinding()),empty={k:'array',element:elementType,items:[],span},packetType={kind:'tuple',elements:[result.type,vectorType]},packetOrigin={...parser.origins.typeOrigin(packetType,[]),fields:[result,vector],copyEvidence:false,proven:true};let body;
 if(selected.method==='clear')body=tuple([{k:'unit',span},empty]);
 else{
  const accumulator=parser.freshProductBinding(),item=parser.freshProductBinding(),ret=selected.method==='pop'?packetType:{kind:'tuple',elements:[rustFixedType('usize'),vectorType]};let initial,step;
  if(selected.method==='pop'){
   const sum=result.standardSumSource,prior=parser.freshProductBinding(),some=value=>({k:'app',fn:{k:'name',path:sum.constructorPaths.Some},args:[value],span});
   initial=tuple([{k:'app',fn:{k:'name',path:sum.constructorPaths.None},args:[],span},empty]);
   step={k:'match1',scrutinee:project(accumulator,0),cases:[{pattern:{k:'ctor',path:sum.constructorPaths.None,binds:[]},body:tuple([some(reference(item)),project(accumulator,1)])},{pattern:{k:'ctor',path:sum.constructorPaths.Some,binds:[prior]},body:tuple([some(reference(item)),{k:'array',element:elementType,items:[{value:project(accumulator,1),spread:true},{value:reference(prior),spread:false}],span}])}],span};
  }else{
   const number=value=>({k:'num',value,type:rustFixedType('usize'),span});initial=tuple([reference(names[1]),empty]);
   step={k:'if',cond:{k:'binary',op:'gt',left:project(accumulator,0),right:number('0'),span},then:tuple([{k:'binary',op:'sub',left:project(accumulator,0),right:number('1'),span},{k:'array',element:elementType,items:[{value:project(accumulator,1),spread:true},{value:reference(item),spread:false}],span}]),else:reference(accumulator),span};
  }
  const fold={k:'method',object:reference(names[0]),op:'reduce',args:[{k:'lambda',params:[{name:accumulator,type:ret},{name:item,type:elementType}],ret,captures:[],body:step,span},initial],span};
  if(selected.method==='pop')body=fold;else{const held=parser.freshProductBinding();body={k:'let',name:held,type:ret,value:fold,body:tuple([{k:'unit',span},project(held,1)]),span};}
 }
 const declaration=attachRustVectorMutationInterface(parser,{k:'fn',name:helper,generated:true,params:names.map((name,index)=>({name,type:readRustRecordSourceType(parser,parameters[index].type,parameters[index].path)})),ret:readRustRecordSourceType(parser,packetType,[]),body,span},selected.method,parameters,element,result),helpers=parser.exclusiveSourceVectorHelpers??=[];
 if(helpers.length>=65536)fail('actual Vec source helpers exceed their finite declaration budget',span);
 parser.origins.registerCatalogEntry(helper,{k:'fn',parameters,result:packetOrigin});helpers.push(declaration);
 return {callee:['crate',helper],parameters,result};
}
export function finishRustExclusiveSourceVectorHelpers(parser,program){
 const helpers=parser.exclusiveSourceVectorHelpers??[];if(!helpers.length)return program;
 return {...program,items:[...program.items,...helpers]};
}
