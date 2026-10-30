// Source-defined constant evaluation over actual primitive fact columns.
import {readRequiredNumberFact,readRequiredTextFact} from './fact-column-rules.js';
import {evaluateRustConstantIntegerOperation,evaluateRustConstantBooleanOperation,requireRustConstantBooleanRight,readRustConstantPatternEvaluationError,readRustConstantPrimitiveAliasOwner} from './rust-constant-integer-rules.js';
/** @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {string} name @returns {number} */
function constantField(starts,counts,labels,targets,node,name){if(node<0)return -1;const start=readRequiredNumberFact(starts,node),count=readRequiredNumberFact(counts,node);for(let at=start;at<start+count;at+=1)if(readRequiredTextFact(labels,at)===name)return readRequiredNumberFact(targets,at);return -1;}
/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {string} name @returns {string} */
function constantText(texts,starts,counts,labels,targets,node,name){const child=constantField(starts,counts,labels,targets,node,name);return child<0?'':readRequiredTextFact(texts,child);}
/** @param {number[]} starts @param {number[]} counts @param {number[]} targets @param {number} node @param {number} index @returns {number} */
function constantElement(starts,counts,targets,node,index){return node<0||index<0||index>=readRequiredNumberFact(counts,node)?-1:readRequiredNumberFact(targets,readRequiredNumberFact(starts,node)+index);}
/** @param {string} owner @returns {number} */
function constantBits(owner){return owner==='u8'||owner==='i8'?8:owner==='u16'||owner==='i16'?16:owner==='u32'||owner==='i32'?32:owner==='u64'||owner==='i64'||owner==='usize'||owner==='isize'?64:owner==='u128'||owner==='i128'?128:0;}
/** @param {string} owner @returns {string[]} */
function constantDomain(owner){const bits=constantBits(owner);if(bits===0)return [];let modulus=1n;for(let at=0;at<bits;at+=1)modulus*=2n;const signed=owner.startsWith('i');return [String(signed?-(modulus/2n):0n),String(signed?modulus/2n-1n:modulus-1n)];}
/** Exact integer source spelling, including nondecimal bases. @param {string} text @param {boolean} negative @returns {string[]} */
export function readRustSourceConstantIntegerLiteral(text,negative){
 let digits=text,base=10n,limit=10,negated=negative;if(digits.startsWith('-')){digits=digits.slice(1);negated=!negated;}
 if((digits.startsWith('0x')||digits.startsWith('0X'))){digits=digits.slice(2);base=16n;limit=16;}else if((digits.startsWith('0o')||digits.startsWith('0O'))){digits=digits.slice(2);base=8n;limit=8;}else if((digits.startsWith('0b')||digits.startsWith('0B'))){digits=digits.slice(2);base=2n;limit=2;}
 if(digits==='')return ['error','a constant integer literal needs its actual integer spelling'];
 let value=0n;
 for(let at=0;at<digits.length;at+=1){let digit='0123456789abcdefABCDEF'.indexOf(digits.charAt(at));if(digit>=16)digit-=6;if(digit<0||digit>=limit)return ['error','a constant integer literal needs its actual integer spelling'];let add=0n;for(let index=0;index<digit;index+=1)add+=1n;value=value*base+add;}
 return ['integer',String(negated?-value:value)];
}
/** @param {string[]} kinds @param {string[]} values @param {number} first @returns {number} */
export function readRustSourceConstantInitializerBoundary(kinds,values,first){
 if(kinds.length!==values.length||!Number.isInteger(first)||first<0||first>=values.length)return -1;
 let depth=0;for(let at=first;at<values.length;at+=1){if(readRequiredTextFact(kinds,at)!=='punct')continue;const token=readRequiredTextFact(values,at);if(token===';'&&depth===0)return at;if(token==='('||token==='['||token==='{')depth+=1;else if(token===')'||token===']'||token==='}'){if(depth===0)return -1;depth-=1;}}
 return -1;
}
/** @param {string} operation @returns {string} */
function constantOperation(operation){return operation==='conjoinRustPrimitiveBits'?'bitand':operation==='disjoinRustPrimitiveBits'?'bitor':operation==='excludeEqualRustPrimitiveBits'?'bitxor':operation==='complementRustPrimitiveBits'?'bitnot':operation==='shiftRustPrimitiveBitsLeft'||operation==='readCheckedRustLeftShift'?'shl':operation==='shiftRustPrimitiveBitsRight'||operation==='readCheckedRustRightShift'?'shr':operation;}
/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} rows @param {string} owner @returns {number} */
function constantRow(texts,starts,counts,labels,targets,rows,owner){if(rows<0)return -1;for(let at=0;at<readRequiredNumberFact(counts,rows);at+=1){const row=constantElement(starts,counts,targets,rows,at);if(constantText(texts,starts,counts,labels,targets,row,'owner')===owner)return row;}return -1;}
/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {string} fallback @returns {string} */
function constantTypeOwner(texts,starts,counts,labels,targets,node,fallback){
 const explicit=constantText(texts,starts,counts,labels,targets,node,'sourceIntegerOwner');
 let type=constantField(starts,counts,labels,targets,node,'type');if(type<0)type=constantField(starts,counts,labels,targets,node,'to');
 const kind=constantText(texts,starts,counts,labels,targets,type,'kind');if(kind==='bool')return 'bool';
 if(kind!=='fixed')return fallback;
 const bits=constantText(texts,starts,counts,labels,targets,type,'bits'),signed=constantText(texts,starts,counts,labels,targets,type,'signed')==='true';
 if(explicit!=='')return readRustConstantPrimitiveAliasOwner(kind,'primitive',explicit,0,0,bits,signed)!==''?explicit:'@invalid';
 const inherited=constantBits(fallback),width=bits==='8'?8:bits==='16'?16:bits==='32'?32:bits==='64'?64:bits==='128'?128:0;
 return inherited===width&&width>0&&fallback.startsWith('i')===signed?fallback:(signed?'i':'u')+bits;
}
/** Probe types without evaluating values or branches.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} rows @param {number} node @param {string[]} names @param {string[]} values @param {number} depth @returns {string} */
function constantHint(texts,starts,counts,labels,targets,rows,node,names,values,depth){
 if(node<0||depth>256)return '';
 const kind=constantText(texts,starts,counts,labels,targets,node,'k'),typed=constantTypeOwner(texts,starts,counts,labels,targets,node,'');if(typed!=='')return typed;
 if(kind==='bool')return 'bool';if(kind==='str')return 'str';
 if(kind==='name'||kind==='var'||kind==='call'){
  const path=constantField(starts,counts,labels,targets,node,'path'),head=constantElement(starts,counts,targets,path,0),name=kind==='var'?constantText(texts,starts,counts,labels,targets,node,'name'):head<0?'':readRequiredTextFact(texts,head),bound=kind==='var'||path>=0&&readRequiredNumberFact(counts,path)===1?names.lastIndexOf(name):-1;
  if(bound>=0)return readRequiredTextFact(values,bound*3+1);
  const reference=constantText(texts,starts,counts,labels,targets,node,'sourceConstantOwner'),row=constantRow(texts,starts,counts,labels,targets,rows,reference!==''?reference:kind==='call'?constantText(texts,starts,counts,labels,targets,node,'fn'):'');return constantText(texts,starts,counts,labels,targets,row,'domain');
 }
 if(kind==='unary')return constantHint(texts,starts,counts,labels,targets,rows,constantField(starts,counts,labels,targets,node,'arg'),names,values,depth+1);
 if(kind==='binary'){const left=constantHint(texts,starts,counts,labels,targets,rows,constantField(starts,counts,labels,targets,node,'left'),names,values,depth+1);return left!==''?left:constantHint(texts,starts,counts,labels,targets,rows,constantField(starts,counts,labels,targets,node,'right'),names,values,depth+1);}
 return '';
}
/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} node @param {string} expected @returns {string[]} */
function constantLiteral(texts,starts,counts,labels,targets,node,expected){
 const kind=constantText(texts,starts,counts,labels,targets,node,'k');
 if(kind==='bool')return ['boolean','bool',constantText(texts,starts,counts,labels,targets,node,'value')];
 if(kind==='str')return ['string','str',constantText(texts,starts,counts,labels,targets,node,'value')];
 const literalType=constantField(starts,counts,labels,targets,node,'type'),literalKind=constantText(texts,starts,counts,labels,targets,literalType,'kind');
 if(kind==='lit'&&literalKind==='bool')return ['boolean','bool',constantText(texts,starts,counts,labels,targets,node,'value')];
 if(kind==='lit'&&literalKind==='string')return ['string','str',constantText(texts,starts,counts,labels,targets,node,'value')];
 if(kind==='num'||kind==='lit'){
  const owner=constantTypeOwner(texts,starts,counts,labels,targets,node,constantBits(expected)>0?expected:'i32'),domain=constantDomain(owner),literal=readRustSourceConstantIntegerLiteral(constantText(texts,starts,counts,labels,targets,node,'value'),constantText(texts,starts,counts,labels,targets,node,'negative')==='true');
  if(readRequiredTextFact(literal,0)==='error')return literal;if(domain.length!==2)return ['error','a constant numeric expression needs its actual integer owner'];
  const checked=evaluateRustConstantIntegerOperation('add',readRequiredTextFact(literal,1),'0',readRequiredTextFact(domain,0),readRequiredTextFact(domain,1),constantBits(owner));return readRequiredTextFact(checked,0)==='error'?checked:['integer',owner,readRequiredTextFact(checked,1)];
 }
 return ['error','a constant literal needs its actual scalar source type'];
}
/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} rows @param {number} node @param {string} expected @param {string[]} names @param {string[]} values @param {string[]} visiting @param {number} depth @param {number} fuel @returns {string[]} */
function constantCast(texts,starts,counts,labels,targets,rows,node,expected,names,values,visiting,depth,fuel){
  const target=constantTypeOwner(texts,starts,counts,labels,targets,node,expected),domain=constantDomain(target),arg=constantField(starts,counts,labels,targets,node,'arg'),hint=constantHint(texts,starts,counts,labels,targets,rows,arg,names,values,0),value=constantExpression(texts,starts,counts,labels,targets,rows,arg,hint!==''?hint:'i32',names,values,visiting,depth+1,fuel-1);
  if(readRequiredTextFact(value,0)==='error')return value;if(readRequiredTextFact(value,0)!=='integer'&&readRequiredTextFact(value,0)!=='boolean'||domain.length!==2)return ['error','a constant cast requires actual integer source and target owners'];
  const result=evaluateRustConstantIntegerOperation('cast',readRequiredTextFact(value,0)==='boolean'?readRequiredTextFact(value,2)==='true'?'1':'0':readRequiredTextFact(value,2),'0',readRequiredTextFact(domain,0),readRequiredTextFact(domain,1),constantBits(target));return readRequiredTextFact(result,0)==='error'?result:['integer',target,readRequiredTextFact(result,1)];
}
/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} rows @param {number} node @param {string} expected @param {string[]} names @param {string[]} values @param {string[]} visiting @param {number} depth @param {number} fuel @returns {string[]} */
function constantOptionalChoice(texts,starts,counts,labels,targets,rows,node,expected,names,values,visiting,depth,fuel){
 const left=constantExpression(texts,starts,counts,labels,targets,rows,constantField(starts,counts,labels,targets,node,'left'),expected,names,values,visiting,depth+1,Math.floor((fuel-1)/2)),kind=readRequiredTextFact(left,0);
 if(kind==='error')return left;
 if(kind==='some'){const owner=readRequiredTextFact(left,1);return [owner==='bool'?'boolean':owner==='str'?'string':'integer',owner,readRequiredTextFact(left,2)];}
 if(kind==='none')return constantExpression(texts,starts,counts,labels,targets,rows,constantField(starts,counts,labels,targets,node,'right'),expected,names,values,visiting,depth+1,Math.floor((fuel-1)/2));
 return ['error','a constant optional choice requires its actual optional value'];
}
/** Scalar result: kind, actual primitive owner, canonical value; errors: error, reason.
 * @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} rows @param {number} node @param {string} expected @param {string[]} names @param {string[]} values @param {string[]} visiting @param {number} depth @param {number} fuel @returns {string[]} */
