/** @param {string} owner @param {string} method @returns {number} */
export function readRustOwnedStandardSumCallbackArity(owner,method){
 if(owner==='std::option::Option')return ['unwrap_or_else','ok_or_else','map','and_then','or_else'].includes(method)?1:-1;
 return owner==='std::result::Result'&&['unwrap_or_else','map','map_err','and_then','or_else'].includes(method)?1:-1;
}
/** @param {string} owner @param {string} method @param {number} minor @returns {string} */
export function readRustOwnedStandardSumCallbackVersionError(owner,method,minor){
 if(readRustOwnedStandardSumCallbackArity(owner,method)<0)return 'the owned standard sum callback method is outside this profile';
 return Number.isInteger(minor)&&minor>=90&&minor<100?'':'the owned standard sum callback source version is uncovered';
}
/** @param {string} owner @param {string} method @returns {number} */
export function readRustOwnedStandardSumCallbackParameter(owner,method){
 if(readRustOwnedStandardSumCallbackArity(owner,method)<0)return -2;
 return owner==='std::option::Option'?method==='map'||method==='and_then'?0:-1:method==='map'||method==='and_then'?0:1;
}
/** @param {string} owner @param {string} method @returns {string} */
export function readRustOwnedStandardSumCallbackResult(owner,method){
 if(readRustOwnedStandardSumCallbackArity(owner,method)<0)return '';
 return method==='and_then'||method==='or_else'?'bound sum':method==='unwrap_or_else'?'payload':method==='ok_or_else'?'std::result::Result':owner;
}
/** Retained complete argument identity in the callback result, -1 if none.
 * @param {string} owner @param {string} method @returns {number} */
export function readRustOwnedStandardSumCallbackRetainedArgument(owner,method){
 if(readRustOwnedStandardSumCallbackArity(owner,method)<0)return -2;
 return method==='or_else'?0:owner==='std::result::Result'&&method==='and_then'?1:-1;
}
/** @param {string} source @param {string} result @param {number} layers
 * @param {boolean} identity @returns {string} */
export function readRustOwnedStandardSumCallbackBoundResultError(source,result,layers,identity){return source===result&&(source==='std::option::Option'||source==='std::result::Result')&&layers===0&&identity?'':'the callback must return its actual direct same-family sum and preserve every retained argument identity';}
/** -1 denotes the actual callback result origin.
 * @param {string} owner @param {string} method @returns {number[]} */
export function planRustOwnedStandardSumCallbackArguments(owner,method){
 if(readRustOwnedStandardSumCallbackArity(owner,method)<0||['unwrap_or_else','and_then','or_else'].includes(method))return [];
 if(method==='ok_or_else')return [0,-1];
 if(owner==='std::option::Option')return [-1];
 return method==='map'?[-1,1]:[0,-1];
}
/** @param {string} owner @param {string} variant @param {string} method @returns {string[]} */
export function planRustOwnedStandardSumCallbackBranch(owner,variant,method){
 if(owner==='std::option::Option'){
  if(method==='and_then')return variant==='Some'?['','invoke']:variant==='None'?['None','empty']:[];
  if(method==='or_else')return variant==='Some'?['Some','payload']:variant==='None'?['','invoke']:[];
  if(method==='unwrap_or_else')return variant==='Some'?['','payload']:variant==='None'?['','invoke']:[];
  if(method==='ok_or_else')return variant==='Some'?['Ok','payload']:variant==='None'?['Err','invoke']:[];
  if(method==='map')return variant==='Some'?['Some','invoke']:variant==='None'?['None','empty']:[];
 }
 if(owner==='std::result::Result'){
  if(method==='and_then')return variant==='Ok'?['','invoke']:variant==='Err'?['Err','payload']:[];
  if(method==='or_else')return variant==='Ok'?['Ok','payload']:variant==='Err'?['','invoke']:[];
  if(method==='unwrap_or_else')return variant==='Ok'?['','payload']:variant==='Err'?['','invoke']:[];
  if(method==='map')return variant==='Ok'?['Ok','invoke']:variant==='Err'?['Err','payload']:[];
  if(method==='map_err')return variant==='Ok'?['Ok','payload']:variant==='Err'?['Err','invoke']:[];
 }
 return [];
}
/** Separate callable-Copy proof from the movable source signature domains.
 * @param {boolean} ordinary @param {string[]} names @param {string[]} roles
 * @param {string} result @returns {string} */
export function readRustOwnedStandardSumCallbackClosureHeaderError(ordinary,names,roles,result){
 if(!ordinary)return 'an owned-domain callback needs the active ordinary source profile';
 if(names.length!==roles.length||names.length>1024)return 'the complete owned-domain callback header exceeds its source budget';
 for(let at=0;at<roles.length;at+=1){if(roles[at]!=='copy'&&roles[at]!=='owned')return 'the callback parameter needs its actual direct Copy or supported owned source domain';for(let before=0;before<at;before+=1)if(names[at]===names[before])return 'callback parameters need distinct source bindings';}
 return result==='copy'||result==='owned'?'':'the callback result needs its actual direct Copy or supported owned source domain';
}
/** @param {number} captures @param {number} assigned @returns {string} */
export function readRustOwnedStandardSumCallbackCaptureError(captures,assigned){return captures===0&&assigned===0?'':'owned-domain callbacks currently need actual capture-free source closures';}
/** Actual lexical capture columns, never parameter/result Copy projections.
 * @param {boolean} moved @param {string[]} names @param {string[]} roles
 * @param {string[]} identities @param {number[]} layers @param {boolean[]} borrowed
 * @param {number} assigned @returns {string} */
