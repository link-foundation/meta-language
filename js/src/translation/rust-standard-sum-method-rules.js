import {readRustStandardSumSourceArgumentCount,readRustStandardSumSourceFieldArgument} from './rust-standard-sum-source-rules.js';
import {testRustOptionPanicConsumer} from './rust-panic-value-rules.js';

/** Only methods with complete owned Copy source semantics are admitted here.
 * @param {string} owner @param {string} method @returns {number} */
export function readRustStandardSumMethodArity(owner,method){
 const count=readRustStandardSumSourceArgumentCount(owner);
 if(count<0)return -1;
 if(owner==='std::option::Option'&&(method==='is_some'||method==='is_none'))return 0;
 if(owner==='std::option::Option'&&testRustOptionPanicConsumer(method))return method==='unwrap'?0:1;
 if(owner==='std::result::Result'&&(method==='is_ok'||method==='is_err'))return 0;
 return ['unwrap_or','unwrap_or_else','map','and_then'].includes(method)?1:-1;
}
/** The existing Option panic payload has no Debug obligation on its value.
 * Result panic methods still require their separate complete error model.
 * @param {string} owner @param {string} method @param {string} role @returns {boolean} */
export function acceptRustStandardSumOwnedPanicReceiver(owner,method,role){return owner==='std::option::Option'&&testRustOptionPanicConsumer(method)&&role==='owned';}

/** -2: no callable; -1: zero parameters; otherwise the actual source child.
 * @param {string} owner @param {string} method @returns {number} */
export function readRustStandardSumMethodCallbackArgument(owner,method){
 if(readRustStandardSumMethodArity(owner,method)<0)return -2;
 if(method==='map'||method==='and_then')return 0;
 return method==='unwrap_or_else'?(owner==='std::option::Option'?-1:1):-2;
}

/** @param {string} owner @param {string} method @returns {string} */
export function readRustStandardSumMethodResult(owner,method){
 if(readRustStandardSumMethodArity(owner,method)<0)return '';
 return method==='map'?'mapped sum':method==='and_then'?'bound sum':method==='unwrap_or'||method==='unwrap_or_else'||testRustOptionPanicConsumer(method)?'payload':'boolean';
}

/** Constructor roles are selected from actual standard field positions.
 * Captures precede the match: even an unused eager default/factory evaluates.
 * @param {string} owner @param {string} constructor @param {string} method
 * @returns {string} */
export function readRustStandardSumMethodBranch(owner,constructor,method){
 if(readRustStandardSumMethodArity(owner,method)<0)return '';
 const argument=readRustStandardSumSourceFieldArgument(owner,constructor);
 if(argument===-2)return '';
 const success=argument===0;
 if(testRustOptionPanicConsumer(method))return success?'payload':'panic';
 if(method==='is_some'||method==='is_ok')return success?'true':'false';
 if(method==='is_none'||method==='is_err')return success?'false':'true';
 if(method==='unwrap_or')return success?'payload':'default';
 if(method==='unwrap_or_else')return success?'payload':'invoke fallback';
 if(method==='map')return success?'construct mapped':'forward failure';
 return success?'invoke mapper':'forward failure';
}

/** Rust source std snapshots are selected independently of the target compiler.
 * @param {string} owner @param {string} method @param {number} minor
 * @returns {string} */
export function readRustStandardSumMethodSourceVersionError(owner,method,minor){
 if(owner!=='std::option::Option'||!testRustOptionPanicConsumer(method))return '';
 return Number.isInteger(minor)&&minor>=90&&minor<100?'':'Option unwrap and expect source semantics are uncovered outside Rust >=1.90.0 <1.100.0';
}

/** @param {string} owner @param {string} method @returns {string} */
export function readRustStandardSumMethodSourceVersionRange(owner,method){
 return owner==='std::option::Option'&&testRustOptionPanicConsumer(method)?'>=1.90.0 <1.100.0':'';
}

/** The message itself is provided by the existing common Option panic policy.
 * @param {string} owner @param {string} method @returns {string} */
export function readRustStandardSumMethodPanicMessageRole(owner,method){
 return owner==='std::option::Option'&&testRustOptionPanicConsumer(method)?method==='expect'?'supplied message':'standard message':'';
}

/** Actual function origins and exact signature comparisons are caller facts.
 * @param {string} kind @param {number} parameters @param {number} expected
 * @param {number} layers @param {boolean} copy @param {boolean} result
 * @returns {string} */
export function readRustStandardSumMethodCallbackError(kind,parameters,expected,layers,copy,result){
 return kind==='function'&&parameters===expected&&layers===0&&copy&&result?'':'a standard sum callback needs its complete direct Copy callable and actual parameter/result source types';
}
