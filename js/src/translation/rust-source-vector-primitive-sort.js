// Genuine closed standard Vec::sort admission and exclusive caller writeback.
// All order/insertion behavior comes from the complete ordinary JS source.
import {unsupported,typeError} from './diagnostics.js';
import {sameType,rustFixedType} from './types.js';
import {selectRustExclusiveSourcePlace,recordRustExclusiveSourcePlaceWrite} from './rust-exclusive-source-calls.js';
import {specializePrimitiveVectorSort,lowerPrimitiveVectorSort} from './primitive-vector-sort-specialization.js';
const operation='rustSourcePrimitiveVectorSort',admissions=new WeakSet();
const fail=(reason,span)=>{throw unsupported('Rust source primitive vector sort',reason,span);};
function domain(element,owner){
 if(!owner||!Array.isArray(owner.path)||!Array.isArray(owner.arguments)||owner.arguments.length)return '';
 const name=owner.path.join('::');
 if(element.kind==='bool'&&owner.kind==='primitive'&&name==='bool')return 'bool';
 if(element.kind==='string'&&owner.kind==='library'&&['std::string::String','alloc::string::String'].includes(name))return 'String';
 if(element.kind==='fixed'&&owner.kind==='primitive'&&/^(?:u|i)(?:8|16|32|64|128|size)$/u.test(name)&&sameType(element,rustFixedType(name)))return name;
 return '';
}
export function readRustSourceVectorPrimitiveSort(parser,receiver,method,path){
 if(method.value!=='sort'||parser.exclusiveSourceModel!=='Rust exclusive source loans 1')return null;
 const origin=parser.origins.origin(receiver,path);if(origin.type?.kind!=='array')return null;
 const owner=origin.expandedSourceOwner??origin.sourceOwner,element=origin.type.element,selected=domain(element,owner?.arguments?.[0]),place=selectRustExclusiveSourcePlace(parser,receiver,path),exclusive=place?.slot.exclusiveMode==='exclusive'&&(origin.wrappers.length===0||origin.sourceExclusiveMode==='exclusive'&&origin.wrappers.length===1&&origin.wrappers[0]==='reference');
 if(!parser.origins.collecting&&(!origin.proven||origin.arraySourceModel!=='owned vector'||origin.sourceBorrowedPlace===true&&!exclusive))fail('sort requires its actual owned Vec source value',method);
 if(origin.wrappers.length&&!exclusive||owner?.kind!=='library'||!['std::vec::Vec','alloc::vec::Vec'].includes(owner.path.join('::'))||owner.arguments.length!==1||!selected||origin.arrayElementPosition&&(origin.arrayElementPosition.wrappers.length||origin.arrayElementPosition.arrayElementPosition||domain(origin.arrayElementPosition.type,origin.arrayElementPosition.owner)!==selected))fail('sort admits actual direct standard Vec bool, fixed integer or owned String elements with their complete standard Ord owner',method);
 if(!place)fail('sort requires its actual mutable lexical root and selected exclusive field place',method);
 const c=parser.cursor;c.expect('(','standard Vec sort');c.expect(')','zero-argument standard Vec sort');const span={start:receiver.span?.start??method.start,end:c.peek(-1).end},admission=Object.freeze({domain:selected,element:structuredClone(element),sourceRange:Object.freeze({...span})});admissions.add(admission);
 const node=parser.origins.mark({k:'math',op:operation,args:[{value:receiver,spread:false}],sourcePrimitiveSortAdmission:admission,span},{...place.field,wrappers:[],sourceLifetimes:[],sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLexicalBindingIdentity:null,sourceOwnedResultProof:true});
 return recordRustExclusiveSourcePlaceWrite(parser,place,node,path,span);
}
export function checkRustSourceVectorPrimitiveSort(checker,node,env,path,expected){
 if(node.op!==operation)return null;
 if(checker.language!=='Rust'||!admissions.has(node.sourcePrimitiveSortAdmission)||node.args.length!==1||node.args[0].spread)throw typeError('primitive vector sort requires its actual parsed standard method and exclusive source place admission',node.span);
 const value=checker.referenceValue(checker.expr(node.args[0].value,env,path,expected));
 if(value.type.kind!=='array'||value.type.reference===true||!sameType(value.type.element,node.sourcePrimitiveSortAdmission.element))throw typeError('primitive vector sort retains its actual admitted checked element domain',node.span);
 const namespace=checker.generatedPrefix+'primitiveVectorSort'+String(++checker.fresh),specialization=specializePrimitiveVectorSort(value.type.element,namespace);
 for(const [name,declaration]of specialization.library.declarations){if(checker.items.has(name)||checker.externals.has(name))throw typeError('primitive vector source specialization collides with an actual declaration',node.span);checker.items.set(name,declaration);}
 (checker.rustSourceVectorSortLibraries??=[]).push(...specialization.library.items);
 return lowerPrimitiveVectorSort(node,value,specialization,role=>checker.generatedPrefix+role+String(++checker.fresh));
}
