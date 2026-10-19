// Shared empty-option decisions are authored once as linked policy data.
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData} from './project-data.mjs';
import {DecoratorSet} from '../src/decorators.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {formatProductRuleSource} from './product-rule-style-corrections.mjs';
const policyFile=new URL('../src/translation/data/rust-empty-options.lino',import.meta.url),policy=readProjectData(policyFile);
const source='// Generated from linked empty primitive Rust option policy. Do not edit.\n'
 +'/** @param {string} model @returns {boolean} */\nexport function acceptRustEmptyOptionSourceModel(model){return model === "" || model === '+JSON.stringify(policy.model)+';}\n'
 +['Result','Constructor'].map(kind=>'/** @param {string} path @param {boolean} shadowed @returns {boolean} */\nexport function acceptRustEmptyOption'+kind+'Owner(path,shadowed){return !shadowed && ('+policy[kind==='Result'?'resultPaths':'constructorPaths'].map(path=>'path === '+JSON.stringify(path)).join(' || ')+');}\n').join('')
 +'/** @param {string} kind @param {string} owner @param {boolean} indirect @returns {boolean} */\nexport function acceptRustEmptyOptionElement(kind,owner,indirect){return kind === "float" && owner === "f64" && !indirect;}\n'
 +'/** @param {string} language @param {boolean} forbidden @returns {boolean} */\nexport function acceptRustEmptyOptionContext(language,forbidden){return language === "Rust" && !forbidden;}\n'
 +'/** @param {string} kind @param {boolean} invalid @returns {boolean} */\nexport function acceptRustEmptyOptionFunction(kind,invalid){return kind === "fn" && !invalid;}\n'
 +'/** @param {string} kind @param {string} absence @returns {boolean} */\nexport function acceptRustEmptyOptionExpression(kind,absence){return kind === "none" && absence === "undefined";}\n';
const js=new URL('../src/translation/rust-empty-option-rules.js',import.meta.url),native=new URL('../../rust/src/translation/rust/empty_option_rules.rs',import.meta.url),copy=new URL('../../rust/src/data/rust-empty-options.lino',import.meta.url),style=new URL('../../parity/self-translation/rust-empty-option-rules-decorators.lino',import.meta.url);
if(process.argv.includes('--check')){if(readFileSync(js,'utf8')!==source)throw new Error('JS empty option rules are stale');}else writeFileSync(js,source);
if(!process.argv.includes('--javascript-only')){
 const raw=emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
 if(process.argv.includes('--update-styles')){
  const actions=[];let current=raw;const replace=(from,to)=>{if(current.includes(from)){actions.push({op:'replace',field:'source',from,to});current=current.split(from).join(to);}};
  for(const parameter of ['model','path','kind','owner','language','absence'])replace(parameter+': String',parameter+': &str');
  for(const match of raw.matchAll(/^pub fn (\w+)/gmu))replace('pub fn '+match[1],'/// Read a shared empty primitive option admission decision.\n#[must_use]\npub fn '+match[1]);
  replace('model == ""','model.is_empty()');
  for(const definition of current.matchAll(/pub fn accept_rust_[\s\S]*?\n\}/gu)){
   const block=definition[0],open=block.indexOf('{');let expression=block.slice(open+1,-1).trim();
   expression=expression.replace(/\((\w+ == "[^"]+"|!\w+|\w+\.is_empty\(\))\)/gu,'$1');
   if(expression.startsWith('(')&&expression.endsWith(')'))expression=expression.slice(1,-1);
   replace(block,block.slice(0,open+1)+'\n'+expression+'\n}');
  }
  writeFileSync(style,new DecoratorSet([{id:'rust-empty-option-library-style',level:'emitter',order:0,when:[['format','Rust']],actions}]).toLino());
 }
 const generated='// Generated through the checked JavaScript translator. Do not edit.\n'+formatProductRuleSource(DecoratorSet.fromLino(readFileSync(style,'utf8')).decorate('emitter',{format:'Rust',source:raw}).source);
 if(process.argv.includes('--check')){if(readFileSync(native,'utf8')!==generated||!readFileSync(copy).equals(readFileSync(policyFile)))throw new Error('native empty option rules or policy are stale');}else{writeFileSync(native,generated);writeFileSync(copy,readFileSync(policyFile));}
}
