// Actual token plans for the first ordered record parameter source profile.
// Native algorithms must be generated from the complete recorded closure.
import { acceptJavaScriptSourceBindingName } from './source-identifier-rules.js';
import { acceptJavaScriptArrayBindingName } from './array-binding-rules.js';
import { readRequiredTextFact, readRequiredBooleanFact,readRequiredNumberFact } from './fact-column-rules.js';
import {findJavaScriptPatternDefaultExpressionEnd,readJavaScriptArrayFormalDefaultScopeBoundary} from './array-formal-parameter-rules.js';

/** Detect only an actual object pattern at a formal parameter start. Ordinary
 * names/defaults remain on the old route unless a real pattern also occurs.
 * @param {string[]} kinds @param {string[]} values @param {number} start
 * @returns {number} */
export function findJavaScriptRecordParameterPattern(kinds, values, start) {
  if (kinds.length !== values.length || !Number.isInteger(start) || start < 0 || start >= kinds.length) return -1;
  let depth = 0;
  let binding = true;
  for (let at = start; at < kinds.length; at += 1) {
    const kind = readRequiredTextFact(kinds, at);
    if (kind === 'eof') return -1;
    if (kind !== 'punct') { if (depth === 0) binding = false; continue; }
    const value = readRequiredTextFact(values, at);
    if (depth === 0 && value === ')') return -1;
    if (depth === 0 && binding && value === '{') return at;
    if (value === '(' || value === '[' || value === '{') { depth += 1; binding = false; }
    else if (value === ')' || value === ']' || value === '}') { depth -= 1; if (depth < 0) return -1; }
    else if (depth === 0 && value === ',') binding = true;
    else if (depth === 0 && value !== ',') binding = false;
  }
  return -1;
}

/** Reuse actual lexical identifier admission; equal field names are not equal
 * BoundNames. All duplicate actual local names across the full list refuse.
 * @param {string} kind @param {string} name @param {string[]} names
 * @param {string[]} reserved @returns {boolean} */
export function acceptJavaScriptRecordParameterBinding(kind, name, names, reserved) {
  let duplicate = false;
  for (let index = 0; index < names.length; index += 1) if (readRequiredTextFact(names, index) === name) duplicate = true;
  return acceptJavaScriptArrayBindingName(kind, name, duplicate, reserved);
}

/** Result [endExclusive,count,...instructions]; simple [0,slot,nameToken];
 * record [1,slot,openToken,closeToken,pairCount,...propertyToken,bindingToken].
 * Failure [-reason,actualToken]:1 syntax,2 defaults,3 arrays,4 empty,
 * 5 nested,6 rest,7 computed/string key,8 invalid/duplicate BoundName.
 * @param {string[]} kinds @param {string[]} values @param {number} start
 * @param {string[]} reserved @returns {number[]} */
export function planJavaScriptRecordParameters(kinds, values, start, reserved) {
  return planJavaScriptRecordParameterBindings(kinds, values, start, reserved, false,false);
}

/** Source admission uses ordinary BoundNames and the actual hygienic supply.
 * @param {string[]} kinds @param {string[]} values @param {number} start
 * @param {string[]} reserved @returns {number[]} */
export function planJavaScriptSourceRecordParameters(kinds, values, start, reserved) {
  return planJavaScriptRecordParameterBindings(kinds, values, start, reserved, true,false);
}
/** Separate default profile: each actual record binding owns property/name and
 * initializer start/end columns; -1,-1 means no initializer. Earlier exported
 * required-only plans retain their original two-column contract.
 * @param {string[]} kinds @param {string[]} values @param {number} start
 * @param {string[]} reserved @returns {number[]} */
export function planJavaScriptSourceRecordParametersWithDefaults(kinds,values,start,reserved){return planJavaScriptRecordParameterBindings(kinds,values,start,reserved,true,true);}

/** @param {string} kind @param {string} name @param {string[]} names
 * @param {string[]} reserved @param {boolean} ordinary @returns {boolean} */
function acceptJavaScriptRecordParameterBoundName(kind, name, names, reserved, ordinary) {
  if (!ordinary) return acceptJavaScriptRecordParameterBinding(kind, name, names, reserved);
  let duplicate = false;
  for (let index = 0; index < names.length; index += 1)
    if (readRequiredTextFact(names, index) === name) duplicate = true;
  return acceptJavaScriptSourceBindingName(kind, name, duplicate, reserved);
}

