// A distinct scope interpretation: intrinsic allocation and deferred bodies
// cannot introduce an ambient source callback during initializer evaluation.
// Original initializer translation, abrupt completion and signatures remain
// independent obligations. The preceding inert-value rules remain unchanged.
import {readRequiredNumberFact,readRequiredTextFact,readRequiredBooleanFact} from './fact-column-rules.js';
import {acceptInertModuleBindingScope} from './inert-module-binding-rules.js';
/** @param {string[]} terms @param {number[]} parents @param {number} owner @returns {number[]} */
function childrenOfIntrinsicInitializer(terms,parents,owner){
 let result=[];
 for(let at=0;at<terms.length;at+=1)if(readRequiredNumberFact(parents,at)===owner&&readRequiredTextFact(terms,at)!=='comment'&&readRequiredTextFact(terms,at)!=='whitespace')result=result.concat([at]);
 return result;
}
/** @param {boolean[]} values @param {number} count @param {number} at @returns {boolean} */
function intrinsicInitializerValue(values,count,at){return at>=0&&at<count&&readRequiredBooleanFact(values,count-1-at);}
/** @param {string[]} terms @param {number[]} parents @param {boolean[]} values
 * @param {number} owner @param {string} open @param {string} close @param {boolean} object @returns {boolean} */
function intrinsicInitializerSequence(terms,parents,values,owner,open,close,object){
 const children=childrenOfIntrinsicInitializer(terms,parents,owner);
 if(children.length<2||readRequiredTextFact(terms,readRequiredNumberFact(children,0))!==open||readRequiredTextFact(terms,readRequiredNumberFact(children,children.length-1))!==close)return false;
 let expectValue=true;
 for(let index=1;index<children.length-1;index+=1){
  const child=readRequiredNumberFact(children,index),term=readRequiredTextFact(terms,child);
  if(expectValue){if(!intrinsicInitializerValue(values,terms.length,child)||object&&term!=='pair'&&term!=='method_definition')return false;}
  else if(term!==',')return false;
  expectValue=!expectValue;
 }
 return true;
}
/** @param {string[]} terms @param {string[]} fields @param {number[]} parents @param {number} owner @returns {boolean} */
function intrinsicInitializerDeferredBody(terms,fields,parents,owner){
 const kind=readRequiredTextFact(terms,owner);let parameters=0,body=0,names=0;
 for(const child of childrenOfIntrinsicInitializer(terms,parents,owner)){
  const term=readRequiredTextFact(terms,child),field=readRequiredTextFact(fields,child);
  if(field==='parameters'&&term==='formal_parameters'||field==='parameter'&&term==='identifier')parameters+=1;
  else if(field==='body')body+=1;
  else if(field==='name'){
   if(kind==='method_definition'&&term!=='property_identifier'&&term!=='string'&&term!=='number'||kind!=='method_definition'&&term!=='identifier')return false;
   names+=1;
  }else if(term!=='function'&&term!=='=>'&&term!=='async'&&term!=='*'&&term!=='get'&&term!=='set')return false;
 }
 return parameters===1&&body===1&&names<2&&(kind!=='method_definition'||names===1);
}
/** Complete parser-owned preorder facts. Result [actual name, required owner:
 * 0=intrinsics only,1=Map,2=Set,3=Object]. This is no executable translation.
 * @param {string[]} terms @param {string[]} fields @param {number[]} parents
 * @param {number[]} starts @param {number[]} ends @param {string[]} texts
 * @param {number[]} errors @param {number} sourceBytes @returns {number[]} */
