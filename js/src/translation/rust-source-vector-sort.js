// A deliberately closed source comparator admission. Actual String ownership,
// nominal fields and the exclusive place are checked before a shared source
// algorithm replaces the comparator. Arbitrary closures remain unsupported.
import {unsupported,typeError} from './diagnostics.js';
import {STRING,sameType} from './types.js';
import {selectRustExclusiveSourcePlace,recordRustExclusiveSourcePlaceWrite} from './rust-exclusive-source-calls.js';
import {readRustOwnedRecordStateLayout} from './rust-owned-record-state.js';
import {requireRustNonconstantBinding} from './rust-source-constants.js';
import {readRustRecordSourceType} from './rust-records.js';
import {specializeStableVectorSort,lowerStableVectorSort} from './stable-vector-sort-specialization.js';
const operation='rustSourceStableUtf16VectorSort',admissions=new WeakSet();
const fail=(reason,span)=>{throw unsupported('Rust source stable UTF16 vector sort',reason,span);};
export function readRustSourceVectorSort(parser,receiver,method,path){
 if(method.value!=='sort_by'||parser.exclusiveSourceModel!=='Rust exclusive source loans 1')return null;
 const origin=parser.origins.origin(receiver,path);if(origin.type?.kind!=='array')return null;
 const owner=origin.expandedSourceOwner??origin.sourceOwner;
 if(owner?.kind!=='library'||owner.path.join('::')!=='std::vec::Vec'||owner.arguments.length!==1||origin.wrappers.length)fail('sorting requires its actual direct standard owned Vec owner',method);
 if(origin.arrayElementPosition?.wrappers?.length||owner.arguments[0]?.kind==='library'&&['std::boxed::Box','alloc::boxed::Box'].includes(owner.arguments[0].path.join('::')))fail('this nominal field sort requires an actual direct element without erased reference or Box layers',method);
 const place=selectRustExclusiveSourcePlace(parser,receiver,path);if(!place)fail('sorting requires an actual mutable lexical root and selected field place',method);
 const element={...parser.origins.typeOrigin(origin.type.element,origin.path),sourceOwner:structuredClone(owner.arguments[0])},layout=readRustOwnedRecordStateLayout(parser,element);
 if(!layout)fail('this source comparator requires its actual complete nominal record element',method);
 const c=parser.cursor;c.expect('(','sort_by');c.expect('|','sort_by immutable comparator');
 const first=c.identifier('first comparator binding');requireRustNonconstantBinding(parser,first.value,path,first);c.expect(',','sort_by comparator');
 const second=c.identifier('second comparator binding');requireRustNonconstantBinding(parser,second.value,path,second);if(first.value===second.value)throw typeError('a comparator cannot bind the same name twice',second);
 c.expect('|','sort_by comparator');const block=Boolean(c.eat('{'));
 const projection=binder=>{const actual=c.identifier('comparator operand');if(actual.value!==binder.value)fail('the comparator must project each actual immutable parameter in order',actual);c.expect('.','comparator field');const field=c.identifier('comparator field');c.expect('.','UTF16 iterator');c.expect('encode_utf16','actual String UTF16 iterator');c.expect('(','UTF16 iterator');c.expect(')','UTF16 iterator');return field;};
 const left=projection(first);c.expect('.','iterator comparison');c.expect('cmp','actual UTF16 iterator lexicographic comparison');c.expect('(','iterator comparison');const right=projection(second);c.expect(')','iterator comparison');
 if(block)c.expect('}','sort_by comparator result');c.eat(',');c.expect(')','sort_by');
 if(left.value!==right.value)fail('both comparator operands must project the same actual nominal field',right);
 const index=layout.layout.names.indexOf(left.value),field=layout.entry.fields[index],fieldOwner=field?.expandedSourceOwner??field?.sourceOwner;
 if(index<0||field?.type?.kind!=='string'||field.wrappers.length||field.textSourceModel!=='owned string'||fieldOwner?.kind!=='library'||fieldOwner.path.join('::')!=='std::string::String')fail('the selected comparator field must be its actual direct owned standard String',left);
 const span={start:receiver.span?.start??method.start,end:c.peek(-1).end},binder=parser.freshProductBinding(),selectedField=parser.freshProductBinding(),read={k:'name',path:[binder],span},key={k:'match1',scrutinee:read,cases:[{pattern:{k:'ctor',path:['crate',...layout.layout.constructor.split('.')],binds:layout.entry.fields.map((_,at)=>at===index?selectedField:null)},body:{k:'name',path:[selectedField],span}}],span};
 const admission=Object.freeze({field:index,constructor:layout.layout.constructor,sourceRange:Object.freeze({...span}),binders:Object.freeze([first,second].map(token=>Object.freeze({name:token.value,start:token.start,end:token.end})))});admissions.add(admission);
 const lambda={k:'lambda',params:[{name:binder,type:readRustRecordSourceType(parser,element.type,element.path)}],ret:STRING,captures:[],body:key,span};
 const node=parser.origins.mark({k:'math',op:operation,args:[{value:receiver,spread:false},{value:lambda,spread:false}],sourceSortAdmission:admission,span},{...place.field,wrappers:[],sourceLifetimes:[],sourceBindingIdentity:null,sourcePlaceBindingIdentity:null,sourceLexicalBindingIdentity:null,sourceOwnedResultProof:true});
 return recordRustExclusiveSourcePlaceWrite(parser,place,node,path,span);
}
export function checkRustSourceVectorSort(checker,node,env,path,expected){
 if(node.op!==operation)return null;
 if(checker.language!=='Rust'||!admissions.has(node.sourceSortAdmission)||node.args.length!==2||node.args.some(argument=>argument.spread))throw typeError('stable vector sorting requires its actual parsed source comparator admission',node.span);
 const value=checker.referenceValue(checker.expr(node.args[0].value,env,path,expected));
 if(value.type.kind!=='array'||value.type.reference===true)throw typeError('stable vector sorting requires its actual owned array carrier',node.span);
 const key=checker.expr(node.args[1].value,env,path,{kind:'function',params:[value.type.element],ret:STRING});
 if(key.type.kind!=='function'||key.type.params.length!==1||!sameType(key.type.params[0],value.type.element)||!sameType(key.type.ret,STRING))throw typeError('stable vector sorting retains its actual immutable field projection',node.span);
 const admission=node.sourceSortAdmission,body=key.body,selected=body?.cases?.[0],pattern=selected?.pattern;
 if(key.k!=='lambda'||key.params.length!==1||value.type.element.kind!=='data'||body?.k!=='match'||body.scrutinee?.k!=='var'||body.scrutinee.name!==key.params[0].name||body.cases.length!==1||pattern?.k!=='ctor'||pattern.data!==value.type.element.name||pattern.ctor!==admission.constructor.split('.').at(-1)||selected.body?.k!=='var'||selected.body.name!==pattern.binds?.[admission.field])throw typeError('stable vector sorting retains only its actual admitted nominal field projection body',node.span);
 const namespace=checker.generatedPrefix+'stableVectorSort'+String(++checker.fresh),specialization=specializeStableVectorSort(value.type.element,namespace);
 for(const [name,declaration]of specialization.library.declarations){if(checker.items.has(name)||checker.externals.has(name))throw typeError('stable vector source specialization collides with an actual declaration',node.span);checker.items.set(name,declaration);}
 (checker.rustSourceVectorSortLibraries??=[]).push(...specialization.library.items);
 return lowerStableVectorSort(node,value,key,specialization,role=>checker.generatedPrefix+role+String(++checker.fresh));
}
