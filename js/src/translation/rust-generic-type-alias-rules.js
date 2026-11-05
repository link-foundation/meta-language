// Source draft consumed by the ordinary JavaScript alias frontend.
// These decisions consume actual token/binding facts; they never substitute
// source spellings or manufacture a source type owner.
import {readRequiredTextFact,readRequiredBooleanFact} from './fact-column-rules.js';
import {readRustTypeAliasExpansionError,acceptRustTypeAliasEmptyVectorMember} from './rust-type-alias-rules.js';
import {acceptRustSourceSignaturePrimitive} from './rust-source-signature-rules.js';
import {readRustStandardSumSourceArgumentCount,readRustStandardSumSourceDepthLimit,readRustStandardSumSourceConstructorArity} from './rust-standard-sum-source-rules.js';
import {acceptRustSourceArrayOwner,acceptRustSourceArrayElement} from './rust-array-rules.js';
import {readRustTypeClosingTokenParts} from './rust-type-token-rules.js';
import {readRustFunctionPointerTypeError,readRustFunctionPointerHeaderTypeError,readRustFunctionPointerParameterLimit,readRustFunctionPointerOwnedDomainError,acceptRustFunctionPointerSafeSource} from './rust-function-pointer-rules.js';
import {acceptRustRetainedFunctionOwner} from './rust-source-owner-retention-rules.js';

/** Restrict this first declaration profile to explicit, unconstrained type
 * parameters. Raw spelling remains necessary to distinguish r#gen from gen.
 * This is a token-column policy, not a lexer or a Rust identifier validator.
 * @param {string} name @param {string} raw @returns {boolean} */
function acceptParameterName(name,raw){
 if(name===''||name==='_'||['Self','self','super','crate'].includes(name))return false;
 // The actual Rust lexer has already validated the identifier and normalized
 // its value to NFC. Original raw bytes can therefore differ from that value.
 // Only the source raw-prefix distinction belongs to this parameter policy.
 if(raw.startsWith('r#'))return true;
 if(raw==='')return false;
 return !['as','async','await','break','const','continue','dyn','else','enum','extern','false','fn','for','if','impl','in','let','loop','match','mod','move','mut','pub','ref','return','static','struct','trait','true','type','unsafe','use','where','while','abstract','become','box','do','final','gen','macro','override','priv','try','typeof','unsized','virtual','yield'].includes(name);
}

/** Read only the <T,U> portion of an already discovered actual alias item.
 * Success: parameters, closing token index, unconsumed punctuation, names.
 * The caller partitions the actual closing token with the existing token rule;
 * >= therefore retains its = and original source offsets. No cursor mutates.
 * Parameter limits are explicit proposed bounds for this source draft profile.
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

/** Additional directory retains original token coordinates for every default.
 * Rows are name, default start token, end token, end punctuation offset; -1
 * marks no default. The legacy explicit-parameter projection remains intact.
 * @param {string[]} names @param {string[]} starts @param {string[]} ends
 * @param {string[]} offsets @param {number} close @param {string} remainder
 * @returns {string[]} */
function parameterDirectory(names,starts,ends,offsets,close,remainder){
 if(remainder!==''&&remainder!=='=')return ['error','malformed generic alias parameter delimiter'];
 const result=['parameters',String(close),remainder,String(names.length)];
 for(let at=0;at<names.length;at+=1)result.push(readRequiredTextFact(names,at),readRequiredTextFact(starts,at),readRequiredTextFact(ends,at),readRequiredTextFact(offsets,at));
 return result;
}
/** @param {string[]} kinds @param {string[]} values @param {string[]} raws
 * @param {number} position @returns {string[]} */
