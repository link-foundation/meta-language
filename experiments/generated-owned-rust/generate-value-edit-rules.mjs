// An additive source interpretation; the original model and rules stay unchanged.
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {writeProjectData,checkProjectData} from '../../js/scripts/project-data.mjs';
import {DecoratorSet} from '../../js/src/decorators.js';
import {formatProductRuleSource} from '../../js/scripts/product-rule-style-corrections.mjs';
import {stripRustSourceHeaderStyleParentheses} from '../../js/scripts/rust-source-header-library-style.mjs';
import {generatedOwnedRustPolicy} from './generate-rules.mjs';

export const generatedOwnedRustValueEditPolicy={
 version:1,model:'closed generated owned Rust literal value edits 1',baseModel:generatedOwnedRustPolicy.model,
 string:{kind:'string',operation:'owned standard String literal construction',domain:'Unicode scalar String',reader:'actual complete production Rust cooked string token',quoterModule:'js/src/translation/rust-string-literals.js',quoter:'rustString'},
 integer:{kind:'fixed',operation:'owned signed i32 literal',bits:32,signed:true,suffix:'i32',minimum:'-2147483648',maximum:'2147483647',removedSignReplacement:' ',scope:'An original unary minus belongs only to its actual checked i32 literal operation. Its complete signed value may change within i32; removing the minus inserts one ASCII separator and retains every original comment and trivia interval. A new minus is inserted only inside that actual literal emission. The complete emitted module is reinterpreted before its conditions are rebound.'},
 proofField:'literalEdits',fields:['operationNode','literal','signToken'],
 boundaries:['All original declarations, types, lexical owners, clone owners, runtime operations and explicit success conditions remain required.','The original model, literal-domain predicate, policy and fixtures are unchanged. This is an independently selected experimental source interpretation.','String values contain only Unicode scalar text. This does not compose with the generic JavaScript UTF16 code-unit model.','Unchanged literals retain their original valid spelling; edited String literals use the actual shared Rust emitter quoter.','Every String and i32 literal edit record references its actual original source operation, checked literal owner and optional actual unary-minus token.','The newly emitted complete source must pass the selected model and reconstruct all operation-bound conditions at its new spans. No Rust native support, production selector or ledger evidence is claimed.'],
};
export function generatedOwnedRustValueEditRules(){const p=generatedOwnedRustValueEditPolicy;return '// Generated from the additive linked owned Rust literal-edit policy. Do not edit.\n'
 +'/** @param {string} model @returns {boolean} */\nexport function acceptGeneratedOwnedRustValueEditModel(model){return model === '+JSON.stringify(p.model)+';}\n'
 +'/** @param {string} kind @param {string} operation @returns {boolean} */\nexport function acceptGeneratedOwnedRustLiteralEditOwner(kind,operation){return kind === '+JSON.stringify(p.string.kind)+' && operation === '+JSON.stringify(p.string.operation)+' || kind === '+JSON.stringify(p.integer.kind)+' && operation === '+JSON.stringify(p.integer.operation)+';}\n'
 +'/** @param {boolean} negative @param {boolean} owned @returns {boolean} */\nexport function acceptGeneratedOwnedRustSignOwner(negative,owned){return negative === owned;}\n';}

async function translatedRules(check){
 const [{parseJavaScript},{checkProgram},{emitRust}]=await Promise.all([import('../../js/src/translation/javascript.js'),import('../../js/src/translation/check.js'),import('../../js/src/translation/emit-rust.js')]);
 const raw=emitRust(checkProgram(parseJavaScript(generatedOwnedRustValueEditRules()))).definitions.join('\n\n'),actions=[];let current=raw;
 const replace=(from,to)=>{if(!current.includes(from))throw new Error('literal-edit style target disappeared');actions.push({op:'replace',field:'source',from,to});current=current.split(from).join(to);};
 for(const name of['model','kind','operation'])replace(name+': String',name+': &str');
 for(const match of[...current.matchAll(/pub fn accept_generated_owned_rust_[\s\S]*?\n\}/gu)]){const definition=match[0],at=definition.indexOf('{'),body=stripRustSourceHeaderStyleParentheses(definition.slice(at+1,-1));replace(definition,'/// Experimental generated owned literal-edit decision.\n#[must_use]\n'+definition.slice(0,at+1)+'\n    '+body+'\n}');}
 const style=new DecoratorSet([{id:'generated-owned-rust-literal-edit-library-style',level:'emitter',order:0,when:[['format','Rust']],actions}]).toLino();
 const output='// Experimental automatic translation; not a native runtime integration.\n'+formatProductRuleSource(DecoratorSet.fromLino(style).decorate('emitter',{format:'Rust',source:raw}).source);
 for(const[file,text]of[[new URL('./data/value-edit-rules.rs',import.meta.url),output],[new URL('./data/value-edit-library-styles.lino',import.meta.url),style]]){if(check){if(readFileSync(file,'utf8')!==text)throw new Error('literal-edit translation or styles are stale');}else writeFileSync(file,text);}
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const check=process.argv.includes('--check'),file=new URL('./data/value-edit-policy.lino',import.meta.url),rules=new URL('./value-edit-rules.mjs',import.meta.url),source=generatedOwnedRustValueEditRules();
 if(check){checkProjectData(file,generatedOwnedRustValueEditPolicy);if(readFileSync(rules,'utf8')!==source)throw new Error('literal-edit rules are stale');}else{writeProjectData(file,generatedOwnedRustValueEditPolicy);writeFileSync(rules,source);}
 await translatedRules(check);
}
