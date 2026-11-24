// Qualified host IO bridge. Only the explicitly captured shipped runtime-data
// adapter executes outside the portable VM. No other source body is a fallback.
import {readFileSync,existsSync} from 'node:fs';
import {dirname,join} from 'node:path';
import {fileURLToPath,URL as NativeURL} from 'node:url';
import {readRuntimeProjectData} from '../project-data-loader.js';
import {readProjectData} from '../project-data-files.js';
import {captureRuntimeDataAdapterSource,checkJavaScriptRuntimeDataModule} from './runtime-data-module-source.js';
import {readJavaScriptRuntimeDataVersionError} from './javascript-module-initialization-rules.js';
import {createJavaScriptModuleInitialization,stepJavaScriptModuleInitialization,resumeJavaScriptModuleInitialization,readJavaScriptModuleBinding} from './javascript-module-initialization.js';
import {captureJavaScriptRuntimeDataFunctions} from './javascript-error-classes.js';
import {createJavaScriptErrorRealm} from './javascript-error-realm.js';
import {beginJavaScriptErrorModule} from './javascript-error-module.js';
import {readShippedJavaScriptProjectDataSource,prepareJavaScriptProjectDataModule} from './javascript-project-data-module.js';
import {runJavaScriptSourceTransition} from '../node-source-modules.js';
const scopes=new WeakMap(),sourceReaders=new WeakMap(),defaultScope=Object.freeze({});scopes.set(defaultScope,new WeakMap());
const loaderURL=new NativeURL('../project-data-loader.js',import.meta.url),delegateURL=new NativeURL('../project-data-files.js',import.meta.url);

export function readActiveRuntimeDataVersions(){
 const [nodeMajor,nodeMinor]=process.versions.node.split('.').map(Number);let directory=dirname(fileURLToPath(import.meta.resolve('links-notation'))),linksVersion='';
 for(let depth=0;depth<8;depth++){const file=join(directory,'package.json');if(existsSync(file)){const value=JSON.parse(readFileSync(file,'utf8'));if(value.name==='links-notation'){linksVersion=value.version;break;}}const parent=dirname(directory);if(parent===directory)break;directory=parent;}
 const error=readJavaScriptRuntimeDataVersionError(nodeMajor,nodeMinor,linksVersion);if(error)throw new TypeError(error+'; active Node '+process.versions.node+', links-notation '+(linksVersion||'unidentified'));
 return {nodeMajor,nodeMinor,linksVersion};
}

export function readShippedRuntimeDataAdapterContract(){
 return captureRuntimeDataAdapterSource({loaderPath:'js/src/project-data-loader.js',loaderSource:readFileSync(loaderURL,'utf8'),delegatePath:'js/src/project-data-files.js',delegateSource:readFileSync(delegateURL,'utf8'),...readActiveRuntimeDataVersions()});
}

/** A scope is one host embedding/module-cache context, never an archived
 * certificate. Callers may create separate contexts explicitly. */
export function createRuntimeDataModuleScope(){const scope=Object.freeze({});scopes.set(scope,new WeakMap());return scope;}