/** Complete actual flat object-pattern syntax shared by formal and lexical
 * binding contexts. Profile 0 preserves historical BoundNames, 1 admits
 * ordinary required names, 2 adds exact original initializer intervals.
 * @param {string[]} kinds @param {string[]} values @param {number} start
 * @param {string[]} reserved @param {string[]} previous @param {number} profile
 * @returns {number[]} */
export function planJavaScriptObjectPatternBindings(kinds,values,start,reserved,previous,profile){
 if(kinds.length!==values.length||!Number.isInteger(start)||start<0||start>=kinds.length||profile<0||profile>2||!Number.isInteger(profile))return [-1,start];
 if(readRequiredTextFact(kinds,start)!=='punct'||readRequiredTextFact(values,start)!=='{')return [-1,start];
 let at=start+1;let names=previous.slice(),pairs=[];
 while(at<kinds.length&&!(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)==='}')){
  if(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)==='...')return [-6,at];
  if(readRequiredTextFact(kinds,at)!=='identifier')return [-7,at];
  const property=at;at+=1;if(at>=kinds.length)return [-1,at];let binding=property;
  if(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)===':'){
   at+=1;if(at>=kinds.length)return [-1,at];
   if(readRequiredTextFact(kinds,at)==='punct'&&(readRequiredTextFact(values,at)==='{'||readRequiredTextFact(values,at)==='['))return [-5,at];
   binding=at;at+=1;
  }
  const name=readRequiredTextFact(values,binding);
  if(!acceptJavaScriptRecordParameterBoundName(readRequiredTextFact(kinds,binding),name,names,reserved,profile>0))return [-8,binding];
  names=names.concat([name]);if(at>=kinds.length)return [-1,at];let first=-1,last=-1;
  if(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)==='='){
   if(profile!==2)return [-2,at];first=at+1;last=findJavaScriptPatternDefaultExpressionEnd(kinds,values,first,'}');if(last<0)return [-1,first];at=last;
  }
  pairs=pairs.concat(profile===2?[property,binding,first,last]:[property,binding]);
  if(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)===','){at+=1;continue;}
  if(!(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)==='}'))return [-1,at];
 }
 if(pairs.length===0)return [-4,start];
 if(at>=kinds.length||!(readRequiredTextFact(kinds,at)==='punct'&&readRequiredTextFact(values,at)==='}'))return [-1,at];
 return [at+1,pairs.length/(profile===2?4:2)].concat(pairs);
}
/** @param {string[]} kinds @param {string[]} values @param {number} start
 * @param {string[]} reserved @param {boolean} ordinary @param {boolean} defaults @returns {number[]} */
function planJavaScriptRecordParameterBindings(kinds, values, start, reserved, ordinary,defaults) {
  if (kinds.length !== values.length || !Number.isInteger(start) || start < 0 || start >= kinds.length) return [-1, start];
  let at = start;
  let slot = 0;
  /** @type {string[]} */
  const names = [];
  /** @type {number[]} */
  let instructions = [];
  while (at < kinds.length && !(readRequiredTextFact(kinds, at) === 'punct' && readRequiredTextFact(values, at) === ')')) {
    if (readRequiredTextFact(kinds, at) === 'punct' && readRequiredTextFact(values, at) === '...') return [-6, at];
    if (readRequiredTextFact(kinds, at) === 'punct' && readRequiredTextFact(values, at) === '[') return [-3, at];
    if (readRequiredTextFact(kinds, at) === 'punct' && readRequiredTextFact(values, at) === '{') {
      const open=at,profile=defaults?2:ordinary?1:0;
      const pattern=planJavaScriptObjectPatternBindings(kinds,values,at,reserved,names,profile);
      if(readRequiredNumberFact(pattern,0)<0)return pattern;
      at=readRequiredNumberFact(pattern,0);const count=readRequiredNumberFact(pattern,1),width=defaults?4:2;
      instructions=instructions.concat([1,slot,open,at-1,count],pattern.slice(2));
      for(let index=0;index<count;index+=1)names.push(readRequiredTextFact(values,readRequiredNumberFact(pattern,2+index*width+1)));
    } else {
      const name = readRequiredTextFact(values, at);
      if (!acceptJavaScriptRecordParameterBoundName(readRequiredTextFact(kinds, at), name, names, reserved, ordinary)) return [-8, at];
      names.push(name);
      instructions = instructions.concat([0, slot, at]);
      at += 1;
    }
    if (at >= kinds.length) return [-1, at];
    if (readRequiredTextFact(kinds, at) === 'punct' && readRequiredTextFact(values, at) === '=') return [-2, at];
    slot += 1;
    if (readRequiredTextFact(kinds, at) === 'punct' && readRequiredTextFact(values, at) === ',') { at += 1; continue; }
    if (!(readRequiredTextFact(kinds, at) === 'punct' && readRequiredTextFact(values, at) === ')')) return [-1, at];
  }
  if (at >= kinds.length || !(readRequiredTextFact(kinds, at) === 'punct' && readRequiredTextFact(values, at) === ')')) return [-1, at];
  return [at + 1, slot].concat(instructions);
}

