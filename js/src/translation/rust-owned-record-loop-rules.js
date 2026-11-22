// Exact finite-domain record loops. The bound follows from the complete u8
// domain AND one authentic decrement on every continuing path, never fuel.
/** @returns {string} */
export function readRustOwnedRecordLoopOperation(){return 'rustOwnedRecordFiniteLoop';}
/** @returns {string} */
export function readRustOrderedCollectionLoopOperation(){return 'rustStandardCollectionOrderedLoop';}
/** @returns {number} */
export function readRustOwnedRecordLoopChunkSize(){return 8;}
/** @param {number} loop @param {number} chunk @param {number} collision @returns {string} */
export function readRustOwnedRecordLoopChunkName(loop,chunk,collision){if(!acceptRustOwnedRecordLoopCount(loop)||!acceptRustOwnedRecordLoopCount(chunk)||!acceptRustOwnedRecordLoopCount(collision))return '';return 'ml_finite_record_loop_chunk_'+String(loop)+'_'+String(chunk)+(collision===0?'':'_'+String(collision));}
/** @param {string} name @returns {boolean} */
export function testRustOwnedRecordLoopChunkName(name){return name.startsWith('ml_finite_record_loop_chunk_');}
/** This is an additional restriction on actual checked helpers, never a
 * caller-provided recursion/ownership certificate.
 * @param {boolean} generated @param {boolean} recursive @param {boolean} mutual @param {boolean} unknown @returns {string} */
export function readRustOwnedRecordLoopHelperError(generated,recursive,mutual,unknown){if(!generated)return 'a finite record loop chunk needs its actual generated declaration';if(recursive||mutual)return 'finite record loops cannot use a recursive or mutually recursive helper graph';return unknown?'finite record loops need a complete static helper call graph without dynamic application or external calls':'';}
/** @param {string} kind @param {number} bits @param {boolean} signed @param {number} wrappers @param {string} owner @param {string} operation @returns {string} */
export function readRustOwnedRecordLoopCounterError(kind,bits,signed,wrappers,owner,operation){return kind==='fixed'&&bits===8&&!signed&&wrappers===0&&owner==='u8'&&(operation==='ne'||operation==='gt')?'':'a finite record loop needs its actual direct u8 counter and pure nonzero head';}
/** Every ordinal denotes one possible successful nonzero iteration.
 * @param {number} bits @returns {number[]} */
export function planRustOwnedRecordLoopIterations(bits){const result=[];if(bits!==8)return result;for(let index=0;index<255;index+=1)result.push(index);return result;}
/** @param {string} kind @returns {boolean} */
export function testRustOwnedRecordLoopContinuingExit(kind){return kind==='value'||kind==='continue';}
/** @param {string} kind @returns {boolean} */
export function acceptRustOwnedRecordLoopExit(kind){return kind==='value'||kind==='continue'||kind==='break'||kind==='return'||kind==='abort';}
/** @param {string[]} exits @param {number[]} counts @returns {string} */
export function readRustOwnedRecordLoopProgressError(exits,counts){
 const budgetError='the complete record loop exits exceed their finite source budget';
 if(exits.length!==counts.length||exits.length===0||exits.length>256)return budgetError;
 for(let index=0;index<exits.length;index+=1){
  const exit=exits[index],count=counts[index];
  if(exit===undefined)return budgetError;
  if(count===undefined)return budgetError;
  if(!acceptRustOwnedRecordLoopExit(exit)||!acceptRustOwnedRecordLoopCount(count)||count>256)return budgetError;
  if(testRustOwnedRecordLoopContinuingExit(exit)&&count!==1)return 'every continuing record loop path must decrement its actual counter exactly once';
 }
 return '';
}
/** Finite nonnegative safe integers, with negative zero rejected explicitly.
 * @param {number} value @returns {boolean} */
export function acceptRustOwnedRecordLoopCount(value){return value>=0&&value<=9007199254740991&&value%1===0&&(value!==0||1/value>0);}
/** @param {boolean} overlaps @param {boolean} exactDecrement @returns {string} */
export function readRustOwnedRecordLoopWriteError(overlaps,exactDecrement){return overlaps&&!exactDecrement?'a record loop cannot replace or alias-write its progress counter':'';}
/** @param {string} before @param {string} after @returns {string} */
export function readRustOwnedRecordLoopInvariantError(before,after){return before===after?'':'each continuing record loop path must preserve every actual outer ownership and loan state';}
/** @param {string} kind @returns {string} */
export function readRustOwnedRecordLoopSignal(kind){return kind==='break'?'LoopBreak':kind==='continue'?'LoopContinue':'';}
/** @param {string} kind @returns {string} */
export function readRustOwnedRecordLoopLeafKind(kind){return kind==='break'?'broken':kind==='continue'?'continued':'';}
/** @param {string} constructor @returns {string} */
export function readRustOwnedRecordLoopExitDisposition(constructor){return constructor==='FunctionResult'?'return':constructor==='LoopBreak'?'break':constructor==='LoopContinue'||constructor==='StatementContinuation'?'continue':'';}
/** Full generated expression work/text is charged before copying instances.
 * @param {number} used @param {number} nodes @param {number} text @param {number} copies @returns {string} */