export function readRustGenericTypeAliasParameterDirectory(kinds,values,raws,position){
 if(kinds.length!==values.length||raws.length!==values.length)return ['error','unaligned generic alias token columns'];
 if(position<0||position%1!==0||position>=values.length||readRequiredTextFact(kinds,position)!=='punct'||readRequiredTextFact(values,position)!=='<')return ['error','missing generic alias parameter list'];
 const names=[],starts=[],ends=[],offsets=[];let at=position+1,defaultSeen=false;
 while(at<values.length){
  const name=readRequiredTextFact(values,at);
  if(readRequiredTextFact(kinds,at)!=='identifier'||!acceptParameterName(name,readRequiredTextFact(raws,at)))return ['error','generic alias needs explicit unconstrained type parameters'];
  if(names.includes(name))return ['error','duplicate generic alias type parameter'];
  if(names.length>=64)return ['error','generic alias parameter limit exceeded'];
  names.push(name);at+=1;
  if(at>=values.length)return ['error','unclosed generic alias parameter list'];
  let punctuation=readRequiredTextFact(values,at);
  if(readRequiredTextFact(kinds,at)!=='punct')return ['error','generic alias parameter bounds and defaults need their complete interpretation'];
  if(punctuation==='='){
   defaultSeen=true;at+=1;const first=at;let angles=0,found=false;
   while(at<values.length){
    const kind=readRequiredTextFact(kinds,at),value=readRequiredTextFact(values,at);
    if(kind==='eof'||kind==='punct'&&(value===';'||value==='}'))return ['error','unclosed generic alias default type'];
    if(kind==='punct'&&(value==='('||value==='['||value==='{')){const after=groupAfter(kinds,values,at);if(after<0)return ['error','unclosed generic alias default group'];at=after;continue;}
    if(kind==='punct'&&value==='<')angles+=1;
    const closing=value==='>'?['>']:readRustTypeClosingTokenParts(value);
    if(kind==='punct'&&closing.length>0){
     for(let offset=0;offset<closing.length&&readRequiredTextFact(closing,offset)==='>';offset+=1){
      if(angles>0){angles-=1;continue;}
      if(first===at&&offset===0)return ['error','empty generic alias default type'];
      starts.push(String(first));ends.push(String(at));offsets.push(String(offset));
      return parameterDirectory(names,starts,ends,offsets,at,closing.slice(offset+1).join(''));
     }
    }
    if(kind==='punct'&&value===','&&angles===0){
     if(first===at)return ['error','empty generic alias default type'];
     starts.push(String(first));ends.push(String(at));offsets.push('0');found=true;break;
    }
    at+=1;
   }
   if(!found)return ['error','unclosed generic alias parameter list'];
   punctuation=readRequiredTextFact(values,at);
  }else{
   if(defaultSeen)return ['error','generic alias parameters after a default need their own defaults'];
   starts.push('-1');ends.push('-1');offsets.push('0');
  }
  if(punctuation===','){
   at+=1;if(at>=values.length)return ['error','unclosed generic alias parameter list'];
   punctuation=readRequiredTextFact(values,at);
   if(punctuation!=='>'&&punctuation!=='>=')continue;
  }
  if(punctuation==='>'||punctuation==='>=')return parameterDirectory(names,starts,ends,offsets,at,punctuation==='>='?'=':'');
  return ['error','generic alias parameter bounds and defaults need their complete interpretation'];
 }
 return ['error','unclosed generic alias parameter list'];
}

/** All declared defaults are checked, including defaults overridden at a use.
 * @param {string[]} parameters @param {number} position @param {number} count
 * @param {string[]} occurrences @returns {string[]} */
export function readRustGenericTypeAliasDefaultSubstitutionPlan(parameters,position,count,occurrences){
 const directory=readRustGenericTypeAliasBindingPlan(parameters,parameters.length,[]);if(directory[0]!=='substitution')return directory;
 if(position<0||position%1!==0||position>=parameters.length||count!==position)return ['error','generic alias default needs every earlier actual parameter binding'];
 for(let at=0;at<occurrences.length;at+=1){const identity=readRequiredTextFact(occurrences,at),index=parameters.indexOf(identity);if(index<0||index>=position)return ['error','generic alias defaults cannot reference their own or later parameter bindings'];}
 return position===0?['substitution']:readRustGenericTypeAliasBindingPlan(parameters.slice(0,position),count,occurrences);
}
/** @param {boolean[]} defaults @param {number} count @param {boolean} explicit
 * @returns {string[]} */