/** @param {string} context @returns {number} */
export function readJavaScriptPatternInitializerScopeBoundary(context){return context==='ordinary object binding declaration'?0:readJavaScriptArrayFormalDefaultScopeBoundary(context);}
/** @param {string} context @returns {string} */
export function readJavaScriptPatternInitializerSubject(context){return context==='ordinary object binding declaration'?'binding':'parameter';}
/** @param {string} context @returns {string} */
export function readJavaScriptPatternInitializerReferenceReason(context){return context==='ordinary object binding declaration'?'the current binding initializer profile requires initialized earlier lexical bindings':'the current default profile requires a closed parameter initialization environment';}
/** @param {string} context @returns {string} */
export function readJavaScriptPatternInitializerAssignmentReason(context){return context==='ordinary object binding declaration'?'writes during binding initialization need their shared environment interpretation':'writes during parameter initialization need their shared environment interpretation';}
/** @param {string} mode @returns {string} */
export function readJavaScriptObjectBindingStatement(mode){return mode==='let'?'let':mode==='const'?'const':'';}
/** @param {string} kind @param {string} name @param {string[]} reserved @returns {boolean} */
export function acceptJavaScriptObjectBindingName(kind,name,reserved){return acceptJavaScriptSourceBindingName(kind,name,false,reserved);}
/** Local declarations and their TDZ census share the same admitted names.
 * @param {number[]} plan @returns {number[]} */
export function readJavaScriptObjectPatternBindingPositions(plan){
 if(plan.length<2||readRequiredNumberFact(plan,0)<0)throw new RangeError('an object binding census needs its admitted default-enabled pattern plan');
 const count=readRequiredNumberFact(plan,1);let positions=[];
 for(let index=0;index<count;index+=1)positions=positions.concat([readRequiredNumberFact(plan,3+index*4)]);
 return positions;
}

/** The first profile has no default-environment, mutable parameter or async
 * composition proof. Zero parameter doc tags avoids inventing a documentation
 * name for an unnamed source pattern. Ordinary old signatures stay untouched.
 * @param {string} context @param {boolean} asynchronous @param {number} documentationParameters
 * @param {boolean} boundWrites @param {boolean} variableRedeclarations
 * @returns {boolean} */
export function acceptJavaScriptRecordParameterProfile(context, asynchronous, documentationParameters, boundWrites, variableRedeclarations) {
  return context === 'ordinary named function declaration' && !asynchronous
    && documentationParameters === 0 && !boundWrites && !variableRedeclarations;
}

/** Original local token positions from the complete default-enabled plan.
 * @param {number[]} plan @returns {number[]} */
export function readJavaScriptRecordFormalDefaultBindingPositions(plan){
 if(plan.length<2||readRequiredNumberFact(plan,0)<0)throw new RangeError('a record initializer census needs an admitted complete plan');
 let cursor=2,names=[];
 while(cursor<plan.length){
  const form=readRequiredNumberFact(plan,cursor);cursor+=2;
  if(form===0){names=names.concat([readRequiredNumberFact(plan,cursor)]);cursor+=1;}
  else if(form===1){const count=readRequiredNumberFact(plan,cursor+2);cursor+=3;for(let index=0;index<count;index+=1){names=names.concat([readRequiredNumberFact(plan,cursor+1)]);cursor+=4;}}
  else throw new RangeError('a record initializer census needs actual formal instructions');
 }
 return names;
}
/** @returns {string} */
export function readJavaScriptRecordFormalDefaultOperation(){return 'readJavaScriptRecordFormalDefault';}
/** Actual source operands and known field projection, before ordinary defaults.
 * @param {string} language @param {number} count @param {boolean} spread
 * @param {string} key @returns {boolean} */
export function acceptJavaScriptRecordFormalDefaultSource(language,count,spread,key){return language==='JavaScript'&&count===3&&!spread&&key==='str';}
/** The represented ordinary closed record has fixed own fields and the
 * standard Object prototype. Its inherited properties need their own value
 * protocol; they cannot be replaced with undefined. Open shapes must resolve.
 * @param {string} kind @param {boolean} open @param {boolean} present
 * @param {string} name @returns {string} */
