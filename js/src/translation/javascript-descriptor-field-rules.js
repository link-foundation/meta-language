// Generated from parity/translation/javascript-descriptor-fields.lino. Do not edit.
// Complete Links source sha256 a756fecbd307bde4bc345de00e09bd0444973c36f12f184acca1be36a053b883
/** @param {number} index @returns {{name:string,kind:string,units:number[]}} */
export function readJavaScriptDescriptorField(index){
  if(index === 0) return {name:"enumerable",kind:"boolean",units:[101,110,117,109,101,114,97,98,108,101]};
  if(index === 1) return {name:"configurable",kind:"boolean",units:[99,111,110,102,105,103,117,114,97,98,108,101]};
  if(index === 2) return {name:"value",kind:"value",units:[118,97,108,117,101]};
  if(index === 3) return {name:"writable",kind:"boolean",units:[119,114,105,116,97,98,108,101]};
  if(index === 4) return {name:"get",kind:"accessor",units:[103,101,116]};
  if(index === 5) return {name:"set",kind:"accessor",units:[115,101,116]};
  return {name:"",kind:"invalid",units:[]};
}
