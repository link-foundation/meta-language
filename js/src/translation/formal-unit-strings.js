import {acceptRustStringByteKernelUnitRequirement} from './rust-string-byte-view-rules.js';
// Actual checked IR adapters for the shared formal UTF-16 representation.
import {requireFormalStringUnitRepresentation,renderFormalArraySlice,renderFormalStringUnitLiteral,renderFormalStringUnitsFromScalar,renderFormalStringScalarDisplay,renderFormalStringUnitNumbers,renderFormalStringUnitsFromNumbers,renderFormalStringUnitComparison,renderFormalStringUnitOperation} from './formal-string-unit-rules.js';
import {acceptLinkedJavaScriptStringUnitConstruction} from './javascript-string-unit-rules.js';
import {acceptLinkedJavaScriptStringConstruction,readJavaScriptStringConstructionForm,renderJavaScriptStringConstruction} from './string-construction-rules.js';
import {acceptLinkedJavaScriptNumberConversion} from './string-number-rules.js';
import {acceptLinkedJavaScriptBigIntegerResult,readJavaScriptBigIntegerResultOperation} from './big-integer-conversion-rules.js';
import {unsupported} from './diagnostics.js';
import {renderOptionalProjection,renderOptionalStrictEquality} from './optional-value-rules.js';
import {renderFormalPrimitiveSameValueZero,renderFormalArrayInclusion} from './formal-string-unit-rules.js';
import {acceptUnicodeCaseMappingOperation,readUnicodeCaseEncodingStatement} from './unicode-case-emission-rules.js';

export function requireFormalUnitStrings(program){
 if(program.utf16Strings)return true;
 if(program.sourceLanguage!=='JavaScript'&&program.sourceLanguage!=='Rust')return false;
 const pending=[...program.declarations.values(),program.main],seen=new WeakSet();
 while(pending.length){
  const value=pending.pop();if(!value||typeof value!=='object'||seen.has(value))continue;seen.add(value);
  if(program.sourceLanguage==='JavaScript'&&(requireFormalStringUnitRepresentation('type',value.kind??'','')||requireFormalStringUnitRepresentation(value.k??'',value.op??'',value.object?.type?.kind??'')))return true;
  if(acceptRustStringByteKernelUnitRequirement(program.sourceLanguage,value.k??'',value.op??'',value.object?.type?.kind??''))return true;
  pending.push(...Object.entries(value).filter(([key])=>key!=='span').map(([,child])=>child));
 }
 return false;
}
export function prepareFormalUnitStrings(emitter){
 if(!emitter.utf16Strings)return;
 emitter.helpers.add('utf16Text');
 if(emitter.target==='Rocq'){emitter.floats();emitter.helpers.add('floatIndex');}
 emitter.state.encode('utf16-formal-strings','source strings are ordered UTF-16 code-unit columns represented by natural numbers 0..65535; generated complete shared operations retain isolated units, and scalar display alone replaces unpaired surrogates');
}
export function formalUnitLiteral(emitter,value){return renderFormalStringUnitLiteral(emitter.target,Array.from({length:value.length},(_,at)=>String(value.charCodeAt(at))));}
export function formalScalarUnits(emitter,value){return renderFormalStringUnitsFromScalar(emitter.target,value);}
export function formalUnitDisplay(emitter,value){return renderFormalStringScalarDisplay(emitter.target,value);}
export function formalUnitLength(emitter,value){return emitter.target==='Lean'?'(Float.ofNat ('+value+').size)':'(PrimFloat.of_uint63 (Uint63.of_Z (Z.of_nat (List.length '+value+'))))';}
export function formalUnitTruth(emitter,value){return emitter.target==='Lean'?'(!('+value+').isEmpty)':'(match '+value+' with nil => false | _ => true end)';}