export function readRustOwnedStandardSumCallbackMoveCaptureError(moved,names,roles,identities,layers,borrowed,assigned){
 if(assigned!==0)return 'mutable callback captures need their actual environment state interpretation';
 if(names.length>1024||roles.length!==names.length||identities.length!==names.length||layers.length!==names.length||borrowed.length!==names.length)return 'the complete copied capture environment needs every actual source binding';
 if(names.length>0&&!moved)return 'captured owned-domain callbacks need an explicit move source closure';
 for(let at=0;at<names.length;at+=1){
  if(names[at]===''||roles[at]!=='copy'||identities[at]===''||layers[at]!==0||borrowed[at]!==false)return 'a moved callback capture needs its actual complete direct Copy value without borrowed-place or loan provenance';
  for(let before=0;before<at;before+=1)if(names[before]===names[at])return 'copied callback captures need distinct actual lexical bindings';
 }
 return '';
}
/** @param {string[]} parameters @param {string} result @param {boolean} moved
 * @param {string[]} names @param {string[]} roles @param {string[]} identities
 * @param {number[]} layers @param {boolean[]} borrowed @param {number} assigned @returns {boolean} */
export function acceptRustOwnedStandardSumCallbackMoveClosureCopy(parameters,result,moved,names,roles,identities,layers,borrowed,assigned){
 if(parameters.length>1024||(result!=='copy'&&result!=='owned'))return false;
 for(const role of parameters)if(role!=='copy'&&role!=='owned')return false;
 return readRustOwnedStandardSumCallbackMoveCaptureError(moved,names,roles,identities,layers,borrowed,assigned)==='';
}
/** Nonzero environments remain actual immediate Lambda syntax. Existing fn
 * pointer coercion rules still reject a captured environment.
 * @param {number} captures @param {string} kind @param {boolean} identity
 * @param {boolean} moved @param {number} names @returns {string} */
export function readRustOwnedStandardSumCallbackImmediateCaptureError(captures,kind,identity,moved,names){
 return captures===0||captures>0&&kind==='lambda'&&identity&&moved&&names===captures?'':'a copied callback environment needs its actual explicit move Lambda source and span identity';
}
/** An empty environment is Copy independently of movable input/output types.
 * @param {string[]} parameters @param {string} result @param {number} captures
 * @param {number} assigned @returns {boolean} */
export function acceptRustOwnedStandardSumCallbackClosureCopy(parameters,result,captures,assigned){
 if(captures!==0||assigned!==0||parameters.length>1024||(result!=='copy'&&result!=='owned'))return false;
 for(const role of parameters)if(role!=='copy'&&role!=='owned')return false;return true;
}
/** @param {string} kind @param {number} parameters @param {number} expected
 * @param {number} wrappers @param {boolean} callableCopy @param {string} result @returns {string} */
export function readRustOwnedStandardSumCallbackSourceError(kind,parameters,expected,wrappers,callableCopy,result){return kind==='function'&&parameters===expected&&wrappers===0&&callableCopy&&(result==='copy'||result==='owned')?'':'the callback needs its actual Copy callable and complete movable parameter/result contracts';}
/** Selection is read-only context, never source admission or type inference.
 * @returns {number} */
export function readRustOwnedStandardSumCallbackSelectionTokenLimit(){return 4096;}
/** Code 1 named value, 2 factory call, 3 explicitly written lambda result.
 * The returned source path is a half-open token range, not a parsed type.
 * @param {string[]} values @param {string[]} kinds @returns {number[]} */
export function planRustOwnedStandardSumCallbackSelection(values,kinds){
 if(values.length!==kinds.length||values.length===0||values.length>4096||values[0]!=='(')return [];
 let at=1,layers=0;while(values[at]==='('&&layers<256){at+=1;layers+=1;}
 const moved=values[at]==='move';if(moved)at+=1;
 let code=1;
 if(values[at]==='||'||values[at]==='|'){
  if(values[at]==='||')at+=1;else{at+=1;let budget=0;while(at<values.length&&values[at]!=='|'&&budget<1024){at+=1;budget+=1;}if(values[at]!=='|')return [];at+=1;}
  if(values[at]!=='->')return [];at+=1;code=3;
 }else if(moved)return [];
 const start=at;
 if(kinds[at]!=='identifier'&&!['crate','self','super'].includes(values[at]))return [];
 at+=1;let parts=1;while(values[at]==='::'&&kinds[at+1]==='identifier'&&parts<256){at+=2;parts+=1;}
 if(code===3)return [3,start,at];
 if(values[at]==='(')code=2;else if(values[at]!==')'&&values[at]!==',')return [];
 return [code,start,at];
}
/** @param {string} owner @param {string} path @param {boolean} alias
 * @param {boolean} shadow @returns {boolean} */
export function acceptRustOwnedStandardSumCallbackWrittenSelection(owner,path,alias,shadow){return !alias&&!shadow&&owner==='library'&&(path==='std::string::String'||path==='std::vec::Vec');}
