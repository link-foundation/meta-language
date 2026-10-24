// Formal String representation and mechanical bridges to complete shared
// UTF-16 operations. Internal strings remain unit columns until display.
/** @param {string} kind @param {string} operation @param {string} receiver @returns {boolean} */
export function requireFormalStringUnitRepresentation(kind,operation,receiver){
 if(kind==='type')return operation==='string';
 if(kind==='math')return operation==='createJavaScriptStringFromUnits'||operation==='convertJavaScriptCharacterCodesToString';
 if(kind==='method')return receiver==='string';
 return kind==='stringPoints'||kind==='stringMap'||kind==='stringTest';
}
/** @param {string} target @returns {string} */
export function readFormalStringUnitType(target){return target==='Lean'?'(Array Nat)':target==='Rocq'?'(list N)':'';}
/** The same exact relative bound operation serves arrays of every element.
 * @param {string} target @param {string} value @param {string} start @param {string} end @returns {string} */
export function renderFormalArraySlice(target,value,start,end){
 if(target==='Lean')return '(let ml_values := '+value+'; let ml_begin := '+start+'; let ml_end := '+end+'; let ml_length := Float.ofNat ml_values.size; let ml_first := (MLUtf16Text.slicePosition ml_begin ml_length).toUInt64.toNat; let ml_last := (MLUtf16Text.slicePosition ml_end ml_length).toUInt64.toNat; ml_values.extract ml_first ml_last)';
 if(target==='Rocq')return '(let ml_values := '+value+' in let ml_begin := '+start+' in let ml_end := '+end+' in let ml_length := PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length ml_values))) in let ml_first := Z.to_nat (ml_float_index (MLUtf16Text.slicePosition ml_begin ml_length)) in let ml_last := Z.to_nat (ml_float_index (MLUtf16Text.slicePosition ml_end ml_length)) in List.firstn (Nat.sub ml_last ml_first) (List.skipn ml_first ml_values))';
 return '';
}
/** @param {string} target @param {string[]} units @returns {string} */
export function renderFormalStringUnitLiteral(target,units){
 if(target==='Lean')return '(#['+units.join(', ')+'] : Array Nat)';
 return target==='Rocq'?'('+units.map(unit=>unit+'%N').join(' :: ')+(units.length?' :: ':'')+'(@nil N))':'';
}
/** @param {string} target @param {string} value @returns {string} */
export function renderFormalStringUnitNumbers(target,value){
 if(target==='Lean')return '(('+value+').map Float.ofNat)';
 return target==='Rocq'?'(List.map (fun ml_unit => PrimFloat.of_uint63 (Uint63.of_Z (Z.of_N ml_unit))) '+value+')':'';
}
/** @param {string} target @param {string} value @returns {string} */
export function renderFormalStringUnitsFromNumbers(target,value){
 if(target==='Lean')return '(('+value+').map (fun ml_unit => ml_unit.toUInt64.toNat))';
 return target==='Rocq'?'(List.map (fun ml_unit => Z.to_N (ml_float_index ml_unit)) '+value+')':'';
}
/** @param {string} target @param {string} value @returns {string} */
export function renderFormalStringUnitsFromScalar(target,value){
 if(target==='Lean')return '(String.foldl (fun (ml_units : Array Nat) ml_character => if ml_character.toNat < 65536 then ml_units.push ml_character.toNat else (ml_units.push (55296 + (ml_character.toNat - 65536) / 1024)).push (56320 + (ml_character.toNat - 65536) % 1024)) #[] '+value+')';
 if(target!=='Rocq')return '';
 const bytes='((fix ml_text_bytes (ml_text : string) : list float := match ml_text with EmptyString => nil | String ml_character ml_rest => cons (PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (Ascii.nat_of_ascii ml_character)))) (ml_text_bytes ml_rest) end) '+value+')';
 return renderFormalStringUnitsFromNumbers(target,'(MLUtf16Text.fromUtf8 '+bytes+')');
}
/** @param {string} target @param {string} value @returns {string} */
export function renderFormalStringScalarDisplay(target,value){
 const numbers=renderFormalStringUnitNumbers(target,value);
 if(target==='Lean')return '(String.ofList ((MLUtf16Text.displayPoints '+numbers+').toList.map (fun ml_point => Char.ofNat ml_point.toUInt64.toNat)))';
 if(target==='Rocq')return '((fix ml_display_bytes (ml_bytes : list float) : string := match ml_bytes with nil => EmptyString | cons ml_byte ml_rest => String (Ascii.ascii_of_N (Z.to_N (ml_float_index ml_byte))) (ml_display_bytes ml_rest) end) (MLUtf16Text.displayUtf8 '+numbers+'))';
 return '';
}
/** @param {string} target @param {string} operation @param {string} left @param {string} right @returns {string} */
export function renderFormalStringUnitComparison(target,operation,left,right){
 const value='(MLUtf16Text.compare '+renderFormalStringUnitNumbers(target,left)+' '+renderFormalStringUnitNumbers(target,right)+')';
 if(target==='Lean'){
  if(operation==='eq')return '('+value+' == (0 : Float))';
  if(operation==='ne')return '('+value+' != (0 : Float))';
  if(operation==='lt')return '(decide ('+value+' < (0 : Float)))';
  if(operation==='le')return '(decide ('+value+' <= (0 : Float)))';
  if(operation==='gt')return '(decide ('+value+' > (0 : Float)))';
  return operation==='ge'?'(decide ('+value+' >= (0 : Float)))':'';
 }
 if(target!=='Rocq')return '';
 if(operation==='eq')return '(PrimFloat.eqb '+value+' 0%float)';
 if(operation==='ne')return '(negb (PrimFloat.eqb '+value+' 0%float))';
 if(operation==='lt')return '(PrimFloat.ltb '+value+' 0%float)';
 if(operation==='le')return '(PrimFloat.leb '+value+' 0%float)';
 if(operation==='gt')return '(PrimFloat.ltb 0%float '+value+')';
 return operation==='ge'?'(PrimFloat.leb 0%float '+value+')':'';
}
/** Text receiver first; other inputs are already checked target expressions.
 * @param {string} target @param {string} operation @param {string[]} values @returns {string} */