function constantExpression(texts,starts,counts,labels,targets,rows,node,expected,names,values,visiting,depth,fuel){
 const budget=readRustConstantPatternEvaluationError(depth,65536-fuel,false);if(node<0||fuel<=0||budget!=='')return ['error',budget!==''?budget:'constant pattern evaluation exceeds its source resource budget'];
 const kind=constantText(texts,starts,counts,labels,targets,node,'k');
 if(kind==='bool'||kind==='str'||kind==='num'||kind==='lit')return constantLiteral(texts,starts,counts,labels,targets,node,expected);
 if(kind==='name'||kind==='var'||kind==='call'){
  const path=constantField(starts,counts,labels,targets,node,'path'),head=constantElement(starts,counts,targets,path,0),name=kind==='var'?constantText(texts,starts,counts,labels,targets,node,'name'):head<0?'':readRequiredTextFact(texts,head),bound=kind==='var'||path>=0&&readRequiredNumberFact(counts,path)===1?names.lastIndexOf(name):-1;
  if(bound>=0)return [readRequiredTextFact(values,bound*3),readRequiredTextFact(values,bound*3+1),readRequiredTextFact(values,bound*3+2)];
  const reference=constantText(texts,starts,counts,labels,targets,node,'sourceConstantOwner'),owner=reference!==''?reference:kind==='call'?constantText(texts,starts,counts,labels,targets,node,'fn'):'',row=constantRow(texts,starts,counts,labels,targets,rows,owner);
  const args=constantField(starts,counts,labels,targets,node,'args');if(kind==='call'&&args>=0&&readRequiredNumberFact(counts,args)!==0)return ['error','constant function calls need their actual const source interpretation'];
  if(row<0)return ['error','a constant expression name needs its actual source constant or local binding'];
  if(visiting.includes(owner))return ['error',readRustConstantPatternEvaluationError(depth,0,true)];
  const domain=constantText(texts,starts,counts,labels,targets,row,'domain'),result=constantExpression(texts,starts,counts,labels,targets,rows,constantField(starts,counts,labels,targets,row,'body'),domain,[],[],visiting.concat([owner]),depth+1,fuel-1);
  return readRequiredTextFact(result,0)==='error'?result:readRequiredTextFact(result,1)!==domain?['error','a constant initializer must retain its actual declared primitive owner']:result;
 }
 if(kind==='let'){
  const value=constantExpression(texts,starts,counts,labels,targets,rows,constantField(starts,counts,labels,targets,node,'value'),expected,names,values,visiting,depth+1,Math.floor((fuel-1)/2));if(readRequiredTextFact(value,0)==='error')return value;
  return constantExpression(texts,starts,counts,labels,targets,rows,constantField(starts,counts,labels,targets,node,'body'),expected,names.concat([constantText(texts,starts,counts,labels,targets,node,'name')]),values.concat(value),visiting,depth+1,Math.floor((fuel-1)/2));
 }
 if(kind==='if'){
  const condition=constantExpression(texts,starts,counts,labels,targets,rows,constantField(starts,counts,labels,targets,node,'cond'),'bool',names,values,visiting,depth+1,Math.floor((fuel-1)/2));if(readRequiredTextFact(condition,0)==='error')return condition;if(readRequiredTextFact(condition,0)!=='boolean')return ['error','a constant condition requires its actual Boolean value'];
  return constantExpression(texts,starts,counts,labels,targets,rows,constantField(starts,counts,labels,targets,node,readRequiredTextFact(condition,2)==='true'?'then':'else'),expected,names,values,visiting,depth+1,Math.floor((fuel-1)/2));
 }
 if(kind==='coalesce')return constantOptionalChoice(texts,starts,counts,labels,targets,rows,node,expected,names,values,visiting,depth,fuel);
 if(kind==='abort')return ['error','the constant initializer aborts: '+constantText(texts,starts,counts,labels,targets,node,'message')];
 if(kind==='cast')return constantCast(texts,starts,counts,labels,targets,rows,node,expected,names,values,visiting,depth,fuel);
 if(kind==='unary'||kind==='binary'||kind==='math'){
  const args=constantField(starts,counts,labels,targets,node,'args'),first=constantElement(starts,counts,targets,args,0),second=constantElement(starts,counts,targets,args,1),left=kind==='math'?constantText(texts,starts,counts,labels,targets,first,'k')!==''?first:constantField(starts,counts,labels,targets,first,'value'):constantField(starts,counts,labels,targets,node,kind==='unary'?'arg':'left'),right=kind==='math'?constantText(texts,starts,counts,labels,targets,second,'k')!==''?second:constantField(starts,counts,labels,targets,second,'value'):constantField(starts,counts,labels,targets,node,'right');
  let operation=constantOperation(constantText(texts,starts,counts,labels,targets,node,'op'));
  const leftHint=constantHint(texts,starts,counts,labels,targets,rows,left,names,values,0),rightHint=constantHint(texts,starts,counts,labels,targets,rows,right,names,values,0),operand=leftHint!==''?leftHint:rightHint!==''?rightHint:constantBits(expected)>0?expected:'i32';
  if(operation==='neg'&&(constantText(texts,starts,counts,labels,targets,left,'k')==='num'||constantText(texts,starts,counts,labels,targets,left,'k')==='lit')){
   const owner=constantTypeOwner(texts,starts,counts,labels,targets,left,operand),domain=constantDomain(owner);if(!owner.startsWith('i')||domain.length!==2)return ['error','an unsigned constant integer cannot use unary negation'];
   const literal=readRustSourceConstantIntegerLiteral(constantText(texts,starts,counts,labels,targets,left,'value'),constantText(texts,starts,counts,labels,targets,left,'negative')!=='true');if(readRequiredTextFact(literal,0)==='error')return literal;
   const result=evaluateRustConstantIntegerOperation('add',readRequiredTextFact(literal,1),'0',readRequiredTextFact(domain,0),readRequiredTextFact(domain,1),constantBits(owner));return readRequiredTextFact(result,0)==='error'?result:['integer',owner,readRequiredTextFact(result,1)];
  }
  const unaryInput=kind==='unary'||operation==='bitnot';
  const a=constantExpression(texts,starts,counts,labels,targets,rows,left,operand,names,values,visiting,depth+1,unaryInput?fuel-1:Math.floor((fuel-1)/2));if(readRequiredTextFact(a,0)==='error')return a;
  if(readRequiredTextFact(a,0)==='boolean'&&!requireRustConstantBooleanRight(operation,readRequiredTextFact(a,2)==='true'))return a;
  if(operation==='not'&&readRequiredTextFact(a,0)==='integer')operation='bitnot';
  const unary=unaryInput,b=unary?['integer',readRequiredTextFact(a,1),'0']:constantExpression(texts,starts,counts,labels,targets,rows,right,operation==='shl'||operation==='shr'?rightHint!==''?rightHint:'i32':readRequiredTextFact(a,1),names,values,visiting,depth+1,Math.floor((fuel-1)/2));if(readRequiredTextFact(b,0)==='error')return b;
  if(readRequiredTextFact(a,0)==='boolean'&&(unary||readRequiredTextFact(b,0)==='boolean')){const result=evaluateRustConstantBooleanOperation(operation,readRequiredTextFact(a,2)==='true',readRequiredTextFact(b,2)==='true');return readRequiredTextFact(result,0)==='error'?result:['boolean','bool',readRequiredTextFact(result,1)];}
  if(readRequiredTextFact(a,0)!=='integer'||readRequiredTextFact(b,0)!=='integer'||!unary&&operation!=='shl'&&operation!=='shr'&&readRequiredTextFact(a,1)!==readRequiredTextFact(b,1))return ['error','constant operands require compatible actual primitive owners'];
  const owner=readRequiredTextFact(a,1),domain=constantDomain(owner);if(domain.length!==2)return ['error','a constant numeric expression needs its actual integer owner'];
  const result=evaluateRustConstantIntegerOperation(operation,readRequiredTextFact(a,2),readRequiredTextFact(b,2),readRequiredTextFact(domain,0),readRequiredTextFact(domain,1),constantBits(owner));
  const original=constantText(texts,starts,counts,labels,targets,node,'op');
  if(original==='readCheckedRustLeftShift'||original==='readCheckedRustRightShift'){
   if(readRequiredTextFact(result,0)==='error')return readRequiredTextFact(result,1)==='overflow in a constant integer shift'?['none',owner,'']:result;
   return ['some',owner,readRequiredTextFact(result,1)];
  }
  return readRequiredTextFact(result,0)==='error'?result:[readRequiredTextFact(result,0),readRequiredTextFact(result,0)==='boolean'?'bool':owner,readRequiredTextFact(result,1)];
 }
 return ['error','the constant initializer expression needs its source evaluation interpretation'];
}
/** @param {string[]} texts @param {number[]} starts @param {number[]} counts @param {string[]} labels @param {number[]} targets @param {number} root @param {string} owner @returns {string[]} */
export function evaluateRustSourceConstant(texts,starts,counts,labels,targets,root,owner){
 const rows=constantField(starts,counts,labels,targets,root,'constants'),row=constantRow(texts,starts,counts,labels,targets,rows,owner);if(row<0)return ['error','a constant pattern needs its actual source declaration'];
 const domain=constantText(texts,starts,counts,labels,targets,row,'domain'),result=constantExpression(texts,starts,counts,labels,targets,rows,constantField(starts,counts,labels,targets,row,'body'),domain,[],[],[owner],0,65536);
 return readRequiredTextFact(result,0)==='error'?result:readRequiredTextFact(result,1)!==domain?['error','a constant initializer must retain its actual declared primitive owner']:result;
}
