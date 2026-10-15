// Generate both packages from the reviewed JavaScript-first linked decisions.
import {readFileSync,writeFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {readProjectData} from '../js/scripts/project-data.mjs';
import {DecoratorSet} from '../js/src/decorators.js';
import {formatProductRuleSource} from '../js/scripts/product-rule-style-corrections.mjs';
import {stripRustSourceHeaderStyleParentheses} from '../js/scripts/rust-source-header-library-style.mjs';
export const ownedTupleCallPolicy=readProjectData(new URL('../js/src/translation/data/rust-owned-tuple-calls.lino',import.meta.url));
export function rustOwnedTupleCallRules(){return '// Generated from linked owned tuple call policy. Do not edit.\n'
 +'/** @param {string} model @returns {boolean} */\n'
 +'export function acceptRustOwnedTupleCallModel(model){return model === '+JSON.stringify(ownedTupleCallPolicy.model)+';}\n'
 +'/** @param {boolean} target @param {boolean} argumentsMatch @param {boolean} fieldsMatch @returns {boolean} */\n'
 +'export function acceptRustOwnedTupleCallInitializer(target,argumentsMatch,fieldsMatch){return target&&argumentsMatch&&fieldsMatch;}\n';}
async function generateNativeRules(){
 const [{parseJavaScript},{checkProgram},{emitRust}]=await Promise.all([import('../js/src/translation/javascript.js'),import('../js/src/translation/check.js'),import('../js/src/translation/emit-rust.js')]);
 const raw=emitRust(checkProgram(parseJavaScript(rustOwnedTupleCallRules()))).definitions.join('\n\n');
 const style=new URL('../parity/self-translation/rust-owned-tuple-call-rules-decorators.lino',import.meta.url);
 const actions=[];let current=raw;
 const replace=(from,to)=>{if(!current.includes(from))throw new Error('owned tuple call style target disappeared: '+from);actions.push({op:'replace',field:'source',from,to});current=current.split(from).join(to);};
 replace('model: String','model: &str');
 for(const match of [...current.matchAll(/pub fn accept_rust_owned_tuple_call_[\s\S]*?\n\}/gu)]){
  const definition=match[0],opening=definition.indexOf('{');
  const body=stripRustSourceHeaderStyleParentheses(definition.slice(opening+1,-1));
  const header=definition.slice(0,opening+1).replace('pub fn accept_rust_owned_tuple_call_initializer(', 'pub const fn accept_rust_owned_tuple_call_initializer(');
  replace(definition,'/// Apply a shared owned tuple initialization call decision.\n#[must_use]\n'+header+'\n    '+body+'\n}');
 }
 const expectedStyle=new DecoratorSet([{id:'rust-owned-tuple-call-library-style',level:'emitter',order:0,when:[['format','Rust']],actions}]).toLino();
 if(process.argv.includes('--update-styles'))writeFileSync(style,expectedStyle);
 else if(readFileSync(style,'utf8')!==expectedStyle)throw new Error('owned tuple call library styles are stale');
 const generated='// Generated through the checked JavaScript translator. Do not edit.\n'+formatProductRuleSource(DecoratorSet.fromLino(readFileSync(style,'utf8')).decorate('emitter',{format:'Rust',source:raw}).source);
 const native=new URL('../rust/src/translation/rust/owned_tuple_call_rules.rs',import.meta.url),copy=new URL('../rust/src/data/rust-owned-tuple-calls.lino',import.meta.url),policy=new URL('../js/src/translation/data/rust-owned-tuple-calls.lino',import.meta.url);
 if(process.argv.includes('--check')){if(readFileSync(native,'utf8')!==generated||!readFileSync(copy).equals(readFileSync(policy)))throw new Error('native owned tuple call rules or policy are stale');}
 else{writeFileSync(native,generated);writeFileSync(copy,readFileSync(policy));}
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const file=new URL('../js/src/translation/rust-owned-tuple-call-rules.js',import.meta.url),source=rustOwnedTupleCallRules();
 if(process.argv.includes('--check')){if(readFileSync(file,'utf8')!==source)throw new Error('owned tuple call rules are stale');}else writeFileSync(file,source);
 if(!process.argv.includes('--javascript-only'))await generateNativeRules();
}
