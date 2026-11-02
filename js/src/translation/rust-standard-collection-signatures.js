// Source interface conversion and literal transport use actual checked library
// types/constructors. These operations author AST; they do not run collections.
import {unsupported} from './diagnostics.js';
import {readRustStandardCollectionSourceOriginError,resolveRustStandardCollectionOwner} from './rust-standard-collection-rules.js';
const fail=(reason,span)=>{throw unsupported('Rust standard collection source interface',reason,span);};
function exactRustStandardCollectionFacet(value,fields){return value&&typeof value==='object'&&!Array.isArray(value)&&Object.keys(value).length===fields.length&&fields.every(field=>Object.hasOwn(value,field));}
function acceptRustStandardCollectionLibraryIdentity(value){return typeof value?.entryPath==='string'&&!value.entryPath.startsWith('/')&&!value.entryPath.includes('\\')&&value.entryPath.split('/').every(part=>part!==''&&part!=='.'&&part!=='..')&&value.entryPath.endsWith('/rust-standard-collection-value-packet.js')&&typeof value.sourceSha256==='string'&&/^[0-9a-f]{64}$/u.test(value.sourceSha256);}
/** Provenance binding only: full source graph derivation remains mandatory at
 * admission. This field never makes an unavailable body execute.
 * @param {object} entry @param {string} language */
export function checkRustStandardCollectionLibraryOwner(entry,language){
 const owner=entry.sourceStandardCollectionLibraryOwner;if(owner===undefined)return null;
 if(language!=='Rust'||!exactRustStandardCollectionFacet(owner,['model','entryPath','sourceSha256','declaration','role'])||owner.model!=='Rust standard collection projected library owner 1'||!acceptRustStandardCollectionLibraryIdentity(owner)||owner.declaration!==(entry.fullName??[...(entry.modulePath??[]),entry.name].join('.'))||!['state','packet','kernel'].includes(owner.role))fail('standard collection library owner must bind its actual checked declaration and source identity',entry.span);
 if(owner.role==='state'){
  const fields=['origin','root','keyDomain','valueDomain','pointerWidth','revision','entries'];if(entry.k!=='data'||entry.ctors?.length!==1||entry.ctors[0].name!=='rustStandardCollectionState'||entry.ctors[0].fields.length!==fields.length||entry.ctors[0].fields.some((field,index)=>field.name!==fields[index]))fail('the collection state owner retains its actual complete kernel datatype interface',entry.span);
 }else{
  const count=owner.role==='packet'?5:4;if(entry.k!=='fn'||!entry.body||entry.params?.length!==count||!['data','named'].includes(entry.params[0].type?.kind)||entry.params[1].type?.kind!=='string'||entry.params[2].type?.kind!=='array'||!['data','named','record'].includes(entry.params[3].type?.kind)||owner.role==='packet'&&entry.params[4].type?.kind!=='string'||!['data','named'].includes(entry.ret?.kind))fail('the collection callable owner retains its actual complete packet/kernel body interface',entry.span);
 }
 return {...owner};
}
/** Actual nominal carrier specialization persists in core types and Links.
 * The generic state's source owner binding is checked here; full projected
 * body correspondence is checked independently by source admission.
 * @param {object} data @param {string} language @param {Function} resolveType @param {Function} resolveData */
