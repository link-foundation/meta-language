// Private finite source interpretation. Shared decisions are generated from links.
import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {writeProjectData,checkProjectData} from '../../js/scripts/project-data.mjs';
import {DecoratorSet} from '../../js/src/decorators.js';
import {formatProductRuleSource} from '../../js/scripts/product-rule-style-corrections.mjs';
import {stripRustSourceHeaderStyleParentheses} from '../../js/scripts/rust-source-header-library-style.mjs';
export const rustTextValueProjectionPolicy={
 version:1,model:'closed standard Rust text value projection 1',language:'Rust',signatureModel:'Rust source function signature 1',
 observationModel:'closed Unicode scalar text value and successful allocation observation 1',literalTokenMode:'production Rust string token',
 domains:{owned:'owned standard String',reference:'shared standard String',slice:'shared primitive str',literal:'static primitive str literal'},
 operations:[
  {name:'owned standard String move',inputs:['owned standard String'],allocation:false},
  {name:'standard String construction from borrowed text',inputs:['shared primitive str','static primitive str literal'],allocation:true},
  {name:'standard String clone',inputs:['owned standard String','shared standard String'],allocation:true},
  {name:'standard borrowed str owned construction',inputs:['shared primitive str','static primitive str literal'],allocation:true},
  {name:'standard borrowed str display construction',inputs:['shared primitive str','static primitive str literal'],allocation:true},
 ],
 maximumParameters:2,reservedNames:['String','str','std','core','alloc','main','ml_main','ml_ownedTextInput'],
 fields:{proof:['model','language','observationModel','span','tokens','gaps','operations','functions','worker','inputAssumptions','valueAssumptions'],function:['declaration','signature','resultPosition','operationNode','coreValue','input','position','literal'],assumption:['id','meaning','declaration','operationNode','coreValue','input','position']},
 inputAssumption:{id:'rust-text-projection-input-value-domain',meaning:'This actual referenced input position contains primitive Unicode scalar text. Only its text value is observed; pointer identity, aliasing, ABI, borrowing duration and destructor timing are outside this observation.'},
 valueAssumption:{id:'rust-text-projection-string-operation-success',meaning:'This actual standard owned String construction or clone succeeds and preserves its Unicode scalar text. Allocation failure and counts, pointer identity, aliasing and destruction timing are outside this value observation.'},
 boundaries:['Flat optionally public functions have zero to two distinct direct String, immutable &String or immutable &str parameters and an actual unborrowed standard String result. All reference lifetimes are elided.','Each body is one admitted actual text operation. There are no imports, modules, type declarations, traits, generics, explicit lifetimes, mutable or extra reference layers, arbitrary calls, effects or borrowed results. The independent main is empty.','The actual production Rust parser and source-signature owner checker interpret the complete tokens; the separate finite source-operation grammar verifies the actual operation and source leaf type before erasure.','A String move creates no clone or allocation premise. Shared String clone dispatch is limited to the direct String and one shared reference receiver candidates. Shared str or literal clone cannot produce owned String.','Source-free proof references are actual core declaration, signature, parameter, result and expression identities. Complete tokens and ordinary trivia remain inspectable through text and binary Links.','The generic Rust emitter runtime wrapper remains outside this interpretation. This experiment claims no native support, production selector, complete borrow semantics or ledger evidence.'],
};
export function rustTextValueProjectionRules(){const p=rustTextValueProjectionPolicy;return '// Generated from the private linked Rust text-value projection policy. Do not edit.\n'
 +'/** @param {string} model @returns {boolean} */\nexport function acceptRustTextValueProjectionModel(model){return model === '+JSON.stringify(p.model)+';}\n'
 +'/** @param {boolean} standard @param {boolean} slice @param {number} layers @returns {string} */\nexport function readRustTextValueProjectionDomain(standard,slice,layers){return standard && layers === 0 ? '+JSON.stringify(p.domains.owned)+' : standard && layers === 1 ? '+JSON.stringify(p.domains.reference)+' : slice && layers === 1 ? '+JSON.stringify(p.domains.slice)+' : "";}\n'
 +'/** @param {string} operation @param {string} domain @returns {boolean} */\nexport function acceptRustTextValueProjectionOperation(operation,domain){return '+p.operations.map(row=>'(operation === '+JSON.stringify(row.name)+' && ('+row.inputs.map(domain=>'domain === '+JSON.stringify(domain)).join(' || ')+'))').join(' || ')+';}\n';}
async function nativeRules(check){
 const [{parseJavaScript},{checkProgram},{emitRust}]=await Promise.all([import('../../js/src/translation/javascript.js'),import('../../js/src/translation/check.js'),import('../../js/src/translation/emit-rust.js')]);
 const raw=emitRust(checkProgram(parseJavaScript(rustTextValueProjectionRules()))).definitions.join('\n\n'),actions=[];let current=raw;
 const replace=(from,to)=>{if(!current.includes(from))throw new Error('text projection style target disappeared');actions.push({op:'replace',field:'source',from,to});current=current.split(from).join(to);};
 for(const name of['model','operation','domain'])replace(name+': String',name+': &str');
 for(const match of[...current.matchAll(/pub fn (?:accept|read)_rust_text_value_projection_[\s\S]*?\n\}/gu)]){const definition=match[0],at=definition.indexOf('{'),body=stripRustSourceHeaderStyleParentheses(definition.slice(at+1,-1));replace(definition,'/// Experimental source text-value decision.\n#[must_use]\n'+definition.slice(0,at+1)+'\n    '+body+'\n}');}
 const styles=new DecoratorSet([{id:'rust-text-value-projection-library-style',level:'emitter',order:0,when:[['format','Rust']],actions}]).toLino();
 const output='// Experimental automatic translation; no native runtime integration.\n'+formatProductRuleSource(DecoratorSet.fromLino(styles).decorate('emitter',{format:'Rust',source:raw}).source);
 for(const[file,text]of[[new URL('./data/rules.rs',import.meta.url),output],[new URL('./data/library-styles.lino',import.meta.url),styles]]){if(check){if(readFileSync(file,'utf8')!==text)throw new Error('text projection translation or styles are stale');}else writeFileSync(file,text);}
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 mkdirSync(new URL('./data/',import.meta.url),{recursive:true});const check=process.argv.includes('--check'),file=new URL('./data/policy.lino',import.meta.url),rules=new URL('./rules.mjs',import.meta.url),source=rustTextValueProjectionRules();
 if(check){checkProjectData(file,rustTextValueProjectionPolicy);if(readFileSync(rules,'utf8')!==source)throw new Error('text projection rules are stale');}else{writeProjectData(file,rustTextValueProjectionPolicy);writeFileSync(rules,source);}
 await nativeRules(check);
}