export function readFormalUnitStringExpression(emitter,e,depth=0){
 if(e.k==='method'&&e.object.type.kind==='array'&&e.op==='includes'){
  emitter.helpers.add('utf16Text');
  if(emitter.target==='Rocq'){emitter.floats();emitter.helpers.add('floatIndex');}
  const equal=(type,left,right)=>{
   if(type.kind==='option')return renderOptionalStrictEquality(emitter.target,'option',type.absence,'option',type.absence,equal(type.element,'ml_left','ml_right'),left,right,false);
   const value=renderFormalPrimitiveSameValueZero(emitter.target,type.kind,Boolean(type.signed),emitter.utf16Strings,left,right);
   if(!value)throw unsupported('formal array includes element','the element needs its source SameValueZero interpretation',e.span);
   return value;
  };
  return renderFormalArrayInclusion(emitter.target,emitter.expr(e.object,depth),emitter.expr(e.args[0],depth),emitter.expr(e.args[1],depth),equal(e.object.type.element,'ml_candidate','ml_search'));
 }
 if(e.k==='method'&&e.object.type.kind==='array'&&e.op==='slice'){
  emitter.helpers.add('utf16Text');
  if(emitter.target==='Rocq'){emitter.floats();emitter.helpers.add('floatIndex');}
  const values=[emitter.expr(e.object,depth),...e.args.map(value=>emitter.expr(value,depth))];
  return renderFormalArraySlice(emitter.target,values[0],values[1],values[2]);
 }
 if(!emitter.utf16Strings)return null;
 const target=emitter.target,expression=value=>emitter.expr(value,depth);
 if(e.k==='lit'&&e.type.kind==='string')return formalUnitLiteral(emitter,String(e.value));
 if(e.k==='outNil')return target==='Lean'?'([] : List (Array Nat))':'(@nil (list N))';
 if(e.k==='math'){
  if(acceptLinkedJavaScriptStringUnitConstruction(e.k,e.op))return renderFormalStringUnitsFromNumbers(target,expression(e.args[0]));
  if(acceptLinkedJavaScriptStringConstruction(e.k,e.op)){
   const value=renderJavaScriptStringConstruction(target,e.op,expression(e.args[0]));
   return readJavaScriptStringConstructionForm(e.op)==='characters'?renderFormalStringUnitsFromNumbers(target,value):value;
  }
  if(acceptLinkedJavaScriptNumberConversion(e.k,e.op)&&e.args[0].type.kind==='string'){
   emitter.helpers.add('stringNumber');
   return '(MLStringNumber.convertUnits '+renderFormalStringUnitNumbers(target,expression(e.args[0]))+')';
  }
  if(acceptLinkedJavaScriptBigIntegerResult(e.k,e.op)&&readJavaScriptBigIntegerResultOperation(e.op)==='string-compare'){
   emitter.helpers.add('bigIntegerConversion');
   return '(MLBigIntegerConversion.compareUnits '+renderFormalStringUnitNumbers(target,expression(e.args[0]))+' '+expression(e.args[1])+')';
  }
  if(acceptLinkedJavaScriptBigIntegerResult(e.k,e.op)&&readJavaScriptBigIntegerResultOperation(e.op)==='string'){
   emitter.helpers.add('bigIntegerConversion');
   return '(MLBigIntegerConversion.convertUnits '+renderFormalStringUnitNumbers(target,expression(e.args[0]))+')';
  }
 }
 if(e.k==='length'&&e.array.type.kind==='string')return formalUnitLength(emitter,expression(e.array));
 if(e.k==='optionalProject'&&e.object.type.element.kind==='string')return renderOptionalProjection(target,e.object.type.absence,expression(e.object),'','ml_value',formalUnitLength(emitter,'ml_value'),'');
 if(e.k==='stringPoints')return renderFormalStringUnitOperation(target,'points',[expression(e.object)]);
 if(e.k==='binary'&&e.op==='concat')return target==='Lean'?'('+expression(e.left)+' ++ '+expression(e.right)+')':'(List.app '+expression(e.left)+' '+expression(e.right)+')';
 if(e.k==='binary'&&e.domain?.kind==='string')return renderFormalStringUnitComparison(target,e.op,expression(e.left),expression(e.right));
 if(e.k==='stringTest'){
  const operations={unicodeCodeUnitLessThan:'lt',unicodeCodeUnitLessThanOrEqual:'le',unicodeCodeUnitGreaterThan:'gt',unicodeCodeUnitGreaterThanOrEqual:'ge'};
  return operations[e.op]?renderFormalStringUnitComparison(target,operations[e.op],expression(e.string),expression(e.search))
   :renderFormalStringUnitOperation(target,e.op,[expression(e.string),expression(e.search)])||null;
 }
 if(e.k==='stringMap'){
  if(acceptUnicodeCaseMappingOperation(e.op)){emitter.helpers.add('unicodeCase');emitter.state.encode('unicode-default-case-16',readUnicodeCaseEncodingStatement());}
  return renderFormalStringUnitOperation(target,e.op,[expression(e.string)])||null;
 }
 if(e.k==='method'&&e.object.type.kind==='string'){
  const values=[expression(e.object),...e.args.map(value=>value===null?'':expression(value))];
  const result=renderFormalStringUnitOperation(target,e.op,values);
  if(!result)throw unsupported('formal UTF16 string '+e.op,'the operation needs its complete shared code-unit definition',e.span);
  return result;
 }
 if(e.k==='optionalText'){
  const element=e.arg.type.element;
  let present=element.kind==='string'?'ml_present':element.kind==='unit'?formalUnitLiteral(emitter,'undefined'):emitter.toText({k:'var',name:'ml_present',type:element},...(target==='Lean'?[depth,e.console]:[e.console]));
  if(e.console&&['int','nat','fixed'].includes(element.kind))present=target==='Lean'?'('+present+' ++ '+formalUnitLiteral(emitter,'n')+')':'(List.app '+present+' '+formalUnitLiteral(emitter,'n')+')';
  return emitter.optionalMatch(e.arg,'ml_present',present,formalUnitLiteral(emitter,'null'),formalUnitLiteral(emitter,'undefined'),depth);
 }
 if(e.k==='optionalTypeOf')return emitter.optionalMatch(e.arg,'ml_present',formalUnitLiteral(emitter,e.present??'undefined'),formalUnitLiteral(emitter,'object'),formalUnitLiteral(emitter,'undefined'),depth);
 if(e.k==='method'&&e.object.type.kind==='array'&&e.op==='join'){
  const text=(name,type)=>{
   if(type.kind==='unit')return formalUnitLiteral(emitter,'');
   if(type.kind==='option'){
    const present=type.absence==='both'?'some (some ml_present)':'some ml_present';
    return target==='Lean'?'(match '+name+' with | '+present+' => '+text('ml_present',type.element)+' | _ => '+formalUnitLiteral(emitter,'')+')'
     :'(match '+name+' with | '+(type.absence==='both'?'Some (Some ml_present)':'Some ml_present')+' => '+text('ml_present',type.element)+' | _ => '+formalUnitLiteral(emitter,'')+' end)';
   }
   return type.kind==='string'?name:emitter.toText({k:'var',name,type},...(target==='Lean'?[depth,false]:[false]));
  };
  const value=expression(e.object),converted=text('ml_value',e.object.type.element);
  const values=target==='Lean'?'(('+value+').map (fun ml_value => '+converted+'))':'(List.map (fun ml_value => '+converted+') '+value+')';
  return renderFormalStringUnitOperation(target,'join',[values,expression(e.args[0])]);
 }
 return null;
}