export function readRustGenericTypeAliasDefaultArgumentPlan(defaults,count,explicit){
 if(defaults.length===0||defaults.length>64)return ['error','generic alias needs a bounded explicit parameter directory'];
 if(count<0||count%1!==0||count>defaults.length)return ['error','generic alias argument arity differs from its declaration'];
 const result=['arguments'];
 for(let at=count;at<defaults.length;at+=1){if(!readRequiredBooleanFact(defaults,at))return ['error',explicit?'generic alias argument arity differs from its declaration':'generic alias arguments must be explicit'];result.push(String(at));}
 return result;
}
/** Qualified nominal path tails are actual definition owners, not parameter
 * uses merely because their spelling matches a parameter name.
 * @param {string[]} parameters @param {string} name @param {boolean} qualified
 * @returns {boolean} */
export function acceptRustGenericTypeAliasConcreteIdentifier(parameters,name,qualified){return qualified||!parameters.includes(name);}
/** The existing primitive/member readers inspect at most sixteen actual path
 * segments. This preparation walk follows that same bounded cursor window.
 * @returns {number} */
export function readRustGenericTypeAliasDefaultAccessHeadLimit(){return 16;}
/** The first inherent discovery pass may precede its nominal declaration.
 * Complete alias validation still runs after that catalog is collected.
 * @param {boolean} collecting @param {boolean} discovery @returns {string} */
export function readRustGenericTypeAliasDefaultOwnerPhase(collecting,discovery){return collecting&&discovery?'defer':'prepare';}
/** Only actual safe fn syntax starts a function template. A raw identifier
 * r#fn remains a parameter/type binding, and qualifiers are never discarded.
 * @param {string} value @param {string} raw @returns {boolean} */
export function acceptRustGenericTypeAliasFunctionTemplateHead(value,raw){return acceptRustFunctionPointerSafeSource(value,raw);}
/** @param {string} value @param {string} raw @returns {boolean} */
export function acceptRustGenericTypeAliasFunctionParameterName(value,raw){return value==='_'?raw==='_':acceptParameterName(value,raw);}
/** Symbolic wellformedness checks size only. Signature-domain Copy facts are
 * read from actual substituted origins at materialization, never invented.
 * @param {number} count @returns {string} */
export function readRustGenericTypeAliasFunctionTemplateError(count){return !Number.isInteger(count)||count<0?'invalid generic function template parameter count':count>readRustFunctionPointerParameterLimit()?'the complete pointer signature exceeds its source budget':'';}

/** A substitution plan indexes complete actual argument origins. It does not
 * copy types, re-resolve names, infer arguments, or prove a template well formed.
 * Every occurrence must already have its actual resolved binding identity.
 * Nonparameter identities retain their definition context (-1). Parameter
 * identities select an argument index; the adapter must retain its whole origin.
 * @param {string[]} parameters @param {boolean[]} argumentsProven
 * @param {string[]} occurrences @returns {string[]} */
export function readRustGenericTypeAliasSubstitutionPlan(parameters,argumentsProven,occurrences){
 const binding=readRustGenericTypeAliasBindingPlan(parameters,argumentsProven.length,occurrences);
 if(binding[0]!=='substitution')return binding;
 for(let at=0;at<argumentsProven.length;at+=1){if(!readRequiredBooleanFact(argumentsProven,at))return ['error','generic alias arguments need complete checked source origins'];}
 return binding;
}
/** Private syntax headers retain complete actual type/owner facts while their
 * source sum inventory awaits final admission. This is never a proof of Copy.
 * @param {string[]} parameters @param {boolean} header
 * @param {boolean[]} argumentsTyped @param {boolean[]} argumentsProven
 * @param {boolean[]} argumentsProvisional @param {string[]} occurrences
 * @returns {string[]} */