export function prepareJavaScriptRuntimeDataModule(capture,authority,{moduleURL,scope=defaultScope,trace=false,readerProfile='qualified runtime Links data IO adapter 1',readerRealmIdentity}={}){
 if(!['qualified runtime Links data IO adapter 1','actual project data source functions 1'].includes(readerProfile))throw new TypeError('a runtime data reader needs its explicit predecessor or actual source-body profile');
 const actual=checkJavaScriptRuntimeDataModule(capture),sourceFunctions=captureJavaScriptRuntimeDataFunctions(actual),url=new NativeURL(moduleURL),realmIdentity=authority.identity,realmKey=authority.realm?.objectPrototype,registry=scopes.get(scope);
 if(!registry||!realmKey||typeof realmIdentity!=='bigint'||realmIdentity<=0n||realmIdentity>18446744073709551615n||url.protocol!=='file:')throw new TypeError('runtime data initialization needs an actual file module, explicit realm identity and registered host scope');
 let realm=registry.get(realmKey);if(realm&&realm.identity!==realmIdentity)throw new TypeError('the same actual realm cannot change its semantic identity');if(!realm){realm={identity:realmIdentity,instances:new Map()};registry.set(realmKey,realm);}
 const previous=realm.instances.get(url.href);
 if(previous){
  if(readerProfile==='actual project data source functions 1'&&previous.readerRealmIdentity!==(readerRealmIdentity??(realmKey===Object.prototype?realmIdentity:undefined)))throw new TypeError('an initialized source reader cannot change its actual provider realm identity');
  if(previous.readerProfile!==readerProfile||JSON.stringify(previous.capture)!==JSON.stringify(actual)||previous.definitions.some(row=>Reflect.getOwnPropertyDescriptor(authority.definitions,row.name)?.value!==row.value))throw new TypeError('the actual module instance cannot change its captured source, execution profile or declaration bindings');return previous.api;
 }
 const current=readShippedRuntimeDataAdapterContract();
 if(JSON.stringify(current)!==JSON.stringify(actual.readerContract)||Function.prototype.toString.call(readRuntimeProjectData)!==current.loader.declaration.source||Function.prototype.toString.call(readProjectData)!==current.delegate.declaration.source)throw new TypeError('the IO operation must retain its exact actual shipped adapter/delegation source and active versions');
 if(globalThis.URL!==NativeURL||authority.realm.URL!==NativeURL||actual.readerImport&&new NativeURL(actual.readerImport.specifier,url).href!==loaderURL.href)throw new TypeError('the source URL constructor and imported reader must retain their actual qualified native owners');
 const embedding=createJavaScriptErrorRealm(sourceFunctions,{...authority,identity:realmIdentity,deferModule:true});
 let sourceReader=null;
 if(readerProfile==='actual project data source functions 1'){
  const identity=readerRealmIdentity??(realmKey===Object.prototype?realmIdentity:undefined);
  if(typeof identity!=='bigint'||identity<=0n||identity>18446744073709551615n||(realmKey!==Object.prototype&&identity===realmIdentity))throw new TypeError('the actual Node provider realm needs its explicit distinct identity when the importer belongs to another realm');
  let reader=sourceReaders.get(scope);if(reader&&reader.identity!==identity)throw new TypeError('the same actual Node provider realm cannot change its semantic identity inside a module scope');
  if(!reader){reader={identity,instance:prepareJavaScriptProjectDataModule(readShippedJavaScriptProjectDataSource(),{realmIdentity:identity,trace,runtimeLoader:true})};sourceReaders.set(scope,reader);}
  sourceReader=reader.instance;
 }
 let state=createJavaScriptModuleInitialization(actual.plan,url.href,realmIdentity),active=false;const history=[],requests=[];
 function record(operation,transition,value=null,abrupt=false){if(!trace)return;if(history.length>=16384)throw new RangeError('module control tracing exceeds its explicit event budget');history.push({operation,transition,value,abrupt});}
 const definitions=sourceFunctions.plan.functions.map(row=>({name:row.name,value:Reflect.getOwnPropertyDescriptor(authority.definitions,row.name)?.value}));
 function initialize(){
  if(active)throw new TypeError('an initializing module cannot be reentered through an unmodeled external callback');
  if(state.phase==='evaluated'||state.phase==='errored'){const transition=stepJavaScriptModuleInitialization(actual.plan,state);record('step',transition);return transition;}
  active=true;
  try{for(let step=0;step<8192;step++){
   const transition=stepJavaScriptModuleInitialization(actual.plan,state);state=transition.state;record('step',transition);
   if(transition.$==='moduleContinue')continue;
   if(transition.$==='moduleEvaluated'){embedding.finishModuleInitialization(state);return transition;}
   if(transition.$==='moduleReadData'){
    if(transition.readerOwner!==current.identity||state.moduleURL!==url.href||state.realm!==realmIdentity||globalThis.URL!==NativeURL||authority.realm.URL!==NativeURL)throw new TypeError('pending IO must retain its actual module, reader and native URL owners');
    if(requests.length>=4096)throw new RangeError('actual module IO requests exceed their source initializer budget');requests.push({moduleURL:url.href,realm:String(realmIdentity),binding:transition.binding,readerOwner:transition.readerOwner,specifier:transition.specifier});
    let value,loaded,abrupt=false;
    if(sourceReader){
      const reader=sourceReader.embedding,file=sourceReader.encode(new NativeURL(transition.specifier,url));
      const execution=runJavaScriptSourceTransition(reader,sourceReader.beginRuntimeRead([file]),{steps:1048576}).transition;
      if(execution.$==='machineThrow'){value=embedding.encodeHostError(reader.decode(execution.value));abrupt=true;}
      else if(execution.$!=='machineCompleted')throw new TypeError('the actual source reader must finish within its explicit transition budget: '+execution.$);
      // Structured policy data crosses this qualified module bridge as an
      // alias-preserving graph. The caller's const aliases still share ONE
      // initialized semantic handle; module cache cells stay in the reader VM.
      else loaded=reader.materializeData(execution.value);
    }else{try{loaded=readRuntimeProjectData(new NativeURL(transition.specifier,url));}catch(error){value=embedding.encodeHostError(error);abrupt=true;}}
    // Transport/resource failures are explicit frontend boundaries, not
    // exceptions fabricated as if the genuine source loader had thrown them.
    if(!abrupt)value=embedding.encode(loaded);
    const resumed=resumeJavaScriptModuleInitialization(actual.plan,state,value,abrupt);state=resumed.state;record('resume',resumed,value,abrupt);if(resumed.$==='moduleContinue')continue;return resumed;
   }
   if(transition.$==='moduleTemporalDeadZone'){
    if(typeof authority.realm.raiseUninitializedBinding!=='function')throw new TypeError('an uninitialized lexical read requires actual realm ReferenceError authority');
    let thrown,abrupt=false;try{Reflect.apply(authority.realm.raiseUninitializedBinding,undefined,[transition.name]);}catch(error){thrown=error;abrupt=true;}
    if(!abrupt||!thrown||Object.getPrototypeOf(thrown)!==authority.realm.errors.ReferenceError.prototype)throw new TypeError('the actual realm uninitialized-binding authority must throw its genuine ReferenceError');
    const value=embedding.encodeHostError(thrown);
    const resumed=resumeJavaScriptModuleInitialization(actual.plan,state,value,true);state=resumed.state;record('resume',resumed,value,true);return resumed;
   }
   return transition;
  }throw new RangeError('runtime module initialization exceeds its explicit step budget');}
  finally{active=false;}
 }
 const api={get capture(){return structuredClone(actual);},get sourceFunctions(){return structuredClone(sourceFunctions);},readerProfile,get readerQualification(){return sourceReader?structuredClone(sourceReader.qualification):null;},readerEvents:()=>sourceReader?sourceReader.events():[],embedding,initialize(){return structuredClone(initialize());},state:()=>structuredClone(state),trace:()=>structuredClone(history),requests:()=>structuredClone(requests),
  readExport(name){const matches=actual.exports.filter(row=>row.kind==='data'&&row.name===name);if(matches.length!==1)throw new TypeError('a data export needs one actual public source binding');return structuredClone(readJavaScriptModuleBinding(actual.plan,state,matches[0].binding));},
  begin(name,args,{construct=false}={}){const result=initialize();if(result.$!=='moduleEvaluated')return result;const matches=actual.exports.filter(row=>row.kind==='function'&&row.name===name);if(matches.length!==1)throw new TypeError('a source call needs its actual public module function');return beginJavaScriptErrorModule(embedding.module,embedding.heap(),matches[0].binding,args,{construct});}
 };
 realm.instances.set(url.href,{capture:actual,definitions,readerProfile,readerRealmIdentity:sourceReader?sourceReaders.get(scope).identity:null,api});return api;
}
