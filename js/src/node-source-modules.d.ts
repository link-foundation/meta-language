/** Node-only embedding for explicitly selected supplied-source profiles. */
export type JavaScriptSemanticValue =
 | {$:'undefined'} | {$:'null'} | {$:'boolean';value:boolean}
 | {$:'number';value:number} | {$:'integer';value:bigint}
 | {$:'text';units:number[]} | {$:'sourceText';profile:'inspectable UTF16 source text 1';text:string} | {$:'symbol';identity:bigint}
 | {$:'object';identity:bigint;callable:boolean};
export interface JavaScriptSourceRealmAuthority {
 identity:bigint;
 realm:{errors:Readonly<Record<string,Function>>;objectPrototype:object;toPrimitive:symbol;getPrimitive:Function;assign:Function;raiseUninitializedBinding?:Function};
 definitions:Readonly<Record<string,Function>>;
}
export type JavaScriptSourceTransition =
 | {$:'machineCompleted'|'machineThrow';heap:unknown;value:JavaScriptSemanticValue}
 | {$:'machineInvalid';heap:unknown;reason:string}
 | {$:'machineResourceLimit';machine:unknown;reason:string}
 | {$:'machineContinue';machine:unknown}
 | {$:'machineAllocateObject';machine:unknown;prototype:JavaScriptSemanticValue}
 | {$:'machineNeedCollectionIterable';machine:unknown;target:JavaScriptSemanticValue;input:JavaScriptSemanticValue}
 | {$:'machineAllocateError'|'machineAllocateSourceError'|'machineAllocateErrorValue'|'machineInvokeErrorIntrinsic'|'machineInvokeHostCapability';machine:unknown;[field:string]:unknown};
export interface JavaScriptSourceEmbedding {
 readonly module:unknown;
 readonly authority:{perform(request:unknown):{heap:unknown;value:JavaScriptSemanticValue;abrupt:boolean}};
 heap():unknown;
 adoptHeap(heap:unknown):void;
 allocateIdentity():bigint;
 encode(value:unknown):JavaScriptSemanticValue;
 decode(value:JavaScriptSemanticValue):unknown;
 encodeHostError(value:unknown):JavaScriptSemanticValue;
 materializeData(value:JavaScriptSemanticValue):unknown;
}
export interface PreparedJavaScriptErrorClosure {archive:unknown;embedding:JavaScriptSourceEmbedding}
export interface JavaScriptSourceExecution {$:'callExecution';transition:JavaScriptSourceTransition;steps:number;objectIdentitiesUsed:number}
declare const runtimeDataModuleScope:unique symbol;
export interface RuntimeDataModuleScope {readonly [runtimeDataModuleScope]:true}
export interface JavaScriptModuleInitializationState {
 $:'moduleInitializationState';moduleIdentity:string;sourceSha256:string;moduleURL:string;realm:bigint;
 phase:'uninitialized'|'initializing'|'loading'|'temporal dead zone'|'evaluated'|'errored';position:number;
 slots:{$:'moduleBindingSlot';binding:number;initialized:boolean;value:JavaScriptSemanticValue}[];
 failure:JavaScriptSemanticValue;
}
export type JavaScriptModuleInitializationTransition =
 | {$:'moduleContinue'|'moduleEvaluated'|'moduleWaiting';state:JavaScriptModuleInitializationState}
 | {$:'moduleAbrupt';state:JavaScriptModuleInitializationState;value:JavaScriptSemanticValue}
 | {$:'moduleInvalid';state:JavaScriptModuleInitializationState;reason:string}
 | {$:'moduleReadData';state:JavaScriptModuleInitializationState;binding:number;specifier:string;readerOwner:string}
 | {$:'moduleTemporalDeadZone';state:JavaScriptModuleInitializationState;binding:number;name:string};
