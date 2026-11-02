// Source interface conversion and literal transport use actual checked library
// types/constructors. These operations author AST; they do not run collections.
import {unsupported} from './diagnostics.js';
import {readRustStandardCollectionSourceOriginError} from './rust-standard-collection-rules.js';
const fail=(reason,span)=>{throw unsupported('Rust standard collection source interface',reason,span);};
export function projectRustStandardCollectionLibraryType(type){
 if(type.kind==='data')return {kind:'named',path:['crate',...type.name.split('.')]};
 if(type.kind==='array')return {...type,element:projectRustStandardCollectionLibraryType(type.element)};
 if(type.kind==='tuple')return {...type,elements:type.elements.map(projectRustStandardCollectionLibraryType)};
 if(type.kind==='option')return {...type,element:projectRustStandardCollectionLibraryType(type.element)};
 if(type.kind==='record')return {...type,fields:type.fields.map(field=>({...field,type:projectRustStandardCollectionLibraryType(field.type)}))};
 if(type.kind==='function')return {...type,params:type.params.map(projectRustStandardCollectionLibraryType),ret:projectRustStandardCollectionLibraryType(type.ret)};
 return {...type};
}
export function readRustStandardCollectionSourcePositionError(parser,actual,expected){
 const left=actual?.sourceCollectionTypeIdentity,right=expected?.sourceCollectionTypeIdentity;if(!left&&!right)return '';
 if(!left||!right||left!==right)return 'collection source positions preserve exact owner key and value domain identities';
 const row=parser.standardCollectionSource?.types.get(left);if(!row||!actual.proven||!expected.proven)return 'collection source positions require their actual concrete registered source interpretation';
 return readRustStandardCollectionSourceOriginError(row.origin,row.keyDomain,row.valueDomain,row.pointerWidth);
}
export function readRustStandardCollectionLibraryOrigin(parser,type,path=[]){
 const projected=projectRustStandardCollectionLibraryType(type),origin=parser.origins.typeOrigin(projected,path);
 return {...origin,proven:true,copyEvidence:['bool','fixed','float','int','nat','unit'].includes(type.kind),sourceOwner:type.kind==='data'?{kind:'declaration',path:type.name.split('.'),arguments:[]}:origin.sourceOwner};
}
export function encodeRustStandardCollectionSourceConstant(library,value,type,span,depth=0){
 if(depth>128)fail('collection source constants exceed their finite type depth',span);
 const encode=(value,type)=>encodeRustStandardCollectionSourceConstant(library,value,type,span,depth+1);
 if(type.kind==='bool'&&typeof value==='boolean')return {k:'bool',value,span};
 if(type.kind==='string'&&typeof value==='string')return {k:'str',value,span};
 if(['float','fixed','int','nat'].includes(type.kind)&&(typeof value==='number'||typeof value==='bigint'))return {k:'num',value:String(value),type:projectRustStandardCollectionLibraryType(type),span};
 if(type.kind==='unit'&&value===undefined)return {k:'unit',span};
 if(type.kind==='option')return value===undefined?{k:'none',absence:type.absence==='null'?'null':'undefined',span}:{k:'some',type:projectRustStandardCollectionLibraryType(type),value:encode(value,type.element),span};
 if(type.kind==='array'&&Array.isArray(value))return {k:'array',element:projectRustStandardCollectionLibraryType(type.element),items:value.map(item=>({value:encode(item,type.element),spread:false})),span};
 if(type.kind==='tuple'&&Array.isArray(value)&&value.length===type.elements.length)return {k:'tuple',items:value.map((item,index)=>encode(item,type.elements[index])),span};
 if(type.kind==='record'&&value&&typeof value==='object')return {k:'record',entries:type.fields.map(field=>({name:field.name,value:encode(value[field.name],field.type)})),span};
 if(type.kind==='data'&&value&&typeof value==='object'){
  const declaration=library.program.declarations.get(type.name),constructor=declaration?.ctors.find(row=>row.name===value.$);
  if(!constructor)fail('collection metadata must name an actual complete library data constructor',span);
  return {k:'app',fn:{k:'name',path:['crate',...type.name.split('.'),constructor.name],span},args:constructor.fields.map(field=>encode(value[field.name],field.type)),span};
 }
 fail('collection constant cannot be represented by its actual checked source type',span);
}
export function constructRustStandardCollectionSourceVariant(library,type,tag,values,span){
 if(type.kind!=='data')fail('collection values need their actual library datatype',span);
 const declaration=library.program.declarations.get(type.name),constructor=declaration?.ctors.find(row=>row.name===tag);if(!constructor)fail('collection values retain their actual checked variant owner',span);
 return {k:'app',fn:{k:'name',path:['crate',...type.name.split('.'),constructor.name],span},args:constructor.fields.map(field=>{if(!values.has(field.name))fail('collection variant retains every actual field',span);return values.get(field.name);}),span};
}
export function matchRustStandardCollectionSourceVariant(parser,library,type,expression,tag,matched,rejected,span){
 const declaration=type.kind==='data'?library.program.declarations.get(type.name):null;if(!declaration?.ctors)fail('collection decoding requires its actual complete library datatype',span);
 return {k:'match',scrutinees:[expression],rows:declaration.ctors.map(constructor=>{const names=constructor.fields.map(()=>parser.freshProductBinding()),fields=new Map(constructor.fields.map((field,index)=>[field.name,{expression:{k:'name',path:[names[index]],span},type:field.type}]));return {patterns:[{k:'ctor',path:['crate',...type.name.split('.'),constructor.name],args:names.map(name=>({k:'bind',name,span})),span}],body:constructor.name===tag?matched(fields):rejected(constructor.name,fields),span};}),span};
}
