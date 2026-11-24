// Additive scope interpretation: actual Number literals cannot call user code.
import {readRequiredNumberFact,readRequiredTextFact,readRequiredBooleanFact} from './fact-column-rules.js';
/** @param {string[]} terms @param {number[]} parents @param {number} owner @returns {number[]} */
function readNumericModuleBindingChildren(terms,parents,owner){
 const result=[];
 for(let at=0;at<terms.length;at++)if(readRequiredNumberFact(parents,at)===owner&&readRequiredTextFact(terms,at)!=='comment'&&readRequiredTextFact(terms,at)!=='whitespace')result.push(at);
 return result;
}
/** Modules exclude legacy leading-zero decimals and octals; BigInt has its
 * own arithmetic and abrupt-completion obligations.
 * https://tc39.es/ecma262/2024/multipage/strict-mode-of-ecmascript.html
 * @param {string} text @returns {boolean} */
function acceptNumericModuleBindingNumberLiteral(text){
 if(text.length===0||text.includes('n'))return false;
 for(const prefix of ['00','01','02','03','04','05','06','07','08','09'])if(text.startsWith(prefix))return false;
 return true;
}
/** Classify complete actual CST facts without evaluating an initializer.
 * Only Number literals, parentheses and Number arithmetic are admitted.
 * Return [actual name row, ambient owner 0], or refusal [].
 * @param {string[]} terms @param {string[]} fields @param {number[]} parents
 * @param {number[]} starts @param {number[]} ends @param {string[]} texts
 * @param {number[]} errors @param {number} sourceBytes @returns {number[]} */
export function readNumericModuleBinding(terms,fields,parents,starts,ends,texts,errors,sourceBytes){
 if(terms.length<1||terms.length>4096||fields.length!==terms.length||parents.length!==terms.length||starts.length!==terms.length||ends.length!==terms.length||texts.length!==terms.length||errors.length!==terms.length||!(sourceBytes>=0&&sourceBytes<=65536&&sourceBytes%1===0))return [];
 for(let at=0;at<terms.length;at++){
  const parent=readRequiredNumberFact(parents,at),start=readRequiredNumberFact(starts,at),end=readRequiredNumberFact(ends,at);
  if(!(start>=0&&start%1===0&&end>=start&&end%1===0&&end<=sourceBytes)||readRequiredNumberFact(errors,at)!==0)return [];
  if(at===0){if(parent!==-1||readRequiredTextFact(terms,at)!=='program'||end!==sourceBytes)return [];}
  else if(!(parent>=0&&parent%1===0&&parent<at)||start<readRequiredNumberFact(starts,parent)||end>readRequiredNumberFact(ends,parent))return [];
 }
 const top=readNumericModuleBindingChildren(terms,parents,0);if(top.length!==1)return [];
 let declaration=readRequiredNumberFact(top,0);
 if(readRequiredTextFact(terms,declaration)==='export_statement'){
  let selected=-1,exports=0;
  for(const child of readNumericModuleBindingChildren(terms,parents,declaration)){
   const term=readRequiredTextFact(terms,child);
   if(term==='export')exports++;else if(term==='lexical_declaration'&&selected<0)selected=child;else if(term!==';')return [];
  }
  if(exports!==1||selected<0)return [];declaration=selected;
 }
 if(readRequiredTextFact(terms,declaration)!=='lexical_declaration')return [];
 let binding=-1,constants=0;
 for(const child of readNumericModuleBindingChildren(terms,parents,declaration)){
  const term=readRequiredTextFact(terms,child);
  if(term==='const')constants++;else if(term==='variable_declarator'&&binding<0)binding=child;else if(term!==';')return [];
 }
 if(constants!==1||binding<0)return [];
 let name=-1,value=-1,equals=0;
 for(const child of readNumericModuleBindingChildren(terms,parents,binding)){
  const field=readRequiredTextFact(fields,child),term=readRequiredTextFact(terms,child);
  if(field==='name'&&term==='identifier'&&name<0)name=child;else if(field==='value'&&value<0)value=child;else if(term==='=')equals++;else return [];
 }
 if(name<0||value<0||equals!==1||readRequiredTextFact(texts,name).length===0||readRequiredTextFact(texts,name).includes('\\'))return [];
 const values=[];
 for(let at=terms.length-1;at>=0;at--){
  const term=readRequiredTextFact(terms,at),children=readNumericModuleBindingChildren(terms,parents,at);let admitted=false;
  if(term==='number')admitted=children.length===0&&acceptNumericModuleBindingNumberLiteral(readRequiredTextFact(texts,at));
  else if(term==='parenthesized_expression'&&children.length===3){
   admitted=readRequiredTextFact(terms,readRequiredNumberFact(children,0))==='('&&readRequiredTextFact(terms,readRequiredNumberFact(children,2))===')'&&readRequiredBooleanFact(values,terms.length-1-readRequiredNumberFact(children,1));
  }else if(term==='unary_expression'&&children.length===2){
   const operator=readRequiredTextFact(terms,readRequiredNumberFact(children,0));
   admitted=(operator==='+'||operator==='-'||operator==='~')&&readRequiredBooleanFact(values,terms.length-1-readRequiredNumberFact(children,1));
  }else if(term==='binary_expression'&&children.length===3){
   const operator=readRequiredTextFact(terms,readRequiredNumberFact(children,1));
   const numeric=operator==='+'||operator==='-'||operator==='*'||operator==='/'||operator==='%'||operator==='**'||operator==='<<'||operator==='>>'||operator==='>>>'||operator==='&'||operator==='|'||operator==='^';
   admitted=numeric&&readRequiredBooleanFact(values,terms.length-1-readRequiredNumberFact(children,0))&&readRequiredBooleanFact(values,terms.length-1-readRequiredNumberFact(children,2));
  }
  values.push(admitted);
 }
 return readRequiredBooleanFact(values,terms.length-1-value)?[name,0]:[];
}

/** A translated pure Number initializer may supply its actual scalar static
 * signature. The complete CST owns the name; ordinary literal signatures keep
 * their original route. No container or callable acquires this interpretation.
 * @param {string} kind @param {string} type @param {boolean} literal
 * @param {string} sourceName @param {string} signatureName @returns {boolean} */
export function acceptNumericModuleBindingSignature(kind,type,literal,sourceName,signatureName){
 return kind==='const'&&type==='float'&&!literal&&sourceName.length>0&&sourceName===signatureName;
}