export interface PreparedJavaScriptRuntimeDataModule {
 readonly capture:unknown;
 readonly sourceFunctions:unknown;
 readonly embedding:JavaScriptSourceEmbedding;
 initialize():JavaScriptModuleInitializationTransition;
 state():JavaScriptModuleInitializationState;
 trace():readonly unknown[];
 requests():readonly unknown[];
 readExport(name:string):{$:string;value?:JavaScriptSemanticValue;[field:string]:unknown};
 begin(name:string,args:JavaScriptSemanticValue[],options?:{construct?:boolean}):JavaScriptSourceTransition|JavaScriptModuleInitializationTransition;
}
export function readSuppliedSourceLibraryClosure(sourcePath:string,modules:readonly {path:string;source:string}[],options?:{aliases?:boolean;classes?:string;reExports?:string;errorClasses?:string}):unknown;
export function captureSourceLibraryErrorClosure(closure:unknown):unknown;
export function checkSourceLibraryErrorClosure(archive:unknown):unknown;
export function prepareJavaScriptErrorSourceClosure(archive:unknown,authority:JavaScriptSourceRealmAuthority):PreparedJavaScriptErrorClosure;
export function beginJavaScriptErrorSourceClosure(linked:PreparedJavaScriptErrorClosure,exportName:string,args:JavaScriptSemanticValue[],options?:{construct?:boolean}):JavaScriptSourceTransition;
export function advanceJavaScriptErrorSourceClosure(linked:PreparedJavaScriptErrorClosure,transition:JavaScriptSourceTransition,options?:{steps?:number;objectIdentities?:bigint[]}):JavaScriptSourceTransition;
export function resumeJavaScriptErrorSourceClosure(linked:PreparedJavaScriptErrorClosure,transition:JavaScriptSourceTransition):JavaScriptSourceTransition;
export function readActiveRuntimeDataVersions():{nodeMajor:number;nodeMinor:number;linksVersion:string};
export function readShippedRuntimeDataAdapterContract():unknown;
export function captureJavaScriptRuntimeDataModule(source:string,options:{identifier:string;readerContract:unknown;model?:string}):unknown;
export function checkJavaScriptRuntimeDataModule(capture:unknown):unknown;
export function createRuntimeDataModuleScope():RuntimeDataModuleScope;
export function prepareJavaScriptRuntimeDataModule(capture:unknown,authority:JavaScriptSourceRealmAuthority,options:{moduleURL:string|URL;scope?:RuntimeDataModuleScope;trace?:boolean;readerProfile?:'qualified runtime Links data IO adapter 1'|'actual project data source functions 1';readerRealmIdentity?:bigint}):PreparedJavaScriptRuntimeDataModule;
export interface PreparedJavaScriptProjectDataModule {
 readonly capture:unknown;
 readonly embedding:JavaScriptSourceEmbedding;
 readonly qualification:{model:'actual project data source execution 1';versions:{nodeMajor:number;nodeMinor:number;linksVersion:string};providerSourceSha256:string;decoderSourceSha256:string;externalOperations:readonly {source:string;operation:string;owner:string}[];remainingBoundary:string};
 begin(name:string,args:JavaScriptSemanticValue[]):JavaScriptSourceTransition;
 encode(value:unknown):JavaScriptSemanticValue;
 events():readonly unknown[];
}
export function captureJavaScriptProjectDataSource(source:string,options:{identifier:string;entries?:readonly string[]}):unknown;
export function checkJavaScriptProjectDataSource(capture:unknown):unknown;
export function readShippedJavaScriptProjectDataSource(options?:{entries?:readonly string[]}):unknown;
export function prepareJavaScriptProjectDataModule(capture:unknown,options:{realmIdentity:bigint;trace?:boolean}):PreparedJavaScriptProjectDataModule;
export function runJavaScriptSourceTransition(embedding:JavaScriptSourceEmbedding,transition:JavaScriptSourceTransition,options?:{steps?:number}):JavaScriptSourceExecution;

/** Explicit actual-source profile; unsupported constructs retain diagnostics. */
export function captureJavaScriptOrdinaryClassModule(source:string,options:{identifier:string;entries:string[]}):unknown;
export function checkJavaScriptOrdinaryClassModule(capture:unknown):unknown;
export interface PreparedJavaScriptOrdinaryClassModule {
 readonly capture:unknown;
 readonly embedding:JavaScriptSourceEmbedding;
 readonly collection:unknown;
 callable(name:string):JavaScriptSemanticValue;
 begin(name:string,args:JavaScriptSemanticValue[]):JavaScriptSourceTransition;
}
export function prepareJavaScriptOrdinaryClassModule(capture:unknown,options:{realmIdentity:bigint}):PreparedJavaScriptOrdinaryClassModule;

export function captureJavaScriptOrdinaryClassSourceClosure(closure:unknown,options:{entryPath:string}):unknown;
export function checkJavaScriptOrdinaryClassSourceClosure(archive:unknown):unknown;
export interface PreparedJavaScriptOrdinaryClassSourceClosure {
 readonly archive:unknown;
 readonly capture:unknown;
 readonly embedding:JavaScriptSourceEmbedding;
 readonly collection:unknown;
 begin(exported:string,args:JavaScriptSemanticValue[]):JavaScriptSourceTransition;
}
export function prepareJavaScriptOrdinaryClassSourceClosure(archive:unknown,options:{realmIdentity:bigint}):PreparedJavaScriptOrdinaryClassSourceClosure;