export function readJavaScriptRecordFormalDefaultFieldForm(kind,open,present,name){
 if(kind==='var'||kind==='literal')return 'wait';
 if(kind!=='record')return 'field';
 if(present)return 'present';
 if(open)return 'wait';
 if(name==='__proto__'||name==='constructor'||name==='hasOwnProperty'||name==='isPrototypeOf'||name==='propertyIsEnumerable'||name==='toLocaleString'||name==='toString'||name==='valueOf'||name==='__defineGetter__'||name==='__defineSetter__'||name==='__lookupGetter__'||name==='__lookupSetter__')return 'prototype';
 return 'absent';
}

/** Builtin spelling alone cannot replace an actual local/module/context owner.
 * @param {string} name @param {boolean} shadowed @returns {boolean} */
export function acceptJavaScriptStandardBuiltinOwner(name, shadowed) {
  return (name === 'Map' || name === 'Set' || name === 'console'
    || name === 'Math' || name === 'Number' || name === 'BigInt' || name === 'Boolean' || name === 'Array'
    || name === 'Object' || name === 'String' || name === 'Error'
    || name === 'RangeError' || name === 'TypeError' || name === 'NaN'
    || name === 'Infinity' || name === 'undefined' || name === 'isFinite'
    || name === 'isNaN') && !shadowed;
}

/** Actual bounded body token facts, including the closing brace. A strict
 * directive returns its actual token position; -1 means no strict directive;
 * -2 refuses malformed or unsupported prologue transport. Raw spelling matters:
 * a cooked string with escapes is an ordinary directive, not Use Strict.
 * @param {string[]} kinds @param {string[]} values @param {string[]} raws
 * @param {boolean[]} lineBreaks @param {number} start @returns {number} */
export function findJavaScriptRecordParameterStrictDirective(kinds, values, raws, lineBreaks, start) {
  if (kinds.length !== values.length || kinds.length !== raws.length || kinds.length !== lineBreaks.length
    || !Number.isInteger(start) || start < 0 || start >= kinds.length) return -2;
  let at = start;
  while (at < kinds.length) {
    const current = readRequiredTextFact(kinds, at);
    if (current === 'eof') return -2;
    if (current !== 'string') return -1;
    const raw = readRequiredTextFact(raws, at);
    const strict = raw === '"use strict"' || raw === "'use strict'";
    const next = at + 1;
    if (next >= kinds.length) return -2;
    const kind = readRequiredTextFact(kinds, next), value = readRequiredTextFact(values, next);
    if (kind === 'punct' && value === ';') {
      if (strict) return at;
      at = next + 1;
      continue;
    }
    if (kind === 'punct' && value === '}') return strict ? at : -1;
    if (kind === 'eof') return -2;
    if (!readRequiredBooleanFact(lineBreaks, next)) return -1;
    if (continuesJavaScriptDirectiveExpression(kind, value)) return -1;
    if (kind === 'string' || kind === 'identifier' || kind === 'punct' && (value === '{' || value === '++' || value === '--')) {
      if (strict) return at;
      at = next;
      continue;
    }
    return -2;
  }
  return -2;
}

/** Expression continuations prevent automatic semicolon insertion after a
 * candidate directive. Small source guards keep normal generated formatting
 * bounded; all original punctuators and keyword cases remain represented.
 * @param {string} kind @param {string} value @returns {boolean} */
function continuesJavaScriptDirectiveExpression(kind, value) {
  if (kind === 'template' || kind === 'interpolation') return true;
  if (kind === 'identifier') return value === 'in' || value === 'instanceof';
  if (kind !== 'punct') return false;
  if (value === '(' || value === '[' || value === '.') return true;
  if (value === '?.' || value === '+' || value === '-') return true;
  if (value === '*' || value === '/' || value === '%') return true;
  if (value === '**' || value === '<<' || value === '>>') return true;
  if (value === '>>>' || value === '<' || value === '>') return true;
  if (value === '<=' || value === '>=' || value === '==') return true;
  if (value === '!=' || value === '===' || value === '!==') return true;
  if (value === '&' || value === '^' || value === '|') return true;
  if (value === '&&' || value === '||' || value === '??') return true;
  return value === '?' || value === ',';
}

/** Actual local assignment facts choose the ordinary local binding mechanism.
 * The historical read-only profile predicate retains its original contract.
 * @param {boolean} written @returns {string} */
export function readJavaScriptRecordParameterBindingStatement(written){return written?'let':'const';}