export function readRustOwnedRecordLoopExpansionError(used,nodes,text,copies){return !acceptRustOwnedRecordLoopCount(used)||!acceptRustOwnedRecordLoopCount(nodes)||!acceptRustOwnedRecordLoopCount(text)||!acceptRustOwnedRecordLoopCount(copies)||copies>255||nodes>100000||used>100000-nodes*copies||text>16777216/(copies+1)?'the complete finite record loop expansion exceeds its source budget':'';}
/** Two retained template forms and bounded per-iteration carrier syntax.
 * @param {number} nodes @param {number} text @returns {number[]} */
export function readRustOwnedRecordLoopExpansionReservation(nodes,text){if(!acceptRustOwnedRecordLoopCount(nodes)||!acceptRustOwnedRecordLoopCount(text)||nodes>100000||text>16777216)return [];return [nodes*2+64,text*2];}
/** Authenticated nested loops have their own finite progress derivation. Their
 * writes cannot contribute a variable number of outer-counter decrements.
 * @param {boolean} admitted @param {number} returns @param {boolean} overlap
 * @returns {string} */
export function readRustOwnedRecordNestedLoopError(admitted,returns,overlap){if(!admitted||!acceptRustOwnedRecordLoopCount(returns))return 'a nested loop needs its actual complete finite source admission';if(returns!==0)return 'a nested function return needs its complete enclosing loop exit continuation';return overlap?'a nested loop cannot write its enclosing progress counter':'';}
/** @param {number} depth @returns {string} */
export function readRustOwnedRecordNestedLoopDepthError(depth){return !acceptRustOwnedRecordLoopCount(depth)||depth>8?'the actual nested finite loops exceed their source depth budget':'';}
/** A completed child has actual retained return origins and a separately
 * retained zero-iteration continuation; its enclosing tracer supplies both.
 * @param {boolean} admitted @param {boolean} overlap @returns {string} */
export function readRustOwnedRecordNestedCompletionError(admitted,overlap){return !admitted?'a nested loop needs its actual complete finite source admission':overlap?'a nested loop cannot write its enclosing progress counter':'';}
/** @param {number} nodes @param {number} text @returns {string} */
export function readRustOwnedRecordLoopFootprintError(nodes,text){return !acceptRustOwnedRecordLoopCount(nodes)||!acceptRustOwnedRecordLoopCount(text)||nodes>100000||text>16777216?'the complete loop write footprint exceeds its source budget':'';}
/** @param {number} count @returns {string} */
export function readRustOwnedRecordLoopResidualCountError(count){return acceptRustOwnedRecordLoopCount(count)&&count>=1&&count<=256?'':'a propagated loop exit needs every actual produced residual branch within its source budget';}

/** Source span coordinates use the same UTF-16 positions as tokens.
 * @param {number} start @param {number} end @returns {boolean} */
export function acceptRustSourceRegionSpan(start,end){
 return start>=0&&start%1===0&&end>start&&end<=9007199254740991&&end%1===0;
}

/** @param {number} nameLength @param {number} identityLength @param {number} start @param {number} end @returns {boolean} */
export function acceptRustSourceStateBinder(nameLength,identityLength,start,end){
 return nameLength>0&&nameLength<=4096&&identityLength>0&&identityLength<=4096&&acceptRustSourceRegionSpan(start,end);
}

/** @param {string} language @param {string} model @param {string} operation @returns {boolean} */
export function acceptRustSourceStateTransitionModel(language,model,operation){
 return language==='Rust'&&model==='Rust source state transition 1'&&(operation==='initialize'||operation==='write');
}

/** @param {string} language @param {string} model @param {number} slots @param {number} start @param {number} end @returns {boolean} */
export function acceptRustOwnedVectorSourceFoldModel(language,model,slots,start,end){
 return language==='Rust'&&model==='Rust owned vector source fold 1'&&slots>=0&&slots<=512&&acceptRustSourceRegionSpan(start,end);
}

/** @returns {string[]} */
export function readRustOwnedVectorSourceFoldControlVariants(){
 return ['FunctionResult','StatementContinuation','LoopBreak','LoopContinue'];
}
