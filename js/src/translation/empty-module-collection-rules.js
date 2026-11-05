// Additive scope interpretation; the original literal-binding rules stay exact.
import {readRequiredNumberFact,readRequiredTextFact} from './fact-column-rules.js';
/** @param {string[]} terms @param {number[]} parents @param {number} owner @returns {number[]} */
function readEmptyCollectionBindingChildren(terms,parents,owner){
 const result=[];
 for(let at=0;at<terms.length;at++)if(readRequiredNumberFact(parents,at)===owner&&readRequiredTextFact(terms,at)!=='comment'&&readRequiredTextFact(terms,at)!=='whitespace')result.push(at);
 return result;
}
/** Complete actual CST; return [name row, required owner:1=Map,2=Set].
 * Empty standard allocation cannot call an iterable or evaluate an argument.
 * Module scope separately proves that the required owner is unshadowed.
 * @param {string[]} terms @param {string[]} fields @param {number[]} parents
 * @param {number[]} starts @param {number[]} ends @param {string[]} texts
 * @param {number[]} errors @param {number} sourceBytes @returns {number[]} */
export function readEmptyStandardCollectionModuleBinding(terms,fields,parents,starts,ends,texts,errors,sourceBytes){
 if(terms.length<1||terms.length>4096||fields.length!==terms.length||parents.length!==terms.length||starts.length!==terms.length||ends.length!==terms.length||texts.length!==terms.length||errors.length!==terms.length||!(sourceBytes>=0&&sourceBytes<=65536&&sourceBytes%1===0))return [];
 for(let at=0;at<terms.length;at++){
  const parent=readRequiredNumberFact(parents,at),start=readRequiredNumberFact(starts,at),end=readRequiredNumberFact(ends,at);
  if(!(start>=0&&start%1===0&&end>=start&&end%1===0&&end<=sourceBytes)||readRequiredNumberFact(errors,at)!==0)return [];
  if(at===0){if(parent!==-1||readRequiredTextFact(terms,at)!=='program'||end!==sourceBytes)return [];}
  else if(!(parent>=0&&parent%1===0&&parent<at)||start<readRequiredNumberFact(starts,parent)||end>readRequiredNumberFact(ends,parent))return [];
 }
 const top=readEmptyCollectionBindingChildren(terms,parents,0);if(top.length!==1)return [];
 let declaration=readRequiredNumberFact(top,0);
 if(readRequiredTextFact(terms,declaration)==='export_statement'){
  let selected=-1,exports=0;
  for(const child of readEmptyCollectionBindingChildren(terms,parents,declaration)){
   const term=readRequiredTextFact(terms,child);
   if(term==='export')exports++;else if(term==='lexical_declaration'&&selected<0)selected=child;else if(term!==';')return [];
  }
  if(exports!==1||selected<0)return [];declaration=selected;
 }
 if(readRequiredTextFact(terms,declaration)!=='lexical_declaration')return [];
 let binding=-1,constants=0;
 for(const child of readEmptyCollectionBindingChildren(terms,parents,declaration)){
  const term=readRequiredTextFact(terms,child);
  if(term==='const')constants++;else if(term==='variable_declarator'&&binding<0)binding=child;else if(term!==';')return [];
 }
 if(constants!==1||binding<0)return [];
 let name=-1,value=-1,equals=0;
 for(const child of readEmptyCollectionBindingChildren(terms,parents,binding)){
  const field=readRequiredTextFact(fields,child),term=readRequiredTextFact(terms,child);
  if(field==='name'&&term==='identifier'&&name<0)name=child;else if(field==='value'&&term==='new_expression'&&value<0)value=child;else if(term==='=')equals++;else return [];
 }
 if(name<0||value<0||equals!==1||readRequiredTextFact(texts,name).length===0||readRequiredTextFact(texts,name).includes('\\'))return [];
 let constructor=-1,argumentsAt=-1,news=0;
 for(const child of readEmptyCollectionBindingChildren(terms,parents,value)){
  const field=readRequiredTextFact(fields,child),term=readRequiredTextFact(terms,child);
  if(field==='constructor'&&term==='identifier'&&constructor<0)constructor=child;
  else if(field==='arguments'&&term==='arguments'&&argumentsAt<0)argumentsAt=child;else if(term==='new')news++;else return [];
 }
 if(news!==1||constructor<0)return [];
 if(argumentsAt>=0){
  const children=readEmptyCollectionBindingChildren(terms,parents,argumentsAt);
  if(children.length!==2||readRequiredTextFact(terms,readRequiredNumberFact(children,0))!=='('||readRequiredTextFact(terms,readRequiredNumberFact(children,1))!==')')return [];
 }
 const owner=readRequiredTextFact(texts,constructor);
 if(owner==='Map')return [name,1];if(owner==='Set')return [name,2];return [];
}
