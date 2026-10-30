// Source exceptions are typed intrinsic operations in the existing operation
// container. The historical serialized container tag remains `math`.

/** @param {string} name @returns {string} */
export function readJavaScriptSourceExceptionOperation(name) {
  if(name==='Error')return 'raiseJavaScriptError';
  if(name==='TypeError')return 'raiseJavaScriptTypeError';
  if(name==='RangeError')return 'raiseJavaScriptRangeError';
  if(name==='SyntaxError')return 'raiseJavaScriptSyntaxError';
  if(name==='ReferenceError')return 'raiseJavaScriptReferenceError';
  if(name==='URIError')return 'raiseJavaScriptURIError';
  return name==='EvalError'?'raiseJavaScriptEvalError':'';
}
/** @param {string} operation @returns {string} */
export function readJavaScriptSourceExceptionClass(operation) {
  if(operation==='raiseJavaScriptError')return 'Error';
  if(operation==='raiseJavaScriptTypeError')return 'TypeError';
  if(operation==='raiseJavaScriptRangeError')return 'RangeError';
  if(operation==='raiseJavaScriptSyntaxError')return 'SyntaxError';
  if(operation==='raiseJavaScriptReferenceError')return 'ReferenceError';
  if(operation==='raiseJavaScriptURIError')return 'URIError';
  return operation==='raiseJavaScriptEvalError'?'EvalError':'';
}
/** The source constructor retains raw arguments; the checked raise owns a
 * normalized message and its independent property-presence value.
 * @param {string} name @returns {string} */
export function readJavaScriptExceptionConstructorOperation(name) {
  return readJavaScriptSourceExceptionOperation(name)===''?'':'constructAndRaiseJavaScript'+name;
}
/** @param {string} operation @returns {string} */
export function readJavaScriptExceptionConstructorClass(operation) {
  if(operation==='constructAndRaiseJavaScriptError')return 'Error';
  if(operation==='constructAndRaiseJavaScriptTypeError')return 'TypeError';
  if(operation==='constructAndRaiseJavaScriptRangeError')return 'RangeError';
  if(operation==='constructAndRaiseJavaScriptSyntaxError')return 'SyntaxError';
  if(operation==='constructAndRaiseJavaScriptReferenceError')return 'ReferenceError';
  if(operation==='constructAndRaiseJavaScriptURIError')return 'URIError';
  return operation==='constructAndRaiseJavaScriptEvalError'?'EvalError':'';
}
/** @param {string} kind @param {string} operation @returns {boolean} */
export function acceptLinkedJavaScriptSourceException(kind,operation) {
  return kind==='math'&&readJavaScriptSourceExceptionClass(operation)!=='';
}
/** Error's undefined message is absent; its null message is the text "null".
 * @param {string} absence @returns {string} */
export function readJavaScriptSourceExceptionAbsentMessage(absence) {
  return absence==='null'?'null':'';
}
/** @param {string} kind @returns {string} */
export function readJavaScriptSourceExceptionMessageForm(kind) {
  if(kind==='string')return 'identity';
  if(kind==='unit')return 'empty';
  if(kind==='option')return 'optional';
  return kind==='bool'||kind==='float'||kind==='int'||kind==='nat'?'text':'unsupported';
}
/** @param {string} target @param {string} operation @param {string} message @param {string} present @param {boolean} utf16 @returns {string} */
export function renderJavaScriptSourceExceptionExpression(target,operation,message,present,utf16) {
  const name=readJavaScriptSourceExceptionClass(operation);
  if(name==='')return '';
  if(target==='JavaScript')return '((ml_message,ml_present)=>{throw new '+name+'(ml_present?ml_message:undefined);})('+message+','+present+')';
  if(target==='Rust')return '(crate::ml_source_exception::SourceException { class_name: String::from("'+name+'"), message_units: '+(utf16?'('+message+').0':'('+message+').encode_utf16().collect()')+', message_present: '+present+' }).raise()';
  return '';
}
/** Legacy output pairs have two fields; source exceptions have four.
 * @param {number} fields @returns {string} */
export function readSourceExceptionIgnoredPatternFields(fields){return fields===4?' _ _':'';}
/** @param {boolean} output @param {string[]} names @param {string[]} kinds @returns {boolean} */
export function acceptSourceExceptionPayloadFields(output,names,kinds){
 return output&&names.length===4&&kinds.length===4&&names[0]==='output'&&kinds[0]==='output'
  &&names[1]==='message'&&kinds[1]==='string'&&names[2]==='className'&&kinds[2]==='string'
  &&names[3]==='messagePresent'&&kinds[3]==='bool';
}
/** @returns {string} */
export function readSourceExceptionObservationStatement(){
 return 'ml_source_exception_observation retains the source exception constructor class, exact UTF-16 message and own-message property presence; main projects the display message at its ordinary process boundary';
}
/** Generic output append preserves every exception component.
 * @returns {string} */
export function readSourceExceptionRocqOutputPrelude(){
 return 'Definition ml_emit_source_exception {E : Type} (lines : list string) (next : list string * option E) : list string * option E := (List.app lines (fst next), snd next).';
}

/** Operands already passed ToNumeric; this does not conflate String addition
 * with numeric addition. A failing operation has no resulting value domain.
 * @param {string} operation @param {string} left @param {string} right @returns {string} */
export function readJavaScriptNumericOperationFailure(operation,left,right){
 const first=left==='int'||left==='nat',second=right==='int'||right==='nat';
 if(first&&right==='float'||left==='float'&&second)return 'mixed';
 return first&&second&&operation==='shiftJavaScriptNumberBitsRightUnsigned'?'unsigned':'';
}
/** V8 messages use the fixture's explicit Node version profile.
 * @param {string} failure @returns {string} */
export function readJavaScriptNumericOperationMessage(failure){
 if(failure==='mixed')return 'Cannot mix BigInt and other types, use explicit conversions';
 return failure==='unsigned'?'BigInts have no unsigned right shift, use >> instead':'';
}
