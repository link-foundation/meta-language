import {readFileSync,writeFileSync} from 'node:fs';
import {DecoratorSet} from '../src/decorators.js';
import {formatProductRuleSource} from './product-rule-style-corrections.mjs';
import {stripRustSourceHeaderStyleParentheses} from './rust-source-header-library-style.mjs';
import {readProjectData} from './project-data.mjs';
const policy=readProjectData(new URL('../src/translation/data/rust-runtime-shell.lino',import.meta.url));
const source='// Generated from linked closed Rust runtime-shell decisions. Do not edit.\n'
 +'/** @param {string} model @returns {boolean} */\nexport function acceptRustRuntimeShellModel(model){return model === "" || model === '+JSON.stringify(policy.model)+';}\n'
 +'/** @param {string} name @returns {boolean} */\nexport function acceptRustRuntimeShellReservedName(name){return !('+policy.reservedNames.map(name=>'name === '+JSON.stringify(name)).join(' || ')+');}\n'
 +'/** @param {number} value @returns {boolean} */\nexport function acceptRustRuntimeShellStackShift(value){return value >= '+policy.semantics.thread.stackShiftMinimum+' && value <= '+policy.semantics.thread.stackShiftMaximum+' && value % 1 === 0;}\n';
const file=new URL('../src/translation/rust-runtime-shell-rules.js',import.meta.url);
if(process.argv.includes('--check')){if(readFileSync(file,'utf8')!==source)throw Error('closed runtime-shell decisions are stale');}else writeFileSync(file,source);

if(!process.argv.includes('--javascript-only')){
 const {parseJavaScript}=await import('../src/translation/javascript.js');
 const {checkProgram}=await import('../src/translation/check.js');
 const {emitRust}=await import('../src/translation/emit-rust.js');
 const raw=emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
 const style=new URL('../../parity/self-translation/rust-runtime-shell-rules-decorators.lino',import.meta.url);
 if(process.argv.includes('--update-styles')){
  const actions=[];let current=raw;const replace=(from,to)=>{if(current.includes(from)){actions.push({op:'replace',field:'source',from,to});current=current.split(from).join(to);}};
  for(const parameter of ['model','name'])replace(parameter+': String',parameter+': &str');
  for(const match of raw.matchAll(/^pub fn (\w+)\(/gmu))replace('pub fn '+match[1]+'(','/// Read the shared closed runtime-shell admission.\n#[must_use]\npub fn '+match[1]+'(');
  replace('model == ""','model.is_empty()');
  for(const definition of current.matchAll(/pub fn accept_rust_runtime_shell_[\s\S]*?\n\}/gu)){
   const block=definition[0],open=block.indexOf('{');let expression=stripRustSourceHeaderStyleParentheses(block.slice(open+1,-1));
   expression=expression.replace(/\(([0-9]+f64)\)/gu,'$1').replace(/\((\w+\s*(?:==|!=|>=|<=)\s*(?:"[^"]*"|[0-9]+f64))\)/gu,'$1');
   expression=expression.replace(/\((!?\w+\.is_empty\(\))\)/gu,'$1');
   expression=expression.replace('value >= '+policy.semantics.thread.stackShiftMinimum+'f64 && value <= '+policy.semantics.thread.stackShiftMaximum+'f64','('+policy.semantics.thread.stackShiftMinimum+'f64..='+policy.semantics.thread.stackShiftMaximum+'f64).contains(&value)');
   if(block.startsWith('pub fn accept_rust_runtime_shell_reserved_name('))expression='!matches!(name, '+policy.reservedNames.map(name=>JSON.stringify(name)).join(' | ')+')';
   if(block.startsWith('pub fn accept_rust_runtime_shell_stack_shift('))expression='('+policy.semantics.thread.stackShiftMinimum+'f64..='+policy.semantics.thread.stackShiftMaximum+'f64).contains(&value) && value % 1f64 == 0f64';
   replace(block,block.slice(0,open+1)+'\n    '+stripRustSourceHeaderStyleParentheses(expression)+'\n}');
  }
  writeFileSync(style,new DecoratorSet([{id:'rust-runtime-shell-library-style',level:'emitter',order:0,when:[['format','Rust']],actions}]).toLino());
 }
 const generated='// Generated through the checked JavaScript translator. Do not edit.\n'+formatProductRuleSource(DecoratorSet.fromLino(readFileSync(style,'utf8')).decorate('emitter',{format:'Rust',source:raw}).source);
 const native=new URL('../../rust/src/translation/runtime_shell_rules.rs',import.meta.url),copy=new URL('../../rust/src/data/rust-runtime-shell.lino',import.meta.url),original=new URL('../src/translation/data/rust-runtime-shell.lino',import.meta.url);
 if(process.argv.includes('--check')){if(readFileSync(native,'utf8')!==generated||!readFileSync(copy).equals(readFileSync(original)))throw Error('native runtime-shell rules or linked policy are stale');}else{writeFileSync(native,generated);writeFileSync(copy,readFileSync(original));}
}