export function readRustGenericTypeAliasHeaderSubstitutionPlan(parameters,header,argumentsTyped,argumentsProven,argumentsProvisional,occurrences){
 if(!header||argumentsTyped.length!==argumentsProven.length||argumentsTyped.length!==argumentsProvisional.length)return ['error','generic alias arguments need complete checked source origins'];
 const plan=readRustGenericTypeAliasBindingPlan(parameters,argumentsTyped.length,occurrences);if(plan[0]!=='substitution')return plan;
 for(let at=0;at<argumentsTyped.length;at+=1){if(!readRequiredBooleanFact(argumentsTyped,at)||!readRequiredBooleanFact(argumentsProven,at)&&!readRequiredBooleanFact(argumentsProvisional,at))return ['error','generic alias arguments need complete checked source origins'];}
 return plan;
}
/** The provisional premise comes from the independently resolved actual data
 * row, not a marker on the caller's expression origin.
 * @param {boolean} header @param {string} kind @param {number} wrappers @param {string} ownerKind
 * @param {string} owner @param {number} children @param {string} actualOwner
 * @param {number} actualArguments @param {number} actualWrappers
 * @param {boolean} provisional @returns {boolean} */
export function acceptRustGenericTypeAliasHeaderStandardSumOrigin(header,kind,wrappers,ownerKind,owner,children,actualOwner,actualArguments,actualWrappers,provisional){
 return header&&provisional&&kind==='named'&&wrappers===0&&actualWrappers===0&&ownerKind==='library'&&owner===actualOwner&&children===actualArguments&&readRustGenericTypeAliasStandardSumOwner(actualOwner,'',actualArguments)!=='';
}

/** Symbolic application selects whole argument templates, without claiming
 * they are concrete checked origins. All occurrences already have resolved
 * declaration-scoped binding identities.
 * @param {string[]} parameters @param {number} count
 * @param {string[]} occurrences @returns {string[]} */
