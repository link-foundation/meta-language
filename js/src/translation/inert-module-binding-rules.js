// Scope facts only: initialization cannot introduce an uncounted lexical owner.
import {readRequiredNumberFact,readRequiredTextFact,readRequiredBooleanFact} from './fact-column-rules.js';
/** @param {string[]} terms @param {number[]} parents @param {number} owner @returns {number[]} */
function readInertBindingChildren(terms,parents,owner){
 const result=[];
 for(let at=0;at<terms.length;at++)if(readRequiredNumberFact(parents,at)===owner&&readRequiredTextFact(terms,at)!=='comment'&&readRequiredTextFact(terms,at)!=='whitespace')result.push(at);
 return result;
}
/** Child values are appended in reverse source preorder; every child already
 * has one owned result when its parent is classified.
 * @param {boolean[]} values @param {number} count @param {number} child @returns {boolean} */
function readInertBindingValue(values,count,child){return readRequiredBooleanFact(values,count-1-child);}
/** @param {string[]} terms @param {number[]} parents @param {boolean[]} values @param {number} owner @param {string} open @param {string} close @param {string} element @returns {boolean} */
function acceptInertBindingSequence(terms,parents,values,owner,open,close,element){
 const children=readInertBindingChildren(terms,parents,owner);
 if(children.length<2||readRequiredTextFact(terms,readRequiredNumberFact(children,0))!==open||readRequiredTextFact(terms,readRequiredNumberFact(children,children.length-1))!==close)return false;
 let value=true;
 for(let at=1;at<children.length-1;at++){
  const child=readRequiredNumberFact(children,at),term=readRequiredTextFact(terms,child);
  if(value){if(!readInertBindingValue(values,parents.length,child)||element!==''&&term!==element)return false;}
  else if(term!==',')return false;
  value=!value;
 }
 return true;
}
/** @param {string[]} terms @param {number[]} parents @param {boolean[]} values @param {number} owner @returns {number} */
function readInertBindingArgument(terms,parents,values,owner){
 const children=readInertBindingChildren(terms,parents,owner);
 if(children.length!==3&&children.length!==4)return -1;
 const first=readRequiredNumberFact(children,0),value=readRequiredNumberFact(children,1),last=readRequiredNumberFact(children,children.length-1);
 if(readRequiredTextFact(terms,first)!=='('||readRequiredTextFact(terms,last)!==')'||!readInertBindingValue(values,parents.length,value))return -1;
 if(children.length===4&&readRequiredTextFact(terms,readRequiredNumberFact(children,2))!==',')return -1;
 return value;
}
/** Complete parser-owned CST facts, bounded before interpretation. Returns
 * [actual name row, required ambient owner:0=none,1=Map,2=Set,3=Object], or [].
 * No initializer or function is executed and no statement is replaced.
 * @param {string[]} terms @param {string[]} fields @param {number[]} parents
 * @param {number[]} starts @param {number[]} ends @param {string[]} texts
 * @param {number[]} errors @param {number} sourceBytes @returns {number[]} */
