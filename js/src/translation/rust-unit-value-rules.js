// Actual source type facts select existing Unit and ordered binding operations.
/** A collecting pass defers only direct call-result ownership to the complete
 * declaration catalog. The final source pass requires an actual direct Unit.
 * @param {string} kind @param {boolean} collecting @param {boolean} proven
 * @param {string} type @param {boolean} indirected @returns {string} */
export function readRustUnitStatementDisposition(kind,collecting,proven,type,indirected){
 if(kind==='abort')return 'abort';
 if(proven&&type==='unit'&&!indirected)return 'bind';
 return collecting&&kind==='app'?'bind':'refuse';
}

/** @param {string} name @returns {string} */
export function readRustFunctionMacroDisposition(name){return name==='println'?'print':'';}

/** The host has identified an actual return keyword at start. Only punctuation
 * appears in values; literal text cannot supply a delimiter. A terminal return
 * has no continuation before the owning function block closes.
 * -1 refuses a continuation; -2 requests more actual source tokens.
 * @param {string[]} values @param {number} start @returns {number} */
export function findRustTerminalReturnBoundary(values,start){
 if(!Number.isSafeInteger(start)||start<0||start>=values.length)return -1;
 const stack=[];
 for(let at=start+1;at<values.length;at+=1){
  const value=values[at]??'';
  if(value==='('||value==='['||value==='{'){stack.push(value);continue;}
  if(stack.length===0&&(value===';'||value==='}')){
   let end=at;while(values[end]===';')end+=1;
   if(end>=values.length)return -2;
   if(values[end]==='}')return end;
   return -1;
  }
  if(value===')'||value===']'||value==='}'){
   const previous=stack[stack.length-1]??'';
   if(value===')'&&previous!=='('||value===']'&&previous!=='['||value==='}'&&previous!=='{')return -1;
   stack.pop();
  }
 }
 return -2;
}
