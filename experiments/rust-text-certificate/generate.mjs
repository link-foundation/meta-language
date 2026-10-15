import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readProjectData,writeProjectData,checkProjectData} from '../../js/scripts/project-data.mjs';
import {parseJavaScript} from '../../js/src/translation/javascript.js';
import {checkProgram} from '../../js/src/translation/check.js';
import {emitRust} from '../../js/src/translation/emit-rust.js';
import {textCertificatePolicy as p,symbol as s} from './policy.mjs';
import {rustOwnedLocalStatePortableRules} from '../generate-rust-owned-local-state-rules.mjs';
const dependency="import {acceptRustPureUnitNameCharacter} from '../../js/src/translation/rust-pure-unit-call-rules.js';\nimport {readRustTextValueProjectionDomain,acceptRustTextValueProjectionOperation} from '../rust-text-value-projection/rules.mjs';\nimport {acceptRustOwnedLocalTextUnits} from '../../js/src/translation/rust-owned-local-state-rules.js';\n";
const width=name=>p.layouts[name].length;
const read=(array,base,column,layout)=>'('+array+'['+base+'+'+p.layouts[layout].indexOf(column)+']??-1)';
export function textCertificateSource(){return '// Generated from the linked portable Rust text certificate. Do not edit.\n'+dependency+`
/** @param {number} identity @returns {number[]} */
export function readTextCertificateKnownSymbol(identity){${p.symbols.map((value,index)=>'if(identity==='+String(index+1)+')return ['+[...Array(value.length)].map((_,at)=>value.charCodeAt(at)).join(',')+'];').join('')}return [-1];}
/** @param {number[]} kinds @param {number[]} tokens @param {number[]} declarations @param {number[]} parameters @param {number[]} positions @param {number[]} paths @param {number[]} layers @param {number[]} cores @param {number[]} operations @param {number[]} parts @param {number[]} names @param {number[]} units @returns {boolean} */
export function acceptTextCertificateStorage(kinds,tokens,declarations,parameters,positions,paths,layers,cores,operations,parts,names,units){
 if(kinds.length>${p.maximumRecords}||tokens.length>9*${p.maximumRecords}||(kinds[0]??-1)!==0||names.length%2!==0||names.length<${(p.symbols.length+1)*2}||(names[0]??-1)!==0||(names[1]??-1)!==0)return false;
 let partEnd=0;for(let row=0;row<operations.length;row=row+6){if((operations[row+4]??-1)!==partEnd)return false;partEnd=partEnd+(operations[row+5]??-1);}if(partEnd!==parts.length)return false;
 let pathEnd=0;let layerEnd=0;for(let row=0;row<positions.length;row=row+11){if((positions[row+5]??-1)!==pathEnd||(positions[row+7]??-1)!==layerEnd||(positions[row+8]??-1)!==(positions[row+9]??-1))return false;pathEnd=pathEnd+(positions[row+6]??-1);layerEnd=layerEnd+(positions[row+8]??-1)*2;}if(pathEnd!==paths.length||layerEnd!==layers.length)return false;
 for(let id=1;id<kinds.length;id++){
  const kind=kinds[id]??-1;let occurrences=0;
  if(kind===1){for(let row=0;row<tokens.length;row=row+9)if((tokens[row]??-1)===id)occurrences=occurrences+1;}
  else if(kind===2){for(let row=0;row<declarations.length;row=row+${width('declaration')})if((declarations[row]??-1)===id)occurrences=occurrences+1;}
  else if(kind===3){for(let row=0;row<parameters.length;row=row+8)if((parameters[row]??-1)===id)occurrences=occurrences+1;}
  else if(kind===4){for(let row=0;row<declarations.length;row=row+${width('declaration')})if((declarations[row+4]??-1)===id)occurrences=occurrences+1;}
  else if(kind===5){for(let row=0;row<positions.length;row=row+11)if((positions[row]??-1)===id)occurrences=occurrences+1;}
  else if(kind===6){for(let row=0;row<cores.length;row=row+7)if((cores[row]??-1)===id)occurrences=occurrences+1;}
  else if(kind===7){for(let row=0;row<operations.length;row=row+6)if((operations[row]??-1)===id)occurrences=occurrences+1;}
  else if(kind===8)occurrences=1;
  else return false;
  if(occurrences!==1)return false;
 }
 let end=0;for(let id=1;id<names.length/2;id++){
  const start=names[id*2]??-1;const length=names[id*2+1]??-1;if(start!==end||length<0||(units[start+length]??-1)!==0)return false;end=start+length+1;
  if(id<=${p.symbols.length}){const expected=readTextCertificateKnownSymbol(id);if(expected.length!==length)return false;for(let offset=0;offset<length;offset++)if((units[start+offset]??-1)!==(expected[offset]??-1))return false;}
  for(let previous=1;previous<id;previous++){if((names[previous*2+1]??-1)===length){let equal=true;for(let offset=0;offset<length;offset++)if((units[start+offset]??-1)!==(units[(names[previous*2]??-1)+offset]??-1))equal=false;if(equal)return false;}}
 }return end===units.length;
}
/** @param {number[]} rows @param {number} width @param {number} identity @returns {number} */
export function findTextCertificateRow(rows,width,identity){let result=-1;for(let at=0;at<rows.length;at=at+width){if((rows[at]??-1)===identity){if(result>=0)return -1;result=at;}}return result;}
/** @param {number[]} values @returns {boolean} */
export function acceptTextCertificateNumbers(values){for(let index=0;index<values.length;index++){const value=values[index]??-2;if(!Number.isInteger(value)||value< -1||value>9007199254740991||value===0&&1/value<0)return false;}return true;}
/** @param {number[]} rows @param {number} width @param {number[]} kinds @param {number} kind @returns {boolean} */
export function acceptTextCertificateIdentities(rows,width,kinds,kind){if(rows.length%width!==0)return false;for(let at=0;at<rows.length;at=at+width){const id=rows[at]??-1;if(id<=0||(kinds[id]??-1)!==kind||findTextCertificateRow(rows,width,id)!==at)return false;}return true;}
/** @param {number} identity @param {number[]} names @param {number[]} units @returns {boolean} */
export function acceptTextCertificateName(identity,names,units){
 if(identity<=0||${p.forbiddenNames.map(name=>'identity==='+s(name)).join('||')})return false;
 const start=names[identity*2]??-1;const length=names[identity*2+1]??-1;if(start<0||length<=0||start+length>units.length)return false;
 if(length>=3&&(units[start]??-1)===109&&(units[start+1]??-1)===108&&(units[start+2]??-1)===95)return false;
 for(let index=0;index<length;index++){if(!acceptRustPureUnitNameCharacter(units[start+index]??-1,index===0))return false;}return true;
}
/** @param {number[]} positions @param {number[]} paths @param {number[]} layers @param {number} identity @param {number} signature @param {number} ordinal @returns {number} */
export function readTextCertificateDomain(positions,paths,layers,identity,signature,ordinal){
 const at=findTextCertificateRow(positions,${width('position')},identity);if(at<0)return 0;
 if(${read('positions','at','signature','position')}!==signature||${read('positions','at','ordinal','position')}!==ordinal||${read('positions','at','type','position')}!==${s('string')}||${read('positions','at','ownerArguments','position')}!==0)return 0;
 const count=${read('positions','at','layerCount','position')};const life=${read('positions','at','lifetimeCount','position')};const start=${read('positions','at','layerStart','position')};if(count!==life||count>1||start<0||start+count*2>layers.length)return 0;
 for(let index=0;index<count;index++){if((layers[start+index*2]??-1)!==${s('reference')}||(layers[start+index*2+1]??-1)!==0)return 0;}
 const path=${read('positions','at','pathStart','position')};const size=${read('positions','at','pathCount','position')};if(path<0||path+size>paths.length)return 0;
 const standard=${read('positions','at','ownerKind','position')}===${s('library')}&&size===3&&(paths[path]??-1)===${s('std')}&&(paths[path+1]??-1)===${s('string')}&&(paths[path+2]??-1)===${s('String')};
 const slice=${read('positions','at','ownerKind','position')}===${s('primitive')}&&size===1&&(paths[path]??-1)===${s('str')};
 const domain=readRustTextValueProjectionDomain(standard,slice,count);return domain===${JSON.stringify(p.symbols[s('owned standard String')-1])}?1:domain===${JSON.stringify(p.symbols[s('shared standard String')-1])}?2:domain===${JSON.stringify(p.symbols[s('shared primitive str')-1])}?3:0;
}
/** @param {number} domain @returns {number[]} */
export function readTextCertificateType(domain){${p.parameterTypes.map(row=>'if(domain==='+row.domain+')return ['+row.syntax.map(s).join(',')+'];').join('')}return [-1];}
/** @param {number} code @returns {number[]} */
export function readTextCertificatePrefix(code){${p.operations.map(row=>'if(code==='+row.code+')return ['+row.prefix.map(s).join(',')+'];').join('')}return [-1];}
/** @param {number} code @returns {number[]} */
export function readTextCertificateSuffix(code){${p.operations.map(row=>'if(code==='+row.code+')return ['+row.suffix.map(s).join(',')+'];').join('')}return [-1];}
/** @param {number} code @returns {string} */
export function readTextCertificateOperation(code){${p.operations.map(row=>'if(code==='+row.code+')return '+JSON.stringify(row.name)+';').join('')}return '';}
/** @param {number} code @returns {string} */
export function readTextCertificateDomainName(code){return code===1?'owned standard String':code===2?'shared standard String':code===3?'shared primitive str':code===4?'static primitive str literal':'';}
/** @param {number} code @returns {boolean} */
export function acceptTextCertificateAllocation(code){return ${p.operations.filter(row=>row.allocation).map(row=>'code==='+row.code).join('||')};}
/** @param {number[]} tokens @param {number} cursor @param {number[]} syntax @returns {number} */
export function matchTextCertificateSyntax(tokens,cursor,syntax){
 for(let index=0;index<syntax.length;index++){const at=(cursor+index)*${width('token')};const expected=syntax[index]??-1;if(expected<=0||at<0||at+${width('token')}>tokens.length||${read('tokens','at','syntax','token')}!==expected||${read('tokens','at','role','token')}!==0)return -1;
 const kind=${read('tokens','at','kind','token')};if(expected===${s('')}?kind!==${s('eof')}:expected===${s('pub')}||expected===${s('fn')}||expected===${s('main')}||expected===${s('String')}||expected===${s('str')}||expected===${s('from')}||expected===${s('clone')}||expected===${s('to_owned')}||expected===${s('to_string')}?kind!==${s('identifier')}:kind!==${s('punct')})return -1;
 }return cursor+syntax.length;
}
/** @param {number[]} tokens @param {number} cursor @param {number} role @param {number} target @param {number} value @param {number[]} names @param {number[]} units @returns {boolean} */
export function acceptTextCertificateToken(tokens,cursor,role,target,value,names,units){const at=cursor*${width('token')};return ${read('tokens','at','kind','token')}===${s('identifier')}&&${read('tokens','at','role','token')}===role&&${read('tokens','at','target','token')}===target&&${read('tokens','at','value','token')}===value&&acceptTextCertificateName(value,names,units)&&acceptTextCertificateName(${read('tokens','at','rawName','token')},names,units);}
/** @param {number[]} operations @param {number[]} parts @param {number} identity @param {number} code @param {number} start @param {number} end @param {number[]} expected @returns {boolean} */
export function acceptTextCertificateOperation(operations,parts,identity,code,start,end,expected){const at=findTextCertificateRow(operations,${width('operation')},identity);if(at<0||${read('operations','at','code','operation')}!==code||${read('operations','at','start','operation')}!==start||${read('operations','at','end','operation')}!==end||${read('operations','at','partCount','operation')}!==expected.length)return false;const offset=${read('operations','at','partStart','operation')};if(offset<0||offset+expected.length>parts.length)return false;for(let index=0;index<expected.length;index++){if((parts[offset+index]??-1)!==(expected[index]??-1))return false;}return true;}
/** @param {number[]} operations @param {number[]} parts @param {number} identity @param {number} code @param {number[]} tokens @param {number} start @param {number} end @param {number} nested @returns {boolean} */
export function acceptTextCertificateTokenOperation(operations,parts,identity,code,tokens,start,end,nested){const at=findTextCertificateRow(operations,${width('operation')},identity);if(at<0||end<=start||${read('operations','at','code','operation')}!==code||${read('operations','at','start','operation')}!==(tokens[start*${width('token')}+7]??-1)||${read('operations','at','end','operation')}!==(nested>0?(operations[findTextCertificateRow(operations,6,nested)+3]??-1):(tokens[(end-1)*${width('token')}+8]??-1))||${read('operations','at','partCount','operation')}!==end-start+(nested>0?1:0))return false;const offset=${read('operations','at','partStart','operation')};if(offset<0||offset+end-start>parts.length)return false;for(let index=start;index<end;index++){if((parts[offset+index-start]??-1)!==(tokens[index*${width('token')}]??-1))return false;}return nested<=0||(parts[offset+end-start]??-1)===nested;}
/** @param {number[]} conditions @param {number} ordinal @param {number} id @param {number} meaning @param {number} declaration @param {number} operation @param {number} core @param {number} input @param {number} position @returns {boolean} */
export function acceptTextCertificateCondition(conditions,ordinal,id,meaning,declaration,operation,core,input,position){const at=ordinal*${width('condition')};return (conditions[at]??-1)===id&&(conditions[at+1]??-1)===meaning&&(conditions[at+2]??-1)===declaration&&(conditions[at+3]??-1)===operation&&(conditions[at+4]??-1)===core&&(conditions[at+5]??-1)===input&&(conditions[at+6]??-1)===position;}
/** @param {number[]} context @param {number[]} kinds @param {number[]} tokens @param {number[]} declarations @param {number[]} parameters @param {number[]} positions @param {number[]} paths @param {number[]} layers @param {number[]} cores @param {number[]} functions @param {number[]} operations @param {number[]} parts @param {number[]} inputs @param {number[]} allocations @param {number[]} names @param {number[]} units @param {number[]} items @returns {number[]} */
export function checkRustTextCertificate(context,kinds,tokens,declarations,parameters,positions,paths,layers,cores,functions,operations,parts,inputs,allocations,names,units,items){
 if(!acceptTextCertificateNumbers(context)||!acceptTextCertificateNumbers(kinds)||!acceptTextCertificateNumbers(tokens)||!acceptTextCertificateNumbers(declarations)||!acceptTextCertificateNumbers(parameters)||!acceptTextCertificateNumbers(positions)||!acceptTextCertificateNumbers(paths)||!acceptTextCertificateNumbers(layers)||!acceptTextCertificateNumbers(cores)||!acceptTextCertificateNumbers(functions)||!acceptTextCertificateNumbers(operations)||!acceptTextCertificateNumbers(parts)||!acceptTextCertificateNumbers(inputs)||!acceptTextCertificateNumbers(allocations)||!acceptTextCertificateNumbers(names)||!acceptTextCertificateNumbers(units)||!acceptTextCertificateNumbers(items))return [-1,0,0,0,0];
 if(!acceptTextCertificateStorage(kinds,tokens,declarations,parameters,positions,paths,layers,cores,operations,parts,names,units))return [-20,0,0,0,0];
 if(!acceptTextCertificateIdentities(tokens,${width('token')},kinds,1)||!acceptTextCertificateIdentities(declarations,${width('declaration')},kinds,2)||!acceptTextCertificateIdentities(parameters,${width('parameter')},kinds,3)||!acceptTextCertificateIdentities(positions,${width('position')},kinds,5)||!acceptTextCertificateIdentities(cores,${width('core')},kinds,6)||!acceptTextCertificateIdentities(operations,${width('operation')},kinds,7))return [-2,0,0,0,0];
 if(context.length!==15||(context[0]??-1)!==1||(context[1]??-1)!==${s('Rust')}||(context[2]??-1)!==(context[6]??-1)||(kinds[context[2]??-1]??-1)!==8||(context[3]??-1)!==0||(context[7]??-1)!==${s(p.sourceModel)}||(context[8]??-1)!==${s('Rust')}||(context[9]??-1)!==${s(readProjectData(new URL('../rust-text-value-projection/data/policy.lino',import.meta.url)).observationModel)}||(context[11]??-1)!==0)return [-3,0,0,0,0];
 const count=declarations.length/${width('declaration')};if(count<1||functions.length!==count*${width('function')}||(context[4]??-1)!==count||(context[5]??-1)!==count||items.length!==count||cores.length!==count*${width('core')}||inputs.length%7!==0||allocations.length%7!==0||!acceptRustOwnedLocalTextUnits(units))return [-4,0,0,0,0];
 for(let index=0;index<units.length;index++){if((units[index]??-1)<0||(units[index]??-1)>65535)return [-4,0,0,0,0];}
 let cursor=0;let inputCount=0;let allocationCount=0;let parameterCount=0;const root=context[10]??-1;const rootAt=findTextCertificateRow(operations,6,root);if(rootAt<0||(operations[rootAt+1]??-1)!==9||(operations[rootAt+5]??-1)!==count+2)return [-5,0,0,0,0];const rootParts=operations[rootAt+4]??-1;
 for(let index=0;index<count;index++){
  const decl=index*${width('declaration')};const fact=index*${width('function')};const id=declarations[decl]??-1;const signature=declarations[decl+4]??-1;const core=declarations[decl+5]??-1;const coreAt=findTextCertificateRow(cores,7,core);const parameterStart=declarations[decl+6]??-1;const arity=declarations[decl+7]??-1;
  if((items[index]??-1)!==id||(functions[fact]??-1)!==id||(functions[fact+1]??-1)!==signature||(functions[fact+4]??-1)!==core||coreAt<0||(cores[coreAt+1]??-1)!==id||(cores[coreAt+3]??-1)!==${s('string')}||(declarations[decl+1]??-1)!==(declarations[decl+2]??-1)||(declarations[decl+3]??-1)!==0||(declarations[decl+8]??-1)!==${s('string')}||(declarations[decl+9]??-1)!==0||(declarations[decl+10]??-1)!==0||(declarations[decl+13]??-1)!==(declarations[decl+1]??-1)||(declarations[decl+14]??-1)!==${s(p.sourceSignatureModel)}||(declarations[decl+15]??-1)!==${s('Rust')}||(declarations[decl+16]??-1)!==0||(declarations[decl+17]??-1)!==${s('fn')}||(declarations[decl+18]??-1)!==arity||(declarations[decl+19]??-1)!==(functions[fact+2]??-1)||(kinds[signature]??-1)!==4||arity<0||arity>${p.maximumParameters}||parameterStart!==parameterCount*8)return [-6,cursor,index,inputCount,allocationCount];
  for(let previous=0;previous<index;previous++){if((declarations[previous*${width('declaration')}+1]??-1)===(declarations[decl+1]??-1))return [-6,cursor,index,inputCount,allocationCount];}
  const begin=cursor;if((tokens[cursor*9+2]??-1)===${s('pub')})cursor=matchTextCertificateSyntax(tokens,cursor,[${p.syntax.visibility.map(s).join(',')}]);const fnToken=cursor;cursor=matchTextCertificateSyntax(tokens,cursor,[${p.syntax.functionPrefix.map(s).join(',')}]);if(cursor<0||!acceptTextCertificateToken(tokens,cursor,1,id,declarations[decl+1]??-1,names,units))return [-7,cursor,index,inputCount,allocationCount];cursor=matchTextCertificateSyntax(tokens,cursor+1,[${p.syntax.parametersOpen.map(s).join(',')}]);if(cursor<0)return [-7,cursor,index,inputCount,allocationCount];
  for(let ordinal=0;ordinal<arity;ordinal++){
   const param=parameterStart+ordinal*8;const owner=parameters[param]??-1;const name=parameters[param+3]??-1;const position=parameters[param+7]??-1;const domain=readTextCertificateDomain(positions,paths,layers,position,signature,ordinal);
   if((parameters[param+1]??-1)!==id||(parameters[param+2]??-1)!==ordinal||(parameters[param+4]??-1)!==${s('string')}||(parameters[param+5]??-1)!==0||(parameters[param+6]??-1)!==0||!acceptTextCertificateToken(tokens,cursor,2,owner,name,names,units)||ordinal>0&&name===(parameters[parameterStart+3]??-1))return [-8,cursor,index,inputCount,allocationCount];
   cursor=matchTextCertificateSyntax(tokens,cursor+1,[${p.syntax.parameterType.map(s).join(',')}]);cursor=matchTextCertificateSyntax(tokens,cursor,readTextCertificateType(domain));if(cursor<0)return [-8,cursor,index,inputCount,allocationCount];if((tokens[cursor*9+2]??-1)===${s(',')})cursor=matchTextCertificateSyntax(tokens,cursor,[${p.syntax.parameterDelimiter.map(s).join(',')}]);else if(ordinal+1<arity)return [-8,cursor,index,inputCount,allocationCount];
  }
  parameterCount=parameterCount+arity;cursor=matchTextCertificateSyntax(tokens,cursor,[${p.syntax.result.map(s).join(',')}]);if(cursor<0||readTextCertificateDomain(positions,paths,layers,functions[fact+2]??-1,signature,-1)!==1)return [-9,cursor,index,inputCount,allocationCount];
  const open=cursor;cursor=matchTextCertificateSyntax(tokens,cursor,[${p.syntax.bodyOpen.map(s).join(',')}]);const leafStart=cursor;const operation=functions[fact+3]??-1;const opAt=findTextCertificateRow(operations,6,operation);const code=operations[opAt+1]??-1;if(opAt<0||code<1||code>5)return [-10,cursor,index,inputCount,allocationCount];cursor=matchTextCertificateSyntax(tokens,cursor,readTextCertificatePrefix(code));if(cursor<0)return [-10,cursor,index,inputCount,allocationCount];
  const atom=cursor;const token=cursor*9;const input=functions[fact+5]??-1;const position=functions[fact+6]??-1;const literal=functions[fact+7]??-1;let domain=0;
  if((tokens[token+1]??-1)===${s('string')}){if(input!==0||position!==0||literal!==core||(tokens[token+3]??-1)!==3||(tokens[token+4]??-1)!==core||(cores[coreAt+2]??-1)!==${s('lit')}||(tokens[token+5]??-1)<1||(tokens[token+5]??-1)>=names.length/2||(tokens[token+5]??-1)!==(cores[coreAt+4]??-1))return [-11,cursor,index,inputCount,allocationCount];domain=4;}
  else {const param=findTextCertificateRow(parameters,8,input);if(param<parameterStart||param>=parameterStart+arity*8||(parameters[param+7]??-1)!==position||literal!==0||(cores[coreAt+2]??-1)!==${s('var')}||!acceptTextCertificateToken(tokens,cursor,2,input,cores[coreAt+4]??-1,names,units)||(parameters[param+3]??-1)!==(cores[coreAt+4]??-1))return [-11,cursor,index,inputCount,allocationCount];domain=readTextCertificateDomain(positions,paths,layers,position,signature,parameters[param+2]??-1);}
  if(!acceptRustTextValueProjectionOperation(readTextCertificateOperation(code),readTextCertificateDomainName(domain))||(cores[coreAt+5]??-1)!==(tokens[token+7]??-1)||(cores[coreAt+6]??-1)!==(tokens[token+8]??-1))return [-12,cursor,index,inputCount,allocationCount];cursor=matchTextCertificateSyntax(tokens,cursor+1,readTextCertificateSuffix(code));const close=cursor;cursor=matchTextCertificateSyntax(tokens,cursor,[${p.syntax.bodyClose.map(s).join(',')}]);if(cursor<0)return [-13,cursor,index,inputCount,allocationCount];
  const functionId=parts[rootParts+index]??-1;const functionAt=findTextCertificateRow(operations,6,functionId);const functionParts=operations[functionAt+4]??-1;const bodyId=parts[functionParts+open-begin]??-1;
  if(functionAt<0||!acceptTextCertificateTokenOperation(operations,parts,functionId,6,tokens,begin,open,bodyId)||!acceptTextCertificateTokenOperation(operations,parts,operation,code,tokens,leafStart,close,0)||!acceptTextCertificateOperation(operations,parts,bodyId,7,tokens[open*9+7]??-1,tokens[close*9+8]??-1,[tokens[open*9]??-1,operation,tokens[close*9]??-1])||(declarations[decl+11]??-1)!==(tokens[fnToken*9+7]??-1)||(declarations[decl+12]??-1)!==(tokens[cursor*9+7]??-1))return [-14,cursor,index,inputCount,allocationCount];
  if(input>0){if(!acceptTextCertificateCondition(inputs,inputCount,${s(readProjectData(new URL('../rust-text-value-projection/data/policy.lino',import.meta.url)).inputAssumption.id)},${s(readProjectData(new URL('../rust-text-value-projection/data/policy.lino',import.meta.url)).inputAssumption.meaning)},id,operation,core,input,position))return [-15,cursor,index,inputCount,allocationCount];inputCount=inputCount+1;}
  if(acceptTextCertificateAllocation(code)){if(!acceptTextCertificateCondition(allocations,allocationCount,${s(readProjectData(new URL('../rust-text-value-projection/data/policy.lino',import.meta.url)).valueAssumption.id)},${s(readProjectData(new URL('../rust-text-value-projection/data/policy.lino',import.meta.url)).valueAssumption.meaning)},id,operation,core,input,position))return [-16,cursor,index,inputCount,allocationCount];allocationCount=allocationCount+1;}
 }
 const main=cursor;cursor=matchTextCertificateSyntax(tokens,cursor,[${p.syntax.main.map(s).join(',')}]);if(cursor<0||!acceptTextCertificateTokenOperation(operations,parts,parts[rootParts+count]??-1,8,tokens,main,cursor,0)||(context[13]??-1)!==(tokens[main*9+7]??-1)||(context[14]??-1)!==(tokens[main*9+8]??-1))return [-17,cursor,count,inputCount,allocationCount];
 const eof=cursor;cursor=matchTextCertificateSyntax(tokens,cursor,[${p.syntax.end.map(s).join(',')}]);if(cursor<0||cursor*9!==tokens.length||(kinds.length-1)!==(tokens.length/9+count*3+parameterCount+(parameterCount+count)+operations.length/6+1)||(parts[rootParts+count+1]??-1)!==(tokens[eof*9]??-1)||(context[12]??-1)!==(tokens[eof*9+8]??-1)||(operations[rootAt+2]??-1)!==(tokens[7]??-1)||(operations[rootAt+3]??-1)!==(tokens[eof*9+8]??-1)||parameterCount*8!==parameters.length||positions.length!==(parameterCount+count)*11||inputCount*7!==inputs.length||allocationCount*7!==allocations.length||operations.length!==(count*3+2)*6)return [-18,cursor,count,inputCount,allocationCount];
 return [1,cursor,count,inputCount,allocationCount];
}
`;}
export function bundledTextCertificateSource(){return readFileSync(new URL('../../js/src/translation/rust-pure-unit-call-rules.js',import.meta.url),'utf8')+'\n'+readFileSync(new URL('../rust-text-value-projection/rules.mjs',import.meta.url),'utf8')+'\n'+rustOwnedLocalStatePortableRules()+'\n'+textCertificateSource().replace(dependency,'');}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 mkdirSync(new URL('./data/',import.meta.url),{recursive:true});const check=process.argv.includes('--check'),source=textCertificateSource();
 for(const[file,value]of[[new URL('./data/policy.lino',import.meta.url),p]]){if(check)checkProjectData(file,value);else writeProjectData(file,value);}
 const file=new URL('./rules.mjs',import.meta.url);if(check){if(readFileSync(file,'utf8')!==source)throw new Error('portable certificate source is stale');}else writeFileSync(file,source);
 const checked=checkProgram(parseJavaScript(bundledTextCertificateSource())),rust=emitRust(checked),artifact={model:p.model,declarations:checked.items.map(row=>row.fullName),definitions:rust.definitions,completeRust:rust.text};
 if(check)checkProjectData(new URL('./data/translation.lino',import.meta.url),artifact);else writeProjectData(new URL('./data/translation.lino',import.meta.url),artifact);
 console.log('Portable text certificate: complete generated VM and shared dependencies translated automatically ('+checked.items.length+' declarations).');
}
