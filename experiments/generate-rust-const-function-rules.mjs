// JavaScript-first decisions for a bounded source const-qualification model.
import {fileURLToPath} from 'node:url';
import {readFileSync,writeFileSync} from 'node:fs';
import {writeProjectData,checkProjectData} from '../js/scripts/project-data.mjs';
import {DecoratorSet} from '../js/src/decorators.js';
import {formatProductRuleSource} from '../js/scripts/product-rule-style-corrections.mjs';
import {stripRustSourceHeaderStyleParentheses} from '../js/scripts/rust-source-header-library-style.mjs';
export const rustConstFunctionPolicy={
 version:1,model:'closed pure Rust const predicates 1',language:'Rust',keyword:'const',functionKeyword:'fn',
 primitiveTypes:['bool','float'],primitiveOwners:{bool:'bool',float:'f64'},
 interpretation:{inputDomain:'primitive Boolean and binary64 values',observations:['returned primitive Boolean value','owned const function qualification'],excludedObservations:['compile-time invocation and evaluation','compile-time diagnostics','public API visibility','external linkage and ABI']},
 fields:{proof:['model','language','sourceName','keyword','keywordSpan','functionKeywordSpan','headerSpan','bodySpan','interpretation'],
 declaration:['k','name','params','ret','body','span','fullName','modulePath','recursive','decreasing','mutual','generated','sourceSignature','sourceConstFunction'],
 parameter:['name','type','guard','default'],program:['schemaVersion','sourceLanguage','items','main','declarations','imports','externals','referenceSemantics','utf16Strings','outputThreaded','abortsThreaded'],
 expressions:{lit:['k','value','type','span'],var:['k','name','type','span'],unary:['k','op','arg','type','span'],binary:['k','op','left','right','type','domain','semantics','span']}},
 booleanOperations:['and','or','eq','ne'],numberOperations:['eq','ne','lt','le','gt','ge'],
 reservedOwners:['bool','f64','std','core','alloc','main'],emission:{from:'pub fn ',to:'pub const fn '},
 forbiddenNames:['as','async','await','break','const','continue','crate','dyn','else','enum','extern','false','fn','for','if','impl','in','let','loop','match','mod','move','mut','pub','ref','return','self','Self','static','struct','super','trait','true','type','unsafe','use','where','while','abstract','become','box','do','final','gen','macro','override','priv','try','typeof','unsized','virtual','yield'],
};
export function rustConstFunctionRules(policy=rustConstFunctionPolicy){
 const either=(value,values)=>values.map(item=>value+' === '+JSON.stringify(item)).join(' || ');
 return '// Generated from the linked const predicate policy. Do not edit.\n'
 +'/** @param {string} model @returns {boolean} */\nexport function acceptRustConstFunctionModel(model){return model === "" || model === '+JSON.stringify(policy.model)+';}\n'
 +'/** @param {string} kind @returns {boolean} */\nexport function acceptRustConstFunctionType(kind){return '+either('kind',policy.primitiveTypes)+';}\n'
 +'/** @param {string} kind @param {string} operation @returns {boolean} */\nexport function acceptRustConstFunctionBinary(kind,operation){return kind === "bool" && ('+either('operation',policy.booleanOperations)+') || kind === "float" && ('+either('operation',policy.numberOperations)+');}\n'
 +'/** @param {string} kind @param {boolean} dense @param {boolean} unrepresented @returns {boolean} */\nexport function acceptRustConstFunctionContainer(kind,dense,unrepresented){return !unrepresented && (kind === "record" || kind === "map" || (kind === "array" && dense));}\n'
 +'/** @param {number} value @param {boolean} negativeZero @returns {boolean} */\nexport function acceptRustConstFunctionOffset(value,negativeZero){return value >= 0 && value <= 9007199254740991 && value % 1 === 0 && !negativeZero;}\n'
 +'/** @param {number} start @param {number} end @param {boolean} nonempty @returns {boolean} */\nexport function acceptRustConstFunctionInterval(start,end,nonempty){return start <= end && (!nonempty || start < end);}\n'
 +'/** @param {number} code @param {boolean} first @returns {boolean} */\nexport function acceptRustConstFunctionNameCharacter(code,first){return code >= 95 && code <= 95 || code >= 65 && code <= 90 || code >= 97 && code <= 122 || !first && code >= 48 && code <= 57;}\n'
 +'/** @param {string} name @returns {boolean} */\nexport function acceptRustConstFunctionName(name){return name !== "" && name !== "_" && !('+either('name',policy.forbiddenNames)+');}\n'
 +'/** @param {number} value @returns {boolean} */\nexport function acceptRustConstFunctionFiniteLiteral(value){return Number.isFinite(value);}\n';
}
export function rustConstFunctionDecorators(policy=rustConstFunctionPolicy){return new DecoratorSet([{id:'preserve-owned-rust-const-function-keyword',level:'emitter',order:0,when:[['format','Rust']],actions:[{op:'replace',field:'source',from:policy.emission.from,to:policy.emission.to}]}]).toLino();}
async function generateNativeRules(){
 const [{parseJavaScript},{checkProgram},{emitRust}]=await Promise.all([import('../js/src/translation/javascript.js'),import('../js/src/translation/check.js'),import('../js/src/translation/emit-rust.js')]);
 const raw=emitRust(checkProgram(parseJavaScript(rustConstFunctionRules()))).definitions.join('\n\n'),style=new URL('../parity/self-translation/rust-const-function-rules-decorators.lino',import.meta.url);
 const expectedStyle=(()=>{
  const actions=[];let current=raw;const replace=(from,to)=>{if(!current.includes(from))throw new Error('const rule style target disappeared: '+from);actions.push({op:'replace',field:'source',from,to});current=current.split(from).join(to);};
  for(const parameter of['model','kind','operation','name'])replace(parameter+': String',parameter+': &str');
  for(const match of[...current.matchAll(/pub fn accept_rust_const_function_[\s\S]*?\n\}/gu)]){
   const definition=match[0],opening=definition.indexOf('{');let body=stripRustSourceHeaderStyleParentheses(definition.slice(opening+1,-1));
   body=body.replace(/\(([0-9]+f64)\)/gu,'$1').replace(/\(([a-z_]+\s*(?:==|!=|>=|<=|<|>)\s*(?:"[^"\n]*"|[0-9]+f64|[a-z_]+))\)/gu,'$1').replace('model == ""','model.is_empty()').replace('(value.floor())','value.floor()');
   if(definition.startsWith('pub fn accept_rust_const_function_name(')){
    const names=[...definition.matchAll(/name == ("[^"\\]*")/gu)].map(found=>JSON.parse(found[1]));
    if(JSON.stringify(names)!==JSON.stringify(rustConstFunctionPolicy.forbiddenNames))throw new Error('const name styling lost a policy keyword');
    body='!name.is_empty() && name != "_" && !matches!(name, '+names.map(name=>JSON.stringify(name)).join(' | ')+')';
   }
   if(definition.startsWith('pub fn accept_rust_const_function_name_character('))for(const[start,end]of[[95,95],[65,90],[97,122],[48,57]])body=body.replace('code >= '+start+'f64 && code <= '+end+'f64','('+start+'f64..='+end+'f64).contains(&code)');
   if(definition.startsWith('pub fn accept_rust_const_function_offset('))body=body.replace('value >= 0f64 && value <= 9007199254740991f64','(0f64..=9_007_199_254_740_991f64).contains(&value)');
   if(definition.startsWith('pub fn accept_rust_const_function_name_character('))body=body.replace('(!first && code >= 48f64) && code <= 57f64','!first && (48f64..=57f64).contains(&code)');
   const header=definition.slice(0,opening+1).replace(/pub fn (accept_rust_const_function_(?:interval|finite_literal))\(/u,'pub const fn $1(');
   replace(definition,'/// Apply a shared closed const predicate source decision.\n#[must_use]\n'+header+'\n    '+body+'\n}');
  }
  return new DecoratorSet([{id:'rust-const-function-library-style',level:'emitter',order:0,when:[['format','Rust']],actions}]).toLino();
 })();
 if(process.argv.includes('--update-styles'))writeFileSync(style,expectedStyle);
 else if(readFileSync(style,'utf8')!==expectedStyle)throw new Error('const predicate library styles are stale; regenerate the recorded styles');
 const generated='// Generated through the checked JavaScript translator. Do not edit.\n'+formatProductRuleSource(DecoratorSet.fromLino(readFileSync(style,'utf8')).decorate('emitter',{format:'Rust',source:raw}).source),native=new URL('../rust/src/translation/rust/const_function_rules.rs',import.meta.url),copy=new URL('../rust/src/data/rust-const-functions.lino',import.meta.url),data=new URL('../js/src/translation/data/rust-const-functions.lino',import.meta.url);
 if(process.argv.includes('--check')){if(readFileSync(native,'utf8')!==generated||!readFileSync(copy).equals(readFileSync(data)))throw new Error('native const predicate rules or policy are stale');}else{writeFileSync(native,generated);writeFileSync(copy,readFileSync(data));}
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const data=new URL('../js/src/translation/data/rust-const-functions.lino',import.meta.url),source=new URL('../js/src/translation/rust-const-function-rules.js',import.meta.url),style=new URL('../parity/self-translation/rust-const-function-emission-decorators.lino',import.meta.url);
 if(process.argv.includes('--check')){if(!checkProjectData(data,rustConstFunctionPolicy)||readFileSync(source,'utf8')!==rustConstFunctionRules()||readFileSync(style,'utf8')!==rustConstFunctionDecorators())throw new Error('const predicate draft policy, decisions or decorators are stale');}
 else{writeProjectData(data,rustConstFunctionPolicy);writeFileSync(source,rustConstFunctionRules());writeFileSync(style,rustConstFunctionDecorators());}
 if(!process.argv.includes('--javascript-only'))await generateNativeRules();
}
