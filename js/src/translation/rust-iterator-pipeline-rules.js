import {acceptRustIteratorArithmeticConsumer,planRustIteratorArithmeticTerminal} from './rust-iterator-arithmetic-rules.js';
// Lazy scalar iterator composition is expressed in existing common terms.
/** @param {string} operation @returns {boolean} */
export function acceptRustIteratorPipelineStage(operation){return operation==='map'||operation==='filter';}
/** @param {string} operation @param {number} layers @returns {number} */
export function readRustIteratorPipelineParameterLayers(operation,layers){return operation==='filter'?layers+1:operation==='map'?layers:-1;}
/** @param {string} operation @returns {string} */
export function readRustIteratorPipelineResultContext(operation){return operation==='filter'?'bool':operation==='map'?'infer':'';}
/** @param {string} operation @param {string} state @returns {string} */
export function readRustIteratorPipelineResultState(operation,state){return operation==='map'?'owned':operation==='filter'?state:'';}
/** @param {string} kind @param {number} parameters @param {number} wrappers
 * @param {boolean} parameter @param {boolean} result @param {boolean} resultCopy @param {boolean} owner @returns {string} */
export function readRustIteratorPipelineCallableError(kind,parameters,wrappers,parameter,result,resultCopy,owner){return kind==='function'&&parameters===1&&wrappers===0&&parameter&&result&&resultCopy&&owner?'':'a pipeline stage needs its complete immutable scalar callable parameter and result owners';}
/** @param {string} operation @returns {string[]} */
export function planRustIteratorPipelineStage(operation){return operation==='map'?['application','continuation','bind']:operation==='filter'?['application','continuation','inactive','if']:[];}
/** @param {string} consumer @returns {string[]} */
export function planRustIteratorPipelineTerminal(consumer){return acceptRustIteratorArithmeticConsumer(consumer)?planRustIteratorArithmeticTerminal(consumer):consumer==='all'||consumer==='any'?['callback','item','call one']:consumer==='fold'||consumer==='rfold'?['callback','accumulator','item','call two']:consumer==='count'?['accumulator','one','add']:consumer==='collect'?['accumulator','item','append']:[];}
/** @param {string} consumer @returns {string} */
export function readRustIteratorPipelineInactive(consumer){return consumer==='all'?'true':consumer==='any'?'false':consumer==='fold'||consumer==='rfold'||consumer==='count'||consumer==='collect'||acceptRustIteratorArithmeticConsumer(consumer)?'accumulator':'';}
/** @param {string} consumer @returns {string} */
export function readRustIteratorPipelineConsumerOperation(consumer){return consumer==='all'?'every':consumer==='any'?'some':consumer==='fold'||consumer==='rfold'||consumer==='count'||consumer==='collect'||acceptRustIteratorArithmeticConsumer(consumer)?'reduce':'';}
/** The role stream retains receiver/stage/initializer/terminal construction order.
 * @param {string} consumer @param {number} count @returns {string[]} */
export function planRustIteratorPipelineCaptures(consumer,count){
 const roles=['receiver'];for(let index=0;index<count;index+=1)roles.push('stage');
 if(consumer==='fold'||consumer==='rfold')roles.push('initial');
 if(consumer==='all'||consumer==='any'||consumer==='fold'||consumer==='rfold')roles.push('callback');
 return roles;
}

/** @param {string} operation @returns {string} */
export function readRustIteratorPipelineContinuationItem(operation){return operation==='map'?'mapped':operation==='filter'?'original':'';}
/** @param {string} consumer @returns {string} */
export function readRustIteratorPipelineSeed(consumer){return acceptRustIteratorArithmeticConsumer(consumer)?'numeric identity':consumer==='count'?'zero':consumer==='collect'?'empty array':consumer==='fold'||consumer==='rfold'?'initial':'';}
