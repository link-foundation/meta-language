// JavaScript-first decisions for a bounded source const-qualification model.
import {fileURLToPath} from 'node:url';
import {readFileSync,writeFileSync} from 'node:fs';
import {writeProjectData,checkProjectData} from '../js/scripts/project-data.mjs';
import {DecoratorSet} from '../js/src/decorators.js';
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
 +'/** @param {number} value @param {boolean} negativeZero @returns {boolean} */\nexport function acceptRustConstFunctionOffset(value,negativeZero){return value >= 0 && value <= 9007199254740991 && Math.floor(value) === value && !negativeZero;}\n'
 +'/** @param {number} start @param {number} end @param {boolean} nonempty @returns {boolean} */\nexport function acceptRustConstFunctionInterval(start,end,nonempty){return start <= end && (!nonempty || start < end);}\n'
 +'/** @param {number} code @param {boolean} first @returns {boolean} */\nexport function acceptRustConstFunctionNameCharacter(code,first){return code === 95 || code >= 65 && code <= 90 || code >= 97 && code <= 122 || !first && code >= 48 && code <= 57;}\n'
 +'/** @param {string} name @returns {boolean} */\nexport function acceptRustConstFunctionName(name){return name !== "" && name !== "_" && !('+either('name',policy.forbiddenNames)+');}\n'
 +'/** @param {number} value @returns {boolean} */\nexport function acceptRustConstFunctionFiniteLiteral(value){return Number.isFinite(value);}\n';
}
export function rustConstFunctionDecorators(policy=rustConstFunctionPolicy){return new DecoratorSet([{id:'preserve-owned-rust-const-function-keyword',level:'emitter',order:0,when:[['format','Rust']],actions:[{op:'replace',field:'source',from:policy.emission.from,to:policy.emission.to}]}]).toLino();}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const data=new URL('../js/src/translation/data/rust-const-functions.lino',import.meta.url),source=new URL('../js/src/translation/rust-const-function-rules.js',import.meta.url),style=new URL('../parity/self-translation/rust-const-function-emission-decorators.lino',import.meta.url);
 if(process.argv.includes('--check')){if(!checkProjectData(data,rustConstFunctionPolicy)||readFileSync(source,'utf8')!==rustConstFunctionRules()||readFileSync(style,'utf8')!==rustConstFunctionDecorators())throw new Error('const predicate draft policy, decisions or decorators are stale');}
 else{writeProjectData(data,rustConstFunctionPolicy);writeFileSync(source,rustConstFunctionRules());writeFileSync(style,rustConstFunctionDecorators());}
}