export function readIntrinsicModuleInitializer(terms,fields,parents,starts,ends,texts,errors,sourceBytes){
 const count=terms.length;
 if(count<1||count>4096||fields.length!==count||parents.length!==count||starts.length!==count||ends.length!==count||texts.length!==count||errors.length!==count||!Number.isInteger(sourceBytes)||sourceBytes<0||sourceBytes>65536)return [];
 for(let at=0;at<count;at+=1){
  const parent=readRequiredNumberFact(parents,at),start=readRequiredNumberFact(starts,at),end=readRequiredNumberFact(ends,at);
  if(!Number.isInteger(start)||start<0||!Number.isInteger(end)||end<start||end>sourceBytes||readRequiredNumberFact(errors,at)!==0)return [];
  if(at===0){if(parent!==-1||readRequiredTextFact(terms,at)!=='program'||end!==sourceBytes)return [];}
  else if(!Number.isInteger(parent)||parent<0||parent>=at||start<readRequiredNumberFact(starts,parent)||end>readRequiredNumberFact(ends,parent))return [];
 }
 const top=childrenOfIntrinsicInitializer(terms,parents,0);if(top.length!==1)return [];
 let declaration=readRequiredNumberFact(top,0);
 if(readRequiredTextFact(terms,declaration)==='export_statement'){
  let selected=-1,exports=0;
  for(const child of childrenOfIntrinsicInitializer(terms,parents,declaration)){
   const term=readRequiredTextFact(terms,child);
   if(term==='export')exports+=1;else if(term==='lexical_declaration'&&selected<0)selected=child;else if(term!==';')return [];
  }
  if(exports!==1||selected<0)return [];declaration=selected;
 }
 if(readRequiredTextFact(terms,declaration)!=='lexical_declaration')return [];
 let binding=-1,constants=0;
 for(const child of childrenOfIntrinsicInitializer(terms,parents,declaration)){
  const term=readRequiredTextFact(terms,child);
  if(term==='const')constants+=1;else if(term==='variable_declarator'&&binding<0)binding=child;else if(term!==';')return [];
 }
 if(constants!==1||binding<0)return [];
 let name=-1,value=-1,equals=0;
 for(const child of childrenOfIntrinsicInitializer(terms,parents,binding)){
  const term=readRequiredTextFact(terms,child),field=readRequiredTextFact(fields,child);
  if(field==='name'&&term==='identifier'&&name<0)name=child;else if(field==='value'&&value<0)value=child;else if(term==='=')equals+=1;else return [];
 }
 if(name<0||value<0||equals!==1||readRequiredTextFact(texts,name).length===0||readRequiredTextFact(texts,name).includes('\\'))return [];
 let values=[];
 for(let at=count-1;at>=0;at-=1){
  const term=readRequiredTextFact(terms,at),children=childrenOfIntrinsicInitializer(terms,parents,at);let admitted=false;
  if(term==='string'||term==='number'||term==='true'||term==='false'||term==='null'||term==='regex')admitted=true;
  else if(term==='array')admitted=intrinsicInitializerSequence(terms,parents,values,at,'[',']',false);
  else if(term==='object')admitted=intrinsicInitializerSequence(terms,parents,values,at,'{','}',true);
  else if(term==='function_expression'||term==='generator_function'||term==='arrow_function'||term==='method_definition')admitted=intrinsicInitializerDeferredBody(terms,fields,parents,at);
  else if(term==='pair'){
   let key=-1,item=-1,colons=0;
   for(const child of children){const field=readRequiredTextFact(fields,child),part=readRequiredTextFact(terms,child);
    if(field==='key'&&key<0&&(part==='property_identifier'||part==='string'||part==='number'))key=child;
    else if(field==='value'&&item<0)item=child;else if(part===':')colons+=1;else{key=-2;break;}}
   admitted=key>=0&&item>=0&&colons===1&&intrinsicInitializerValue(values,count,item);
  }else if(term==='parenthesized_expression'&&children.length===3){
   admitted=readRequiredTextFact(terms,readRequiredNumberFact(children,0))==='('&&readRequiredTextFact(terms,readRequiredNumberFact(children,2))===')'&&intrinsicInitializerValue(values,count,readRequiredNumberFact(children,1));
  }else if(term==='unary_expression'&&children.length===2){
   const operation=readRequiredTextFact(terms,readRequiredNumberFact(children,0));
   admitted=(operation==='+'||operation==='-')&&readRequiredTextFact(terms,readRequiredNumberFact(children,1))==='number';
  }
  values=values.concat([admitted]);
 }
 if(intrinsicInitializerValue(values,count,value))return [name,0];
 const kind=readRequiredTextFact(terms,value),children=childrenOfIntrinsicInitializer(terms,parents,value);
 let callee=-1,argumentsAt=-1,news=0;
 for(const child of children){const field=readRequiredTextFact(fields,child),term=readRequiredTextFact(terms,child);
  if((field==='constructor'&&term==='identifier'||field==='function'&&term==='member_expression')&&callee<0)callee=child;
  else if(field==='arguments'&&term==='arguments'&&argumentsAt<0)argumentsAt=child;else if(term==='new')news+=1;else return [];
 }
 if(callee<0||argumentsAt<0)return [];
 const argumentsChildren=childrenOfIntrinsicInitializer(terms,parents,argumentsAt);
 if(argumentsChildren.length!==3&&argumentsChildren.length!==4)return [];
 const first=readRequiredNumberFact(argumentsChildren,0),argument=readRequiredNumberFact(argumentsChildren,1),last=readRequiredNumberFact(argumentsChildren,argumentsChildren.length-1);
 if(readRequiredTextFact(terms,first)!=='('||readRequiredTextFact(terms,last)!==')'||!intrinsicInitializerValue(values,count,argument))return [];
 if(argumentsChildren.length===4&&readRequiredTextFact(terms,readRequiredNumberFact(argumentsChildren,2))!==',')return [];
 if(kind==='new_expression'&&news===1){
  if(readRequiredTextFact(terms,argument)!=='array')return [];
  const owner=readRequiredTextFact(texts,callee);
  if(owner==='Set')return [name,2];
  if(owner!=='Map')return [];
  for(const child of childrenOfIntrinsicInitializer(terms,parents,argument)){
   const term=readRequiredTextFact(terms,child);if(term==='['||term===']'||term===',')continue;
   if(term!=='array')return [];
   const pair=childrenOfIntrinsicInitializer(terms,parents,child);if(pair.length!==5&&pair.length!==6)return [];
  }
  return [name,1];
 }
 if(kind!=='call_expression'||news!==0)return [];
 const member=childrenOfIntrinsicInitializer(terms,parents,callee);if(member.length!==3)return [];
 const object=readRequiredNumberFact(member,0),dot=readRequiredNumberFact(member,1),property=readRequiredNumberFact(member,2);
 if(readRequiredTextFact(terms,object)!=='identifier'||readRequiredTextFact(texts,object)!=='Object'||readRequiredTextFact(terms,dot)!=='.'||readRequiredTextFact(terms,property)!=='property_identifier'||readRequiredTextFact(texts,property)!=='freeze')return [];
 if(readRequiredTextFact(terms,argument)!=='array'&&readRequiredTextFact(terms,argument)!=='object')return [];
 return [name,3];
}
/** Independent scope admission; all previous unique-owner and shadowing
 * decisions apply unchanged to the actual selected required ambient owner.
 * @param {string[]} forms @param {number[]} owners @param {string[]} bindings @returns {boolean} */
export function acceptIntrinsicModuleInitializerScope(forms,owners,bindings){
 const translated=[];
 for(const form of forms)translated.push(form==='intrinsic value binding'?'owned inert value binding':form);
 return acceptInertModuleBindingScope(translated,owners,bindings);
}