export function readRustGenericTypeAliasBindingPlan(parameters,count,occurrences){
 if(parameters.length===0||parameters.length>64)return ['error','generic alias needs a bounded explicit parameter directory'];
 if(parameters.length!==count)return ['error','generic alias argument arity differs from its declaration'];
 if(occurrences.length>262144)return ['error','generic alias occurrence limit exceeded'];
 for(let at=0;at<parameters.length;at+=1){
  const identity=readRequiredTextFact(parameters,at);
  if(identity===''||parameters.slice(0,at).includes(identity))return ['error','generic alias parameter binding identities must be actual and distinct'];
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

/** @param {number} depth @returns {string} */
export function readRustGenericTypeAliasTemplateDepthError(depth){return depth>=128?'generic alias template depth limit exceeded':'';}
/** @param {string[]} parameters @param {string[]} used @returns {string} */
export function readRustGenericTypeAliasUnusedParameterError(parameters,used){
 for(let at=0;at<parameters.length;at+=1){if(!used.includes(readRequiredTextFact(parameters,at)))return 'unused generic alias type parameter';}
 return '';
}
/** Closed first template profile. Tuple admission still checks each concrete
 * instantiation through the existing source product rules.
 * @param {string} kind @param {number} wrappers @param {string} ownerKind
 * @param {string} owner @param {string} bits @param {boolean} signed
 * @param {number} children @param {boolean} proven @returns {boolean} */
export function acceptRustGenericTypeAliasConcreteTemplate(kind,wrappers,ownerKind,owner,bits,signed,children,proven){
 return proven&&wrappers===0&&children===0&&(kind==='unit'?ownerKind==='unit'&&owner==='':ownerKind==='primitive'&&['fixed','bool','float'].includes(kind)&&acceptRustSourceSignaturePrimitive(kind,owner,bits,signed));
}
/** Concrete callable leaves retain the actual parsed safe fn pointer domain.
 * This operation does not introduce parameter-dependent callable templates.
 * @param {boolean} ordinary @param {string} kind @param {number} wrappers
 * @param {string} ownerKind @param {number} ownerPaths @param {number} children
 * @param {number} parameters @param {boolean[]} parameterCopies
 * @param {boolean} resultCopy @param {boolean} resultTyped
 * @param {boolean} proven @param {boolean} copy @param {boolean} closure
 * @returns {boolean} */
export function acceptRustGenericTypeAliasConcreteCallableTemplate(ordinary,kind,wrappers,ownerKind,ownerPaths,children,parameters,parameterCopies,resultCopy,resultTyped,proven,copy,closure){
 return wrappers===0&&!closure&&resultTyped&&parameterCopies.length===parameters&&acceptRustRetainedFunctionOwner('Rust source function signature 2',ownerKind,kind,ownerPaths,children,parameters,false)&&proven&&copy&&readRustFunctionPointerTypeError(ordinary,parameterCopies,resultCopy)==='';
}
/** Owned value domains are separate from the unchanged Copy-only leaf.
 * The callable itself is still an actually admitted capture-free fn pointer.
 * @param {boolean} ordinary @param {string} kind @param {number} wrappers
 * @param {string} ownerKind @param {number} ownerPaths @param {number} children
 * @param {number} parameters @param {string[]} roles @param {string} resultRole
 * @param {boolean} resultTyped @param {boolean} proven @param {boolean} copy
 * @param {boolean} closure @param {number} minor @returns {boolean} */
export function acceptRustGenericTypeAliasOwnedCallableTemplate(ordinary,kind,wrappers,ownerKind,ownerPaths,children,parameters,roles,resultRole,resultTyped,proven,copy,closure,minor){
 return wrappers===0&&!closure&&resultTyped&&roles.length===parameters&&acceptRustRetainedFunctionOwner('Rust source function signature 2',ownerKind,kind,ownerPaths,children,parameters,false)&&proven&&copy&&readRustFunctionPointerOwnedDomainError(ordinary,roles,resultRole,minor)==='';
}
/** Private fn-pointer children may await nominal/sum admission. Their existing
 * parser header flag is transported without upgrading proven or Copy facts.
 * @param {boolean} ordinary @param {string} kind @param {number} wrappers
 * @param {string} ownerKind @param {number} ownerPaths @param {number} children
 * @param {number} parameters @param {boolean[]} parameterCopies
 * @param {boolean} resultCopy @param {boolean} resultTyped
 * @param {boolean} header @param {boolean} pointerHeader @param {boolean} closure
 * @returns {boolean} */
export function acceptRustGenericTypeAliasHeaderCallableTemplate(ordinary,kind,wrappers,ownerKind,ownerPaths,children,parameters,parameterCopies,resultCopy,resultTyped,header,pointerHeader,closure){
 return header&&pointerHeader&&wrappers===0&&!closure&&resultTyped&&parameterCopies.length===parameters&&acceptRustRetainedFunctionOwner('Rust source function signature 2',ownerKind,kind,ownerPaths,children,parameters,false)&&readRustFunctionPointerHeaderTypeError(ordinary,parameterCopies,resultCopy,header)==='';
}
/** A concrete nominal leaf retains its actual checked definition identity.
 * Copy is deliberately not a premise of symbolic template wellformedness.
 * @param {string} kind @param {number} wrappers @param {string} ownerKind
 * @param {string} owner @param {number} children @param {boolean} proven
 * @param {string} actualDataOwner @returns {boolean} */
export function acceptRustGenericTypeAliasNamedTemplate(kind,wrappers,ownerKind,owner,children,proven,actualDataOwner){
 return kind==='named'&&wrappers===0&&ownerKind==='declaration'&&children===0&&proven&&actualDataOwner!==''&&owner===actualDataOwner.split('.').join('::');
}
/** Actual source ownership was resolved independently. Constructor spellings
 * cannot become type templates, and every symbolic type argument is explicit.
 * @param {string} owner @param {string} constructor @param {number} count
 * @returns {string} */
export function readRustGenericTypeAliasStandardSumOwner(owner,constructor,count){
 return constructor===''&&count===readRustStandardSumSourceArgumentCount(owner)&&count>0?owner:'';
}
/** The adapter must resolve the actual complete stored standard-sum origin,
 * rather than trust a same-shaped marker or caller-local type spelling.
 * @param {string} kind @param {number} wrappers @param {string} owner
 * @param {number} count @param {boolean} proven @param {boolean} actualProven
 * @param {boolean} actualCopy @param {number} actualWrappers @returns {boolean} */
export function acceptRustGenericTypeAliasStandardSumLeaf(kind,wrappers,owner,count,proven,actualProven,actualCopy,actualWrappers){return kind==='named'&&wrappers===0&&proven&&actualProven&&actualCopy&&actualWrappers===0&&readRustGenericTypeAliasStandardSumOwner(owner,'',count)!=='';}
/** Independent stored ownership admission never becomes Copy evidence.
 * @param {string} kind @param {number} wrappers @param {string} owner
 * @param {number} count @param {boolean} proven @param {boolean} actualProven
 * @param {number} actualWrappers @param {string} profile @returns {boolean} */
export function acceptRustGenericTypeAliasOwnedStandardSumLeaf(kind,wrappers,owner,count,proven,actualProven,actualWrappers,profile){return profile==='owned'&&kind==='named'&&wrappers===0&&proven&&actualProven&&actualWrappers===0&&readRustGenericTypeAliasStandardSumOwner(owner,'',count)!=='';}
/** @param {number} depth @returns {string} */
export function readRustGenericTypeAliasStandardSumDepthError(depth){return depth>readRustStandardSumSourceDepthLimit()?'generic alias standard sum exceeds its shared depth bound':'';}

/** Source/import resolution and both catalog/alias-root shadow facts belong
 * to the actual definition context. No allocator or custom vector is inferred.
 * @param {string} path @param {boolean} shadowed @returns {string} */
export function readRustGenericTypeAliasVectorOwner(path,shadowed){return acceptRustSourceArrayOwner(path,shadowed)?'std::vec::Vec':'';}
/** Symbolic Vec parameters need no Copy premise. Actual element admission is
 * a separate operation at each concrete instantiation.
 * @param {string} owner @param {number} count @returns {boolean} */
export function acceptRustGenericTypeAliasVectorArguments(owner,count){return owner==='std::vec::Vec'&&count===1;}
/** @param {string} kind @param {number} wrappers @param {string} ownerKind
 * @param {string} owner @param {number} children @param {boolean} proven
 * @param {string} textModel @returns {boolean} */
export function acceptRustGenericTypeAliasOwnedStringTemplate(kind,wrappers,ownerKind,owner,children,proven,textModel){return kind==='string'&&wrappers===0&&ownerKind==='library'&&owner==='std::string::String'&&children===0&&proven&&textModel==='owned string';}
/** Reuse the actual existing element domain and exact checked source owners.
 * Owned String is an admitted element without becoming a scalar Copy value.
 * @param {string} kind @param {number} wrappers @param {string} ownerKind
 * @param {string} owner @param {string} bits @param {boolean} signed
 * @param {number} children @param {boolean} proven @param {string} textModel
 * @returns {string} */
export function readRustGenericTypeAliasVectorElementError(kind,wrappers,ownerKind,owner,bits,signed,children,proven,textModel){
 const scalar=acceptRustGenericTypeAliasConcreteTemplate(kind,wrappers,ownerKind,owner,bits,signed,children,proven);
 const string=acceptRustGenericTypeAliasOwnedStringTemplate(kind,wrappers,ownerKind,owner,children,proven,textModel);
 return acceptRustSourceArrayElement(kind,wrappers!==0)&&(scalar||string)?'':'generic vector alias elements need an actual direct admitted scalar, Unit or owned String source origin';
}
/** Select only an existing constructor from a complete instantiated alias.
 * The sum facts come from the actual named declaration's stored source origin.
 * @param {string} kind @param {number} wrappers @param {string} ownerKind
 * @param {string} owner @param {number} children @param {string} element
 * @param {string} model @param {boolean} proven @param {string} actualSumOwner
 * @param {number} actualArguments @param {boolean} actualProven
 * @param {boolean} actualCopy @param {string} member @returns {string[]} */
export function readRustGenericTypeAliasConstructorPlan(kind,wrappers,ownerKind,owner,children,element,model,proven,actualSumOwner,actualArguments,actualProven,actualCopy,member){
 if(acceptRustTypeAliasEmptyVectorMember(kind,ownerKind,owner,children,element,model,proven,wrappers,member))return ['vector constructor','0'];
 return actualCopy&&proven&&actualProven?standardSumConstructorShape(kind,wrappers,ownerKind,owner,children,actualSumOwner,actualArguments,member):[];
}
/** @param {string} kind @param {number} wrappers @param {string} ownerKind
 * @param {string} owner @param {number} children
 * @param {string} actualSumOwner @param {number} actualArguments
 * @param {string} member @returns {string[]} */
function standardSumConstructorShape(kind,wrappers,ownerKind,owner,children,actualSumOwner,actualArguments,member){
 const arity=readRustStandardSumSourceConstructorArity(actualSumOwner,member);
 return kind==='named'&&wrappers===0&&ownerKind==='library'&&owner===actualSumOwner&&children===actualArguments&&actualArguments===readRustStandardSumSourceArgumentCount(actualSumOwner)&&arity>=0?['sum constructor',String(arity)]:[];
}
/** This separate operation admits only the actual stored owned-sum profile.
 * @param {string} kind @param {number} wrappers @param {string} ownerKind
 * @param {string} owner @param {number} children @param {boolean} proven
 * @param {string} actualSumOwner @param {number} actualArguments
 * @param {boolean} actualProven @param {string} profile @param {string} member
 * @returns {string[]} */
export function readRustGenericTypeAliasOwnedConstructorPlan(kind,wrappers,ownerKind,owner,children,proven,actualSumOwner,actualArguments,actualProven,profile,member){
 return profile==='owned'&&proven&&actualProven?standardSumConstructorShape(kind,wrappers,ownerKind,owner,children,actualSumOwner,actualArguments,member):[];
}
/** Separate private-header path; no admitted/proven/Copy premise is supplied.
 * @param {boolean} header @param {boolean} provisional @param {string} kind
 * @param {number} wrappers @param {string} ownerKind @param {string} owner
 * @param {number} children @param {string} actualOwner
 * @param {number} actualArguments @param {number} actualWrappers
 * @param {string} member @returns {string[]} */
export function readRustGenericTypeAliasHeaderConstructorPlan(header,provisional,kind,wrappers,ownerKind,owner,children,actualOwner,actualArguments,actualWrappers,member){
 return acceptRustGenericTypeAliasHeaderStandardSumOrigin(header,kind,wrappers,ownerKind,owner,children,actualOwner,actualArguments,actualWrappers,provisional)?standardSumConstructorShape(kind,wrappers,ownerKind,owner,children,actualOwner,actualArguments,member):[];
}

/** @param {string[]} kinds @param {string[]} values @param {number} position @returns {number} */
function groupAfter(kinds,values,position){
 const open=readRequiredTextFact(values,position);let stack=[open==='{'?'}':open==='['?']':')'];
 for(let at=position+1;at<values.length;at+=1){
  if(readRequiredTextFact(kinds,at)!=='punct')continue;
  const value=readRequiredTextFact(values,at);
  if(value==='{'||value==='['||value==='(')stack=[...stack,value==='{'?'}':value==='['?']':')'];
  else if(value==='}'||value===']'||value===')'){
   if(value!==readRequiredTextFact(stack,stack.length-1))return -1;
   stack=stack.slice(0,-1);if(stack.length===0)return at+1;
  }
 }
 return -1;
}

/** Additive declaration directory: aliases; then variable rows containing
 * qualified name, keyword, RHS start/end, after item, parameter count, names.
 * The original nongeneric five-field API is unchanged. Module/body opacity and
 * original source token coordinates follow that API; no generated files edit.
 * @param {string[]} kinds @param {string[]} values @param {string[]} raws
 * @returns {string[]} */
export function readRustGenericTypeAliasDeclarationPlan(kinds,values,raws){
 if(kinds.length!==values.length||raws.length!==values.length)return ['error','unaligned source type alias columns'];
 const result=['aliases'];let names=[],owners=[],closes=[],position=0;
 while(position<values.length){
  const kind=readRequiredTextFact(kinds,position),value=readRequiredTextFact(values,position);
  if(closes.length>0&&position===Number(readRequiredTextFact(closes,closes.length-1))){closes=closes.slice(0,-1);owners=owners.slice(0,-1);position+=1;continue;}
  if(kind==='keyword'&&value==='mod'&&position+2<values.length&&readRequiredTextFact(kinds,position+1)==='identifier'&&readRequiredTextFact(values,position+2)==='{'){
   const after=groupAfter(kinds,values,position+2);if(after<0)return result;
   owners=[...owners,readRequiredTextFact(values,position+1)];closes=[...closes,String(after-1)];position+=3;continue;
  }
  if(kind==='keyword'&&value==='type'){
   if(position+3>=values.length||readRequiredTextFact(kinds,position+1)!=='identifier')return ['error','generic, constrained or malformed type aliases require their complete parameter interpretation'];
   const name=[...owners,readRequiredTextFact(values,position+1)].join('.');if(names.includes(name))return ['error','duplicate source type alias'];names=[...names,name];
   let first=position+3,parameters=[];
   if(readRequiredTextFact(values,position+2)==='<'){
    const plan=readRustGenericTypeAliasParameterDirectory(kinds,values,raws,position+2);if(plan[0]!=='parameters')return plan;
    const close=Number(readRequiredTextFact(plan,1)),count=Number(readRequiredTextFact(plan,3));
    for(let at=0;at<count;at+=1)parameters.push(readRequiredTextFact(plan,4+at*4));
    if(readRequiredTextFact(plan,2)==='=')first=close+1;
    else {if(close+1>=values.length||readRequiredTextFact(values,close+1)!=='=')return ['error','generic alias bounds and where clauses need their complete interpretation'];first=close+2;}
   }else if(readRequiredTextFact(values,position+2)!=='=')return ['error','generic, constrained or malformed type aliases require their complete parameter interpretation'];
   let end=first;
   while(end<values.length){
    const boundary=readRequiredTextFact(values,end);
    if(readRequiredTextFact(kinds,end)==='punct'&&boundary===';')break;
    if(readRequiredTextFact(kinds,end)==='punct'&&(boundary==='{'||boundary==='['||boundary==='(')){const after=groupAfter(kinds,values,end);if(after<0)return ['error','unclosed source type alias group'];end=after;}
    else if(readRequiredTextFact(kinds,end)==='eof'||readRequiredTextFact(kinds,end)==='punct'&&boundary==='}')return ['error','unclosed source type alias declaration'];
    else end+=1;
   }
   if(end>=values.length||end===first)return ['error','empty or unclosed source type alias declaration'];
   result.push(name,String(position),String(first),String(end),String(end+1),String(parameters.length),...parameters);
   if(names.length>10000)return ['error','source type alias declaration limit exceeded'];
   position=end+1;continue;
  }
  if(kind==='punct'&&(value==='{'||value==='['||value==='(')){const after=groupAfter(kinds,values,position);if(after<0)return result;position=after;continue;}
  position+=1;
 }
 return result;
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
