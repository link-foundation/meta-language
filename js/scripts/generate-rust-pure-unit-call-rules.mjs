import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData} from './project-data.mjs';
import {DecoratorSet} from '../src/decorators.js';
import {formatProductRuleSource} from './product-rule-style-corrections.mjs';
import {stripRustSourceHeaderStyleParentheses} from './rust-source-header-library-style.mjs';
import assert from 'node:assert/strict';

const policyFile=new URL('../src/translation/data/rust-pure-unit-calls.lino',import.meta.url),policy=readProjectData(policyFile);
const source='// Generated from linked pure Rust Unit-call policy. Do not edit.\n'
 +'/** @param {string} model @returns {boolean} */\nexport function acceptRustPureUnitCallModel(model){return model === "" || model === '+JSON.stringify(policy.model)+' || model === '+JSON.stringify(policy.publicModel)+';}\n'
 +'/** @param {number} code @param {boolean} first @returns {boolean} */\nexport function acceptRustPureUnitNameCharacter(code,first){return code >= 95 && code <= 95 || code >= 65 && code <= 90 || code >= 97 && code <= 122 || !first && code >= 48 && code <= 57;}\n'
 +'/** @param {string} name @returns {boolean} */\nexport function acceptRustPureUnitName(name){return name !== "" && name !== "_" && !name.startsWith("ml_") && !('+policy.forbiddenNames.map(name=>'name === '+JSON.stringify(name)).join(' || ')+');}\n'
 +'/** @param {string} form @returns {boolean} */\nexport function acceptRustPureUnitHeaderForm(form){return '+policy.headerForms.map(form=>'form === '+JSON.stringify(form)).join(' || ')+';}\n'
 +'/** @param {string} form @returns {boolean} */\nexport function acceptRustPureUnitBodyForm(form){return '+policy.bodyForms.map(form=>'form === '+JSON.stringify(form)).join(' || ')+';}\n'
 +'/** @param {string} form @returns {boolean} */\nexport function acceptRustPureUnitPathForm(form){return '+policy.pathForms.map(form=>'form === '+JSON.stringify(form)).join(' || ')+';}\n'
 +'/** @param {string} language @param {boolean} forbidden @returns {boolean} */\nexport function acceptRustPureUnitContext(language,forbidden){return language === "Rust" && !forbidden;}\n'
 +'/** @param {string} model @param {string} form @returns {boolean} */\nexport function acceptRustPureUnitVisibility(model,form){return model === '+JSON.stringify(policy.model)+' && form === '+JSON.stringify(policy.visibilityForms.private)+' || model === '+JSON.stringify(policy.publicModel)+' && form === '+JSON.stringify(policy.visibilityForms.public)+';}\n'
 +'/** @param {string} kind @param {boolean} dense @param {boolean} unrepresented @returns {boolean} */\nexport function acceptRustPureUnitContainer(kind,dense,unrepresented){return !unrepresented && (kind === '+JSON.stringify(policy.containerKinds.array)+' && dense || kind === '+JSON.stringify(policy.containerKinds.map)+');}\n';
const js=new URL('../src/translation/rust-pure-unit-call-rules.js',import.meta.url),native=new URL('../../rust/src/translation/rust/pure_unit_call_rules.rs',import.meta.url),copy=new URL('../../rust/src/data/rust-pure-unit-calls.lino',import.meta.url),style=new URL('../../parity/self-translation/rust-pure-unit-call-rules-decorators.lino',import.meta.url);
if(process.argv.includes('--check')){if(readFileSync(js,'utf8')!==source)throw Error('JavaScript pure Unit-call rules are stale');}else writeFileSync(js,source);
if(!process.argv.includes('--javascript-only')){
 const {parseJavaScript}=await import('../src/translation/javascript.js');
 const {checkProgram}=await import('../src/translation/check.js');
 const {emitRust}=await import('../src/translation/emit-rust.js');
 const raw=emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
 if(process.argv.includes('--update-styles')){
  const actions=[];let current=raw;const replace=(from,to)=>{if(current.includes(from)){actions.push({op:'replace',field:'source',from,to});current=current.split(from).join(to);}};
  for(const parameter of ['model','name','form','language','kind'])replace(parameter+': String',parameter+': &str');
  for(const match of raw.matchAll(/^pub fn (\w+)\(/gmu))replace('pub fn '+match[1]+'(','/// Read a shared pure Rust Unit-call admission.\n#[must_use]\npub fn '+match[1]+'(');
  replace('model == ""','model.is_empty()');replace('name != ""','!name.is_empty()');
  for(const definition of current.matchAll(/pub fn accept_rust_pure_unit_[\s\S]*?\n\}/gu)){
   const block=definition[0],open=block.indexOf('{');let expression=stripRustSourceHeaderStyleParentheses(block.slice(open+1,-1));
   expression=expression.replace(/\(([0-9]+f64)\)/gu,'$1').replace(/\((\w+\s*(?:==|!=|>=|<=)\s*(?:"[^"]*"|[0-9]+f64))\)/gu,'$1');
   expression=expression.replace(/\((!?\w+\.(?:is_empty|starts_with)\((?:"[^"]*")?\))\)/gu,'$1');
   for(const [from,to] of [[95,95],[65,90],[97,122],[48,57]])expression=expression.replaceAll('code >= '+from+'f64 && code <= '+to+'f64','('+from+'f64..='+to+'f64).contains(&code)');
   expression=expression.replace('(!first && code >= 48f64) && code <= 57f64','!first && (48f64..=57f64).contains(&code)');
   // A flat pattern replaces only the generated pure equality disjunction.
   // Validate every literal against the authored policy before recording it.
   if(block.startsWith('pub fn accept_rust_pure_unit_name(')){
    const excluded=[...block.matchAll(/name == ("[^"\\]*")/gu)].map(match=>JSON.parse(match[1]));assert.deepEqual(excluded,policy.forbiddenNames);
    expression='!name.is_empty() && name != "_" && !name.starts_with("ml_") && !matches!(name, '+excluded.map(value=>JSON.stringify(value)).join(' | ')+')';
   }
   expression=stripRustSourceHeaderStyleParentheses(expression);
   replace(block,block.slice(0,open+1)+'\n    '+expression+'\n}');
  }
  writeFileSync(style,new DecoratorSet([{id:'rust-pure-unit-call-library-style',level:'emitter',order:0,when:[['format','Rust']],actions}]).toLino());
 }
 const generated='// Generated through the checked JavaScript translator. Do not edit.\n'+formatProductRuleSource(DecoratorSet.fromLino(readFileSync(style,'utf8')).decorate('emitter',{format:'Rust',source:raw}).source);
 if(process.argv.includes('--check')){if(readFileSync(native,'utf8')!==generated||!readFileSync(copy).equals(readFileSync(policyFile)))throw Error('Native pure Unit-call rules or policy are stale');}else{writeFileSync(native,generated);writeFileSync(copy,readFileSync(policyFile));}
}