export function readInertModuleBinding(terms,fields,parents,starts,ends,texts,errors,sourceBytes){
 if(terms.length<1||terms.length>4096||fields.length!==terms.length||parents.length!==terms.length||starts.length!==terms.length||ends.length!==terms.length||texts.length!==terms.length||errors.length!==terms.length||!(sourceBytes>=0&&sourceBytes<=65536&&sourceBytes%1===0))return [];
 for(let at=0;at<terms.length;at++){
  const parent=readRequiredNumberFact(parents,at),start=readRequiredNumberFact(starts,at),end=readRequiredNumberFact(ends,at);
  if(!(start>=0&&start%1===0&&end>=start&&end%1===0&&end<=sourceBytes)||readRequiredNumberFact(errors,at)!==0)return [];
  if(at===0){if(parent!==-1||readRequiredTextFact(terms,at)!=='program'||end!==sourceBytes)return [];}
  else if(!(parent>=0&&parent%1===0&&parent<at)||start<readRequiredNumberFact(starts,parent)||end>readRequiredNumberFact(ends,parent))return [];
 }
 let top=readInertBindingChildren(terms,parents,0);
 if(top.length!==1)return [];
 let declaration=readRequiredNumberFact(top,0);
 if(readRequiredTextFact(terms,declaration)==='export_statement'){
  const children=readInertBindingChildren(terms,parents,declaration);let selected=-1,exports=0;
  for(const child of children){const term=readRequiredTextFact(terms,child);if(term==='export')exports++;else if(term==='lexical_declaration'&&selected<0)selected=child;else if(term!==';')return [];}
  if(exports!==1||selected<0)return [];declaration=selected;
 }
 if(readRequiredTextFact(terms,declaration)!=='lexical_declaration')return [];
 let binding=-1,constants=0;
 for(const child of readInertBindingChildren(terms,parents,declaration)){
  const term=readRequiredTextFact(terms,child);if(term==='const')constants++;else if(term==='variable_declarator'&&binding<0)binding=child;else if(term!==';')return [];
 }
 if(constants!==1||binding<0)return [];
 let name=-1,value=-1,equals=0;
 for(const child of readInertBindingChildren(terms,parents,binding)){
  const field=readRequiredTextFact(fields,child),term=readRequiredTextFact(terms,child);
  if(field==='name'&&term==='identifier'&&name<0)name=child;else if(field==='value'&&value<0)value=child;else if(term==='=')equals++;else return [];
 }
 if(name<0||value<0||equals!==1||readRequiredTextFact(texts,name).length===0||readRequiredTextFact(texts,name).includes('\\'))return [];
 const values=[];
 for(let at=terms.length-1;at>=0;at--){
  const term=readRequiredTextFact(terms,at),children=readInertBindingChildren(terms,parents,at);let admitted=false;
  if(term==='string'||term==='number'||term==='true'||term==='false'||term==='null')admitted=true;
  else if(term==='array')admitted=acceptInertBindingSequence(terms,parents,values,at,'[',']','');
  else if(term==='object')admitted=acceptInertBindingSequence(terms,parents,values,at,'{','}','pair');
  else if(term==='pair'){
   let key=-1,item=-1,colons=0;
   for(const child of children){const field=readRequiredTextFact(fields,child),part=readRequiredTextFact(terms,child);
    if(field==='key'&&key<0&&(part==='property_identifier'||part==='string'||part==='number'))key=child;
    else if(field==='value'&&item<0)item=child;else if(part===':')colons++;else{key=-2;break;}}
   admitted=key>=0&&item>=0&&colons===1&&readInertBindingValue(values,parents.length,item);
  }else if(term==='unary_expression'&&children.length===2){
   const operator=readRequiredNumberFact(children,0),operand=readRequiredNumberFact(children,1),operation=readRequiredTextFact(terms,operator);
   admitted=(operation==='-'||operation==='+')&&readRequiredTextFact(terms,operand)==='number';
  }else if(term==='parenthesized_expression'&&children.length===3){
   admitted=readRequiredTextFact(terms,readRequiredNumberFact(children,0))==='('&&readRequiredTextFact(terms,readRequiredNumberFact(children,2))===')'&&readInertBindingValue(values,parents.length,readRequiredNumberFact(children,1));
  }
  values.push(admitted);
 }
 if(readInertBindingValue(values,parents.length,value))return [name,0];
 const term=readRequiredTextFact(terms,value),children=readInertBindingChildren(terms,parents,value);
 if(term==='new_expression'){
  let constructor=-1,argumentsAt=-1,news=0;
  for(const child of children){const field=readRequiredTextFact(fields,child),part=readRequiredTextFact(terms,child);
   if(field==='constructor'&&part==='identifier'&&constructor<0)constructor=child;else if(field==='arguments'&&part==='arguments'&&argumentsAt<0)argumentsAt=child;else if(part==='new')news++;else return [];}
  if(news!==1||constructor<0||argumentsAt<0)return [];
  const owner=readRequiredTextFact(texts,constructor),argument=readInertBindingArgument(terms,parents,values,argumentsAt);
  if(argument<0||readRequiredTextFact(terms,argument)!=='array')return [];
  if(owner==='Set')return [name,2];
  if(owner==='Map'){
   for(const child of readInertBindingChildren(terms,parents,argument)){
    const part=readRequiredTextFact(terms,child);if(part==='['||part===']'||part===',')continue;
    if(part!=='array')return [];
    const entries=readInertBindingChildren(terms,parents,child);if(entries.length!==5&&entries.length!==6)return [];
   }
   return [name,1];
  }
 }else if(term==='call_expression'){
  let callee=-1,argumentsAt=-1;
  for(const child of children){const field=readRequiredTextFact(fields,child),part=readRequiredTextFact(terms,child);
   if(field==='function'&&part==='member_expression'&&callee<0)callee=child;else if(field==='arguments'&&part==='arguments'&&argumentsAt<0)argumentsAt=child;else return [];}
  if(callee<0||argumentsAt<0)return [];
  const member=readInertBindingChildren(terms,parents,callee);if(member.length!==3)return [];
  const object=readRequiredNumberFact(member,0),dot=readRequiredNumberFact(member,1),property=readRequiredNumberFact(member,2);
  if(readRequiredTextFact(terms,object)!=='identifier'||readRequiredTextFact(texts,object)!=='Object'||readRequiredTextFact(terms,dot)!=='.'||readRequiredTextFact(terms,property)!=='property_identifier'||readRequiredTextFact(texts,property)!=='freeze')return [];
  const argument=readInertBindingArgument(terms,parents,values,argumentsAt);
  if(argument>=0&&(readRequiredTextFact(terms,argument)==='array'||readRequiredTextFact(terms,argument)==='object'))return [name,3];
 }
 return [];
}
/** Independent extended source-scope admission. Actual module binders exclude
 * required standard owner spellings before any special frontend selects them.
 * @param {string[]} forms @param {number[]} owners @param {string[]} bindings @returns {boolean} */
export function acceptInertModuleBindingScope(forms,owners,bindings){
 if(forms.length!==owners.length)return false;
 for(let at=0;at<forms.length;at++){
  const form=readRequiredTextFact(forms,at),owner=readRequiredNumberFact(owners,at);
  if(form==='owned inert value binding'){
   if(!(owner>=0&&owner<=3&&owner%1===0))return false;
   for(const binding of bindings)if(owner===1&&binding==='Map'||owner===2&&binding==='Set'||owner===3&&binding==='Object')return false;
  }else if(owner!==-1||(form!=='comment'&&form!=='named function declaration'&&form!=='named import'&&form!=='primitive literal binding'&&form!=='empty class declaration'&&form!=='deferred named class declaration'&&form!=='deferred function value binding'))return false;
 }
 return true;
}
