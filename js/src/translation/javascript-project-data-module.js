// The portable VM executes the actual selected project-data function bodies.
// Only operations listed in the qualified provider directory execute here.
import * as filesystem from 'node:fs';
import * as paths from 'node:path';
import * as urls from 'node:url';
import {types as nativeTypes} from 'node:util';
import {createHash} from 'node:crypto';
import {compileFunction} from 'node:vm';
import {readStructuredDataLino} from '../structured-data-links.js';
import {analyzeProgram} from '../program-representation.js';
import {captureJavaScriptProjectDataSource,checkJavaScriptProjectDataSource} from './javascript-project-data-source.js';
import {createJavaScriptErrorRealm} from './javascript-error-realm.js';
import {beginJavaScriptErrorModule} from './javascript-error-module.js';
import {readActiveRuntimeDataVersions} from './runtime-data-module.js';
const providerURL=new urls.URL('../project-data-files.js',import.meta.url),decoderURL=new urls.URL('../structured-data-links.js',import.meta.url),loaderURL=new urls.URL('../project-data-loader.js',import.meta.url);
const hash=text=>createHash('sha256').update(text).digest('hex');

export function readShippedJavaScriptProjectDataSource({entries=['readProjectData','resolveProjectDataPath']}={}){
 return captureJavaScriptProjectDataSource(filesystem.readFileSync(providerURL,'utf8'),{identifier:'js/src/project-data-files.js',entries});
}

function actualDeclaration(source,name){
 const program=analyzeProgram(source,'JavaScript'),binding=program.bindings.find(row=>row.scope==='scope:0'&&row.kind==='function'&&row.name===name);
 const node=binding&&program.sourceMappings.filter(row=>row.term==='function_declaration'&&row.start<=binding.declaration.start&&row.end>=binding.declaration.end).sort((a,b)=>(a.end-a.start)-(b.end-b.start))[0];
 if(!node)throw new TypeError('a protected decoder operation requires its actual complete source declaration');return source.slice(node.start,node.end);
}

/** Declaration creation never invokes a captured body. Its lexical references
 * are executed only after binding-directed compilation in the portable VM. */
function declarationValues(capture){const result=Object.create(null);for(const origin of capture.origins){const declaration=capture.source.slice(origin.start,origin.end);const value=compileFunction('"use strict";return ('+declaration+'\n);',[])();if(Function.prototype.toString.call(value)!==declaration)throw new TypeError('the engine declaration value must retain its exact original source');Object.defineProperty(result,origin.name,{value,enumerable:true});}return result;}

/** This is an explicit Node realm/provider, not a portable filesystem or a
 * translation of the shared Links decoder body. Identity is embedding-owned.
 * Every capability is checked against complete current sources and versions
 * before an effect. Original project-data functions are never host callbacks. */
