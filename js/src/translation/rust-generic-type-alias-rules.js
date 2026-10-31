// Unintegrated source draft. Frontend generic-alias refusals remain in force.
// These decisions consume actual token/binding facts; they never substitute
// source spellings or manufacture a source type owner.
import {readRequiredTextFact,readRequiredBooleanFact} from './fact-column-rules.js';
import {readRustTypeAliasExpansionError} from './rust-type-alias-rules.js';

/** Restrict this first declaration profile to explicit, unconstrained type
 * parameters. Raw spelling remains necessary to distinguish r#gen from gen.
 * This is a token-column policy, not a lexer or a Rust identifier validator.
 * @param {string} name @param {string} raw @returns {boolean} */
function acceptParameterName(name,raw){
 if(name===''||name==='_'||['Self','self','super','crate'].includes(name))return false;
 if(raw==='r#'+name)return true;
 if(raw!==name)return false;
 return !['as','async','await','break','const','continue','dyn','else','enum','extern','false','fn','for','if','impl','in','let','loop','match','mod','move','mut','pub','ref','return','static','struct','trait','true','type','unsafe','use','where','while','abstract','become','box','do','final','gen','macro','override','priv','try','typeof','unsized','virtual','yield'].includes(name);
}

/** Read only the <T,U> portion of an already discovered actual alias item.
 * Success: parameters, closing token index, unconsumed punctuation, names.
 * The caller partitions the actual closing token with the existing token rule;
 * >= therefore retains its = and original source offsets. No cursor mutates.
 * Parameter limits are explicit proposed bounds for this unintegrated profile.
 * @param {string[]} kinds @param {string[]} values @param {string[]} raws
 * @param {number} position @returns {string[]} */
export function readRustGenericTypeAliasParameterPlan(kinds,values,raws,position){
 if(kinds.length!==values.length||raws.length!==values.length)return ['error','unaligned generic alias token columns'];
 if(position<0||position%1!==0||position>=values.length)return ['error','missing generic alias parameter list'];
 if(readRequiredTextFact(kinds,position)!=='punct'||readRequiredTextFact(values,position)!=='<')return ['error','missing generic alias parameter list'];
 const names=[];let at=position+1;
 while(at<values.length){
  const name=readRequiredTextFact(values,at);
  if(readRequiredTextFact(kinds,at)!=='identifier'||!acceptParameterName(name,readRequiredTextFact(raws,at)))return ['error','generic alias needs explicit unconstrained type parameters'];
  if(names.includes(name))return ['error','duplicate generic alias type parameter'];
  if(names.length>=64)return ['error','generic alias parameter limit exceeded'];
  names.push(name);at+=1;
  if(at>=values.length)return ['error','unclosed generic alias parameter list'];
  let punctuation=readRequiredTextFact(values,at);
  if(readRequiredTextFact(kinds,at)!=='punct')return ['error','generic alias parameter bounds and defaults need their complete interpretation'];
  if(punctuation===','){
   at+=1;
   if(at>=values.length)return ['error','unclosed generic alias parameter list'];
   punctuation=readRequiredTextFact(values,at);
   if(punctuation!=='>'&&punctuation!=='>=')continue;
   if(readRequiredTextFact(kinds,at)!=='punct')return ['error','malformed generic alias parameter delimiter'];
  }
  if(punctuation==='>'||punctuation==='>=')return ['parameters',String(at),punctuation==='>='?'=':'',...names];
  return ['error','generic alias parameter bounds and defaults need their complete interpretation'];
 }
 return ['error','unclosed generic alias parameter list'];
}

/** A substitution plan indexes complete actual argument origins. It does not
 * copy types, re-resolve names, infer arguments, or prove a template well formed.
 * Every occurrence must already have its actual resolved binding identity.
 * Nonparameter identities retain their definition context (-1). Parameter
 * identities select an argument index; the adapter must retain its whole origin.
 * @param {string[]} parameters @param {boolean[]} argumentsProven
 * @param {string[]} occurrences @returns {string[]} */
export function readRustGenericTypeAliasSubstitutionPlan(parameters,argumentsProven,occurrences){
 if(parameters.length===0||parameters.length>64)return ['error','generic alias needs a bounded explicit parameter directory'];
 if(parameters.length!==argumentsProven.length)return ['error','generic alias argument arity differs from its declaration'];
 if(occurrences.length>262144)return ['error','generic alias occurrence limit exceeded'];
 for(let at=0;at<parameters.length;at+=1){
  const identity=readRequiredTextFact(parameters,at);
  if(identity===''||parameters.slice(0,at).includes(identity))return ['error','generic alias parameter binding identities must be actual and distinct'];
  if(!readRequiredBooleanFact(argumentsProven,at))return ['error','generic alias arguments need complete checked source origins'];
 }
 const plan=['substitution'];
 for(let at=0;at<occurrences.length;at+=1){
  const identity=readRequiredTextFact(occurrences,at);
  if(identity==='')return ['error','generic alias occurrence needs an actual resolved source binding'];
  // Do not equate a same-spelled definition owner with a parameter identity.
  plan.push(String(parameters.indexOf(identity)));
 }
 return plan;
}

/** Repeated declaration owners refuse even when arguments change. Tracking
 * only instantiated types could miss unbounded expanding recursion. This reuses
 * the existing depth/expansion bounds and supplies no new recursive semantics.
 * @param {string} declaration @param {string[]} declarations
 * @param {number} expansions @returns {string} */
export function readRustGenericTypeAliasExpansionError(declaration,declarations,expansions){
 if(declaration==='')return 'generic alias expansion needs actual declaration owners';
 for(let at=0;at<declarations.length;at+=1){if(readRequiredTextFact(declarations,at)==='')return 'generic alias expansion needs actual declaration owners';}
 if(expansions<0||expansions%1!==0)return 'invalid generic alias expansion count';
 return readRustTypeAliasExpansionError(declarations.length,expansions,declarations.includes(declaration));
}