export function checkRustStandardCollectionCarrier(data,language,resolveType=value=>value,resolveData=()=>null){
 const facet=data.sourceStandardCollectionCarrier;if(facet===undefined)return null;
 if(language!=='Rust'||data.k!=='data'||!exactRustStandardCollectionFacet(facet,['model','origin','keyDomain','valueDomain','pointerWidth','concreteIdentity','kernelType','kernelLibrary'])||facet.model!=='Rust closed standard collections 1')fail('collection carrier requires its actual complete Rust specialization facet',data.span);
 const error=readRustStandardCollectionSourceOriginError(facet.origin,facet.keyDomain,facet.valueDomain,facet.pointerWidth);if(error)fail(error,data.span);
 if(facet.pointerWidth!==64)fail('this collection source carrier profile covers the actual 64-bit frontend interface; 32-bit kernel domains remain separate',data.span);
 const owner=resolveRustStandardCollectionOwner(facet.origin.resolvedOwner,true,false),domains=owner==='std::collections::HashMap'?[facet.keyDomain,facet.valueDomain]:[facet.keyDomain],identity=owner+'<'+domains.join(',')+'>@'+facet.pointerWidth,base='RustStandardCollection'+(owner.endsWith('HashMap')?'HashMap':'HashSet')+'_'+facet.keyDomain+'_'+facet.valueDomain+'_'+facet.pointerWidth;
 if(facet.concreteIdentity!==identity||facet.origin.typeIdentity!==identity||typeof data.name!=='string'||data.name!==base&&!new RegExp('^'+base+'_[1-9][0-9]*$','u').test(data.name))fail('collection nominal identity preserves its canonical owner key/value domains and target width',data.span);
 if(!exactRustStandardCollectionFacet(facet.kernelLibrary,['entryPath','sourceSha256','kernelDeclaration','packetDeclaration'])||!acceptRustStandardCollectionLibraryIdentity(facet.kernelLibrary)||typeof facet.kernelLibrary.kernelDeclaration!=='string'||!facet.kernelLibrary.kernelDeclaration||typeof facet.kernelLibrary.packetDeclaration!=='string'||!facet.kernelLibrary.packetDeclaration)fail('collection carrier retains its actual projected kernel library owner link',data.span);
 if(data.ctors?.length!==1||data.ctors[0].name!=='CollectionState'||data.ctors[0].fields.length!==1||data.ctors[0].fields[0].name!=='state')fail('collection specialization has exactly its actual one-field nominal carrier constructor',data.span);
 const kernelType=resolveType(facet.kernelType),field=resolveType(data.ctors[0].fields[0].type);if(kernelType?.kind!=='data'||field?.kind!=='data'||field.name!==kernelType.name||kernelType.name!==facet.kernelLibrary.kernelDeclaration)fail('collection carrier field retains the actual generic shared kernel state type',data.span);
 const kernel=resolveData(kernelType.name),binding=kernel&&checkRustStandardCollectionLibraryOwner(kernel,language);if(!binding||binding.role!=='state'||binding.entryPath!==facet.kernelLibrary.entryPath||binding.sourceSha256!==facet.kernelLibrary.sourceSha256||binding.declaration!==facet.kernelLibrary.kernelDeclaration)fail('collection specialization cannot substitute an unrelated generic-state declaration or source owner',data.span);
 return {...facet,origin:structuredClone(facet.origin),kernelType,kernelLibrary:{...facet.kernelLibrary}};
}
/** Prevent typed graph edits from exposing an internal carrier through ordinary
 * structural equality or Display, including actual nested datatype fields. */
export function checkRustStandardCollectionObservation(type,resolveData,operation,span){
 const pending=[type],seen=new Set();let count=0;
 while(pending.length){const current=pending.pop();if(!current)continue;if(++count>65536)fail('collection observation exceeds its actual complete type traversal budget',span);
  if(current.kind==='data'){if(seen.has(current.name))continue;seen.add(current.name);const entry=resolveData(current.name);if(!entry)continue;if(entry.sourceStandardCollectionCarrier)fail('standard collection '+operation+' requires its separate actual observation semantics',span);for(const constructor of entry.ctors??[])for(const field of constructor.fields)pending.push(field.type);}
  else if(current.kind==='array'||current.kind==='option'||current.kind==='set')pending.push(current.element);
  else if(current.kind==='tuple')pending.push(...current.elements);
  else if(current.kind==='record')pending.push(...current.fields.map(field=>field.type));
  else if(current.kind==='map')pending.push(current.key,current.value);
 }
}
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