export function prepareJavaScriptProjectDataModule(input,{realmIdentity,trace=false,runtimeLoader=false}={}){
 const capture=checkJavaScriptProjectDataSource(input);
 if(typeof realmIdentity!=='bigint'||realmIdentity<=0n||realmIdentity>18446744073709551615n)throw new TypeError('project data source requires an explicit actual Node realm identity');
 if(capture.identifier!=='js/src/project-data-files.js'||filesystem.readFileSync(providerURL,'utf8')!==capture.source)throw new TypeError('the project-data provider must retain its complete actual shipped source identity');
 const versions=readActiveRuntimeDataVersions(),decoderSource=filesystem.readFileSync(decoderURL,'utf8'),decoderDeclaration=actualDeclaration(decoderSource,'readStructuredDataLino'),loaderSource=runtimeLoader?filesystem.readFileSync(loaderURL,'utf8'):null;
 if(Function.prototype.toString.call(readStructuredDataLino)!==decoderDeclaration)throw new TypeError('the external Links decoder must retain its actual declaration owner');
 const builtins={URL:urls.URL,Buffer,JSON,structuredClone,TypeError,Error,String,Array,Object,Function,Symbol};
 if(globalThis.URL!==urls.URL)throw new TypeError('the source URL binding must be the actual protected Node URL constructor');
 const realm={objectPrototype:Object.prototype,toPrimitive:Symbol.toPrimitive,errors:{Error,TypeError,RangeError,ReferenceError,SyntaxError,URIError,EvalError},getPrimitive:function sourcePrimitiveGet(key){'use strict';return this[key];},assign:function sourceStrictAssign(key,value){'use strict';this[key]=value;return value;}};
 const embedding=createJavaScriptErrorRealm(capture,{realm,definitions:declarationValues(capture),identity:realmIdentity,deferModule:true});
 const events=[],capabilities=[],nativeImports={readFileSync:filesystem.readFileSync,existsSync:filesystem.existsSync,extname:paths.extname,fileURLToPath:urls.fileURLToPath,readStructuredDataLino};let traceCodeUnits=0;
 const protectedMethods=[{object:String.prototype,key:'slice',value:String.prototype.slice},{object:Array.prototype,key:'includes',value:Array.prototype.includes},{object:Buffer,key:'byteLength',value:Buffer.byteLength},{object:JSON,key:'parse',value:JSON.parse},{object:Function.prototype,key:Symbol.hasInstance,value:Function.prototype[Symbol.hasInstance]}];
 const owner='actual project data raw capabilities 1:'+capture.sourceSha256+':'+hash(decoderSource)+':'+JSON.stringify(versions);
 function verify(){
  if(filesystem.readFileSync(providerURL,'utf8')!==capture.source||filesystem.readFileSync(decoderURL,'utf8')!==decoderSource||JSON.stringify(readActiveRuntimeDataVersions())!==JSON.stringify(versions))throw new TypeError('a source capability cannot outlive its actual source/version qualification');
  if(loaderSource!==null&&filesystem.readFileSync(loaderURL,'utf8')!==loaderSource)throw new TypeError('the actual forwarding source module cannot change its captured declaration/import owners');
  for(const [name,value]of Object.entries(builtins))if(globalThis[name]!==value)throw new TypeError('a protected source global has changed its actual realm binding');
  for(const method of protectedMethods)if(Reflect.getOwnPropertyDescriptor(method.object,method.key)?.value!==method.value)throw new TypeError('a protected realm method has changed its actual operation owner');
  if(Object.hasOwn(urls.URL,Symbol.hasInstance)||Reflect.getPrototypeOf(urls.URL)!==Function.prototype||urls.URL.prototype!==urlPrototype||Reflect.getOwnPropertyDescriptor(urlPrototype,'href')?.get!==urlHref)throw new TypeError('URL construction and instance tests require the actual protected native prototype and default hasInstance path');
  if(filesystem.readFileSync!==nativeImports.readFileSync||filesystem.existsSync!==nativeImports.existsSync||paths.extname!==nativeImports.extname||urls.fileURLToPath!==nativeImports.fileURLToPath||Function.prototype.toString.call(readStructuredDataLino)!==decoderDeclaration)throw new TypeError('a native provider operation has changed its actual imported owner');
 }
 const urlPrototype=urls.URL.prototype,arrayPrototype=Array.prototype,bufferPrototype=Buffer.prototype,urlHref=Object.getOwnPropertyDescriptor(urlPrototype,'href')?.get;
 function checkURL(value){if(nativeTypes.isProxy(value)||Reflect.getPrototypeOf(value)!==urlPrototype||typeof urlHref!=='function')throw new TypeError('URL transport requires the actual protected native URL object');Reflect.apply(urlHref,value,[]);}
 function checkArray(value){if(nativeTypes.isProxy(value)||!Array.isArray(value)||Reflect.getPrototypeOf(value)!==arrayPrototype)throw new TypeError('array transport requires the actual protected native Array object');}
 function checkBuffer(value){if(nativeTypes.isProxy(value)||!Buffer.isBuffer(value)||Reflect.getPrototypeOf(value)!==bufferPrototype)throw new TypeError('byte transport requires the actual protected native Buffer object');}
 function encodeData(value,seen=new Set()){
  if(value===null||!['object','function'].includes(typeof value))return embedding.encode(value);
  if(typeof value==='function'||nativeTypes.isProxy(value))throw new TypeError('external data cannot introduce opaque callbacks or proxies');
  if(seen.has(value))return embedding.encode(value);seen.add(value);
  if(Buffer.isBuffer(value))return embedding.encodeOpaqueHost(value,'hostByteBufferObject',checkBuffer);
  if(Array.isArray(value)){embedding.encodeOpaqueHost(value,'hostArrayObject',checkArray);for(const key of Reflect.ownKeys(value)){const descriptor=Reflect.getOwnPropertyDescriptor(value,key);if(!descriptor||!Object.hasOwn(descriptor,'value'))throw new TypeError('native data arrays cannot hide accessors');encodeData(descriptor.value,seen);}return embedding.encode(value);}
  const prototype=Reflect.getPrototypeOf(value);if(prototype!==Object.prototype&&prototype!==null)throw new TypeError('decoded external data requires actual plain records');
  for(const key of Reflect.ownKeys(value)){const descriptor=Reflect.getOwnPropertyDescriptor(value,key);if(!descriptor||!Object.hasOwn(descriptor,'value'))throw new TypeError('decoded external data cannot hide accessors');encodeData(descriptor.value,seen);}return embedding.encode(value);
 }
 function register(source,callable,operation,invoke){
  capabilities.push(embedding.registerCapability(source,callable,{operation,owner,verify,invoke(receiver,args,semantic){
   if(trace){
    let added=0;for(const value of semantic)added+=value.$==='sourceText'?value.text.length:value.$==='text'?value.units.length:0;
    if(events.length>=16384||traceCodeUnits+added>67108864)embedding.refuseCapability('project-data capability trace exceeds its explicit event or 67108864-code-unit bound');traceCodeUnits+=added;events.push({source,operation,arguments:semantic});
   }
   const result=invoke(receiver,args,semantic);
   try{if(result instanceof urls.URL)embedding.encodeOpaqueHost(result,'hostURLObject',checkURL);else if(nativeTypes.isNativeError(result))embedding.encodeHostError(result);else encodeData(result);}
   catch(error){embedding.refuseCapability('actual native result exceeds its declared semantic transport: '+String(error?.message??error));}
   return result;
  }}));
 }
 for(const requirement of capture.capabilities){
  if(requirement.kind==='import'){
   const imported=nativeImports[requirement.imported],expected=requirement.imported==='readStructuredDataLino'?'./structured-data-links.js':['readFileSync','existsSync'].includes(requirement.imported)?'node:fs':requirement.imported==='extname'?'node:path':requirement.imported==='fileURLToPath'?'node:url':'';
   if(typeof imported!=='function'||requirement.specifier!==expected)throw new TypeError('the selected import lacks its actual admitted raw operation owner');
   register(requirement.source,imported,'import '+requirement.specifier+' '+requirement.imported,(_receiver,args)=>Reflect.apply(imported,undefined,args));
  }else if(requirement.kind==='global'){
   const value=builtins[requirement.name];if(value===undefined)throw new TypeError('the selected source global lacks an actual protected owner');
   if(requirement.name==='structuredClone')register(requirement.source,value,'actual structured clone',(_receiver,_args,semantic)=>Reflect.apply(value,undefined,semantic.map(value=>embedding.materializeData(value))));
   else capabilities.push(embedding.capabilityReference(requirement.source,value,{operation:'actual global '+requirement.name,owner}));
  }else if(requirement.kind==='construct'){
   const constructor=builtins[requirement.name];if(typeof constructor!=='function')throw new TypeError('the selected construction lacks actual native authority');
   const factory=function sourceNativeConstruction(){};register(requirement.source,factory,'construct actual '+requirement.name,(_receiver,args)=>Reflect.construct(constructor,args,constructor));
  }else if(requirement.kind==='syntax'&&requirement.operation==='array literal')register(requirement.source,function sourceArrayLiteral(){},'actual array literal allocation',(_receiver,args)=>Array.of(...args));
  else if(requirement.kind==='syntax'&&requirement.operation==='instanceof')register(requirement.source,function sourceInstanceOf(){},'actual protected instanceof',(_receiver,args,semantic)=>{const url=capabilities.find(row=>row.source==='global:URL');if(!url||semantic[1]?.$!=='object'||semantic[1].identity!==url.identity)throw new TypeError('this source profile requires its actual native URL instanceof owner');return args[0] instanceof urls.URL;});
  else throw new TypeError('the selected syntax operation lacks an actual capability provider');
 }
 // Primitive/property lookups return these exact callables. Register their
 // implementation frames independently of source reference spellings.
 const ancillary=[['String.slice',String.prototype.slice],['Array.includes',Array.prototype.includes],['Buffer.byteLength',Buffer.byteLength],['JSON.parse',JSON.parse]];
 for(const [name,callable]of ancillary)register('intrinsic:'+name,callable,'actual protected '+name,(receiver,args)=>Reflect.apply(callable,receiver,args));
 capabilities.push(...embedding.opaqueCapabilities());
 const bindings=capture.cells.map(cell=>({$:'errorModuleBindingValue',identity:cell.binding,value:embedding.encode({value:undefined})}));
 embedding.finishSourceCapabilities(capabilities,bindings);
 let loader=null;
 if(loaderSource!==null){
  const actual=captureJavaScriptProjectDataSource(loaderSource,{identifier:'js/src/project-data-loader.js',entries:['readRuntimeProjectData']}),references=[];
  for(const requirement of actual.capabilities){
   if(requirement.kind!=='import'||requirement.specifier!=='./project-data-files.js'||requirement.imported!=='readProjectData')throw new TypeError('the runtime forwarding unit needs its actual supplied project-data import binding');
   const declaration=capture.plan.functions.find(row=>row.name===requirement.imported),callee=declaration&&embedding.module.registry.owners.find(row=>row.source===declaration.identity);
   if(!callee||!capture.plan.exports.includes(declaration.identity))throw new TypeError('the forwarding import must select an actual exported provider callable');
   references.push(embedding.sourceCallableReference(requirement.source,{$:'object',identity:callee.identity,callable:true},{operation:'actual source import '+requirement.imported,owner:capture.identifier+':'+capture.sourceSha256}));
  }
  loader=embedding.attachSourceUnit(actual,declarationValues(actual),references);
 }
 return {capture,embedding,get loaderCompilation(){return loader;},qualification:{model:'actual project data source execution 1',versions,providerSourceSha256:capture.sourceSha256,decoderSourceSha256:hash(decoderSource),loaderSourceSha256:loader?.capture.sourceSha256??null,externalOperations:capabilities.map(row=>({source:row.source,operation:row.operation,owner:row.owner})),remainingBoundary:'raw Node filesystem/URL operations and shared Links decode execute only as qualified capabilities'},
  beginRuntimeRead(args){if(!loader)throw new TypeError('runtime reads require the actual captured forwarding source module');const origin=loader.capture.origins.find(row=>row.name==='readRuntimeProjectData');return beginJavaScriptErrorModule(loader,embedding.heap(),origin.identity,args);},
  begin(name,args){const origin=capture.origins.find(row=>row.name===name);if(!origin)throw new TypeError('a project data entry needs its actual selected exported source owner');return beginJavaScriptErrorModule(embedding.module,embedding.heap(),origin.identity,args);},
  encode(value){if(value instanceof urls.URL)return embedding.encodeOpaqueHost(value,'hostURLObject',checkURL);return encodeData(value);},
  events:()=>structuredClone(events)};
}