export function renderFormalStringUnitOperation(target,operation,values){
 if(operation==='join'){
  const value=values[0]??'';
  const numbers=renderFormalStringUnitNumbers(target,'ml_text');
  const texts=target==='Lean'?'(('+value+').map (fun ml_text => '+numbers+'))':'(List.map (fun ml_text => '+numbers+') '+value+')';
  return renderFormalStringUnitsFromNumbers(target,'(MLUtf16Text.join '+texts+' '+renderFormalStringUnitNumbers(target,values[1]??'')+')');
 }
 const text=renderFormalStringUnitNumbers(target,values[0]??'');
 const position=values[1]??'';
 if(operation==='slice'||operation==='substring'||operation==='charAt'){
  const name=operation==='charAt'?'character':operation;
  return renderFormalStringUnitsFromNumbers(target,'(MLUtf16Text.'+name+' '+text+' '+position+(operation==='charAt'?'':' '+(values[2]??''))+')');
 }
 if(operation==='at'){
  const result='(MLUtf16Text.relativeCharacter '+text+' '+position+')';
  return target==='Lean'?'(('+result+')[0]?.map (fun ml_units => '+renderFormalStringUnitsFromNumbers(target,'ml_units')+'))'
   :'(match '+result+' with nil => None | cons ml_units _ => Some '+renderFormalStringUnitsFromNumbers(target,'ml_units')+' end)';
 }
 if(operation==='codePointAt'){
  const result='(MLUtf16Text.codePoint '+text+' '+position+')';
  return target==='Lean'?'('+result+')[0]?':'(match '+result+' with nil => None | cons ml_point _ => Some ml_point end)';
 }
 if(operation==='indexOf'||operation==='lastIndexOf')return '(MLUtf16Text.'+(operation==='indexOf'?'index':'lastIndex')+' '+text+' '+renderFormalStringUnitNumbers(target,values[1]??'')+' '+(values[2]??'')+')';
 if(operation==='points'){
  const result='(MLUtf16Text.points '+text+')';
  return target==='Lean'?'(('+result+').map (fun ml_units => '+renderFormalStringUnitsFromNumbers(target,'ml_units')+'))':'(List.map (fun ml_units => '+renderFormalStringUnitsFromNumbers(target,'ml_units')+') '+result+')';
 }
 if(operation==='trim'||operation==='trimStart'||operation==='trimEnd')return renderFormalStringUnitsFromNumbers(target,'(MLUtf16Text.trim '+text+' '+(operation==='trimEnd'?'false':'true')+' '+(operation==='trimStart'?'false':'true')+')');
 const name=operation==='startsWith'?'starts':operation==='endsWith'?'ends':operation==='includes'?'includes':'';
 return name?'(MLUtf16Text.'+name+' '+text+' '+renderFormalStringUnitNumbers(target,values[1]??'')+')':'';
}
