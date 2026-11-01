// Independent complete originals. All expectations are declarations for future
// execution, not observations. The old closed-class inheritance refusal stays
// in its original profile; this successor requires real Error realm authority.
export const javascriptErrorClassVersions={node:'>=22.15 <23 || >=24 <25',language:'ECMAScript 2024-2025',rust:'>=1.90 <1.100'};
export const translationErrorSource=`// Errors raised by the portable-core translator. \`unsupported\` errors are the
// precise obligations the issue requires: they name the construct, the
// source span, and the reason no faithful encoding exists yet.

export class TranslationError extends Error {
  constructor(kind, message, span = undefined, details = {}) {
    super(span ? \`\${message} at \${span.start}..\${span.end}\` : message);
    this.name = 'TranslationError';
    this.kind = kind;
    this.reason = message;
    this.span = span ? { start: span.start, end: span.end } : null;
    this.details = details;
  }
}

/** A precise obligation for a construct outside the portable core. */
export function unsupported(construct, reason, span) {
  return new TranslationError('unsupported', \`\${construct}: \${reason}\`, span, { construct });
}

export function typeError(message, span) {
  return new TranslationError('type', message, span);
}
`;
export const javascriptErrorClassDecisions=[
 {operation:'testJavaScriptErrorConstructorMode',arguments:['derived Error'],expected:true},
 {operation:'testJavaScriptErrorConstructorMode',arguments:['strict'],expected:false},
 {operation:'testJavaScriptErrorIntrinsicMode',arguments:['Error intrinsic'],expected:true},
 {operation:'testJavaScriptErrorIntrinsicMode',arguments:['derived Error'],expected:false},
 {operation:'readJavaScriptErrorConstructionKindError',arguments:['Error',true,'derived Error'],expected:''},
 {operation:'readJavaScriptErrorConstructionKindError',arguments:['TypeError',false,'strict'],expected:''},
 {operation:'readJavaScriptErrorConstructionKindError',arguments:['Custom',false,'strict'],expected:'Error value construction needs its actual standard intrinsic class'},
 {operation:'readJavaScriptErrorConstructionKindError',arguments:['Error',true,'strict'],expected:'only an actual derived constructor binds the initialized receiver'},
];
const ordinary=(identity,callable=false)=>({$:'ordinaryObject',identity,prototype:{$:'null'},extensible:true,callable,properties:[]});
const priorDirectory={$:'errorAllocationDirectory',objects:[{$:'errorAllocationIdentity',identity:1n,kind:'ordinaryObject',callable:false},{$:'errorAllocationIdentity',identity:2n,kind:'ordinaryObject',callable:true}],symbols:[18446744073709551615n]};
const sparseHeap={$:'ordinaryHeap',objects:[ordinary(1n),ordinary(2n,true),{$:'hostErrorObject',identity:7n,prototype:{$:'object',identity:1n,callable:false},extensible:true,callable:false,properties:[],hostGet:{$:'object',identity:2n,callable:true},hostSet:{$:'object',identity:2n,callable:true}}],symbols:[18446744073709551615n]};
export const javascriptErrorClassAllocationDecisions=[
 {operation:'readJavaScriptErrorAllocationResultError',arguments:[sparseHeap,{$:'object',identity:7n,callable:false},{$:'object',identity:1n,callable:false},priorDirectory],expected:''},
 {operation:'readJavaScriptErrorAllocationResultError',arguments:[sparseHeap,{$:'object',identity:1n,callable:false},{$:'object',identity:1n,callable:false},priorDirectory],expected:'an intrinsic Error result requires its actual fresh noncallable global Object identity'},
 {operation:'readJavaScriptErrorPriorDirectoryError',arguments:[{...sparseHeap,objects:sparseHeap.objects.filter(row=>row.identity!==2n)},priorDirectory],expected:'an Error allocation cannot remove or retype prior Object identities'},
 {operation:'readJavaScriptErrorPriorDirectoryError',arguments:[{...sparseHeap,objects:[ordinary(1n,true),...sparseHeap.objects.slice(1)]},priorDirectory],expected:'an Error allocation cannot remove or retype prior Object identities'},
 {operation:'readJavaScriptErrorPriorDirectoryError',arguments:[{...sparseHeap,symbols:[]},priorDirectory],expected:'an Error allocation cannot remove or retype prior Symbol identities'},
];
export const javascriptErrorClassOriginals=[
 {name:'actual TranslationError constructor complete defaults and scalar fields',versions:javascriptErrorClassVersions,source:translationErrorSource,entry:'TranslationError',construct:true,args:['type','bad'],expected:{name:'TranslationError',message:'bad',kind:'type',reason:'bad',span:null,details:{},ownMessage:true,rendered:'TranslationError: bad'}},
 {name:'actual unsupported helper constructor retains independent span and details',versions:javascriptErrorClassVersions,source:translationErrorSource,entry:'unsupported',construct:false,args:['tuple','missing',{start:2,end:9}],expected:{name:'TranslationError',message:'tuple: missing at 2..9',kind:'unsupported',reason:'tuple: missing',span:{start:2,end:9},details:{construct:'tuple'},ownMessage:true,rendered:'TranslationError: tuple: missing at 2..9'}},
 {name:'actual typeError helper accepts zero coordinates without truthiness loss',versions:javascriptErrorClassVersions,source:translationErrorSource,entry:'typeError',construct:false,args:['bad',{start:0,end:0}],expected:{name:'TranslationError',message:'bad at 0..0',kind:'type',reason:'bad',span:{start:0,end:0},details:{},ownMessage:true,rendered:'TranslationError: bad at 0..0'}},
 {name:'undefined Error message has no own message while source reason remains undefined',versions:javascriptErrorClassVersions,source:translationErrorSource,entry:'TranslationError',construct:true,args:['type',undefined],expected:{name:'TranslationError',message:'',kind:'type',reason:undefined,span:null,details:{},ownMessage:false,rendered:'TranslationError'}},
 {name:'derived fields read actual superclass message then earlier source assignment',versions:javascriptErrorClassVersions,source:`export class DetailError extends TypeError { constructor(message) { super(message); this.name = 'DetailError'; this.kind = this.message; this.reason = this.kind; this.span = null; this.details = {}; } }`,entry:'DetailError',construct:true,args:['ordered'],expected:{name:'DetailError',message:'ordered',kind:'ordered',reason:'ordered',span:null,details:{},ownMessage:true,rendered:'DetailError: ordered'}},
];
export const javascriptErrorClassRefusals=[
 {name:'actual local Error function cannot certify an intrinsic',source:`function Error(message) { return message; } export class E extends Error { constructor(message) { super(message); } }`},
 {name:'actual local Error class cannot certify an intrinsic',source:`class Error { constructor(message) { this.message = message; } } export class E extends Error { constructor(message) { super(message); } }`},
 {name:'dynamic bases require a separately complete source path',source:`export class E extends choose() { constructor(message) { super(message); } }`},
 {name:'source options are not silently discarded by super',source:`export class E extends Error { constructor(message,options) { super(message, options); } }`},
 {name:'this before super retains uninitialized receiver requirement',source:`export class E extends Error { constructor(message) { this.name = 'E'; super(message); } }`},
 {name:'returned alternate constructor object requires exact source semantics',source:`export class E extends Error { constructor(message) { super(message); return {}; } }`},
 {name:'later defaults cannot read uninitialized parameters',source:`export class E extends Error { constructor(message = later, later = 'x') { super(message); } }`},
 {name:'prototype mutation source cannot bypass actual newTarget ownership',source:`export class E extends Error { constructor(message) { super(message); this.__proto__ = null; } }`},
];
export const javascriptErrorClassRequiredHostBoundaries=[
 {name:'lazy stack first line observes later source name assignment',assertion:'actual live Error.stack getter retained; read only after constructor field assignments; first line DetailError: ordered'},
 {name:'full original source stack frames require actual source frame mapping',assertion:'exact UTF16 stack including original file/function/line/column must match original source; host allocator frames are not source frames'},
 {name:'actual source getter effects remain translated',assertion:'span.start/span.end accessor bodies execute through machine call frames in original super-message then field-read order; no host body fallback',source:`export class CallbackError extends Error { constructor(name) { super('message'); this.name = name; } } export function forbidden() { throw 'host fallback executed'; } export function render(value) { return value.toString(); }`},
 {name:'opaque Error reflection stays explicit',assertion:'ownKeys, getOwnPropertyDescriptor, deletion, extensibility and prototype mutation require authentic engine protocols rather than ordinary empty-property snapshots'},
];
