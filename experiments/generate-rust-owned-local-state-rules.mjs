// Generate the shared owned text state decisions and their native library styles.
import {fileURLToPath} from 'node:url';
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData} from '../js/scripts/project-data.mjs';
import {DecoratorSet} from '../js/src/decorators.js';
import {parseJavaScript} from '../js/src/translation/javascript.js';
import {checkProgram} from '../js/src/translation/check.js';
import {emitRust} from '../js/src/translation/emit-rust.js';
import {formatProductRuleSource} from '../js/scripts/product-rule-style-corrections.mjs';
const policy=readProjectData(new URL('../js/src/translation/data/rust-owned-local-state.lino',import.meta.url));
export function rustOwnedLocalStatePortableRules(){
 const ranges=policy.textCodeUnits;
 const textInput=`function acceptRustOwnedLocalTextInput(text){
 if(typeof text !== "string")return false;
 for(let index=0;index<text.length;index++){
  const unit=text.charCodeAt(index);
  if(unit>=${ranges.highStart}&&unit<=${ranges.highEnd}){
   index++;
   const next=text.charCodeAt(index);
   if(!(next>=${ranges.lowStart}&&next<=${ranges.lowEnd}))return false;
  }else if(unit>=${ranges.lowStart}&&unit<=${ranges.lowEnd})return false;
 }
 return true;
}`;
 return '// Generated from linked owned local state policy. Do not edit.\n'
 + '/** @param {string} model @returns {boolean} */\nexport function acceptRustOwnedLocalStateModel(model){return model === "" || model === '+JSON.stringify(policy.models.moves)+' || model === '+JSON.stringify(policy.models.copies)+';}\n'
 + '/** @param {string} kind @param {string} bits @param {boolean} signed @returns {boolean} */\nexport function acceptRustOwnedLocalValueType(kind,bits,signed){return kind === "string" || kind === "bool" || (kind === "fixed" && bits === "32" && signed);}\n'
 + '/** @param {string} kind @returns {boolean} */\nexport function acceptRustOwnedLocalExpression(kind){return '+Object.keys(policy.expressions).map(kind=>'kind === '+JSON.stringify(kind)).join(' || ')+';}\n'
 + '/** @param {string} operation @returns {boolean} */\nexport function acceptRustOwnedLocalBinaryOperation(operation){return operation === "eq";}\n'
 + '/** @param {Array<number>} units @returns {boolean} */\nexport function acceptRustOwnedLocalTextUnits(units){\n for(let index=0;index<units.length;index++){\n  const unit=units[index]??-1;\n  if(unit>='+ranges.highStart+'&&unit<='+ranges.highEnd+'){\n   index++;\n   const next=units[index]??-1;\n   if(!(next>='+ranges.lowStart+'&&next<='+ranges.lowEnd+'))return false;\n  }else if(unit>='+ranges.lowStart+'&&unit<='+ranges.lowEnd+')return false;\n }\n return true;\n}\n'
 + '/** @returns {string} */\nexport function renderRustOwnedLocalTextInput(){return '+JSON.stringify(textInput.replace('acceptRustOwnedLocalTextInput','ml_ownedTextInput'))+';}\n';
}
export function rustOwnedLocalStateRules(){
 return rustOwnedLocalStatePortableRules()
 + '/** Convert only actual primitive strings through the selected standard realm. */\n'
 + '/** @param {string} text @returns {boolean} */\n'
 + 'export function acceptRustOwnedLocalTextInput(text){\n'
 + ' if(typeof text !== "string")return false;\n'
 + ' const units=[];\n'
 + ' for(let index=0;index<text.length;index++)units.push(text.charCodeAt(index));\n'
 + ' return acceptRustOwnedLocalTextUnits(units);\n}\n';
}
function generateNativeOwnedLocalStateRules(){
 const policyFile=new URL('../js/src/translation/data/rust-owned-local-state.lino',import.meta.url);
 const emitted=emitRust(checkProgram(parseJavaScript(rustOwnedLocalStatePortableRules()))),raw=emitted.definitions.join('\n\n');
 const style=new URL('../parity/self-translation/rust-owned-local-state-rules-decorators.lino',import.meta.url);
 if(process.argv.includes('--update-styles')){
  const actions=[];let current=raw;const replace=(from,to)=>{if(current.includes(from)){actions.push({op:'replace',field:'source',from,to});current=current.split(from).join(to);}};
  for(const name of ['model','kind','bits','operation'])replace(name+': String',name+': &str');
  replace('mut units: Vec<f64>','units: &[f64]');replace('units: Vec<f64>','units: &[f64]');
  replace('units.clone()', 'units');replace('&units,','units,');replace('match ml_s1.clone()', 'match ml_s1');
  replace('crate::ml_array::','crate::translation::frontend_rules::ml_array::');
  for(const match of [...current.matchAll(/crate::(Ml\w+|ml_\w+)/gu)])replace(match[0],'crate::translation::rust::owned_local_state_rules::'+match[1]);
  replace('((units.len() as f64))','serde_json::Number::from(units.len()).as_f64().expect("a UTF16 unit count has a finite Number length")');
  replace('.clone().unwrap_or_else(|| (-1f64))','.unwrap_or(-1f64)');
  for(const[variable,start,end]of[['unit',policy.textCodeUnits.highStart,policy.textCodeUnits.highEnd],['unit',policy.textCodeUnits.lowStart,policy.textCodeUnits.lowEnd],['next',policy.textCodeUnits.lowStart,policy.textCodeUnits.lowEnd]]){
   const integer=value=>String(value).replace(/\B(?=(\d{3})+(?!\d))/gu,'_');
   replace('(('+variable+' >= ('+start+'f64)) && ('+variable+' <= ('+end+'f64)))','('+integer(start)+'f64..='+integer(end)+'f64).contains(&'+variable+')');
  }
  replace('model == ""','model.is_empty()');
  while(/\} else \{\n\s+if /u.test(current)){
   const start=current.match(/\} else \{\n\s+if /u).index,open=start+7;let depth=0,end=open;
   for(;end<current.length;end++){if(current[end]==='{')depth++;else if(current[end]==='}'&&--depth===0)break;}
   const from=current.slice(start,end+1);replace(from,from.replace(/\} else \{\n\s+if /u,'} else if ').slice(0,-1));
  }
  for(const match of [...current.matchAll(/\(([a-z_]+ == "[^"\n]*"|![_a-z]+|[_a-z]+\.is_empty\(\))\)/gu)])replace(match[0],match[1]);
  // These three complete classifier bodies contain only the recorded Boolean
  // comparisons. Removing their grouping keeps && tighter than || and each
  // left-to-right short-circuit decision unchanged.
  for(const name of ['state_model','value_type','expression']){
   const match=current.match(new RegExp('pub fn accept_rust_owned_local_'+name+'\\([^]*?\\) -> bool \\{\\n([^]*?)\\n\\}', 'u'));
   if(match)replace(match[1],match[1].replace('model.is_empty()','model == \"\"').replace(/[()]/gu,'').replace('model == \"\"','model.is_empty()'));
  }
  replace('if (index < serde_json::Number::from(units.len()).as_f64().expect("a UTF16 unit count has a finite Number length"))', 'if index < serde_json::Number::from(units.len()).as_f64().expect("a UTF16 unit count has a finite Number length")');
  replace('if !((56_320f64..=57_343f64).contains(&next))','if !(56_320f64..=57_343f64).contains(&next)');
  for(const match of [...current.matchAll(/let (index_\d+) = \((index(?:_\d+)? \+ 1f64)\);/gu)])replace(match[0],'let '+match[1]+' = '+match[2]+';');
  for(const match of [...current.matchAll(/\((crate::translation::frontend_rules::ml_array::optional_at\([^\n]*\))\)\.unwrap_or/gu)])replace(match[0],match[1]+'.unwrap_or');
  for(const match of [...current.matchAll(/^pub fn (\w+)/gmu)])replace(match[0],'/// Read a shared owned local text state decision.\n#[must_use]\n'+match[0]);
  writeFileSync(style,new DecoratorSet([{id:'rust-owned-local-state-library-style',level:'emitter',order:0,when:[['format','Rust']],actions}]).toLino());
 }
 const generated='// Generated from the shared owned local state JavaScript kernel. Do not edit.\n'+formatProductRuleSource(DecoratorSet.fromLino(readFileSync(style,'utf8')).decorate('emitter',{format:'Rust',source:raw}).source);
 const native=new URL('../rust/src/translation/rust/owned_local_state_rules.rs',import.meta.url),copy=new URL('../rust/src/data/rust-owned-local-state.lino',import.meta.url);
 if(process.argv.includes('--check')){if(readFileSync(native,'utf8')!==generated||!readFileSync(copy).equals(readFileSync(policyFile)))throw new Error('native owned local state rules or policy are stale');}
 else{writeFileSync(native,generated);writeFileSync(copy,readFileSync(policyFile));}
}
if(process.argv[1]===fileURLToPath(import.meta.url)){
 const output=new URL('../js/src/translation/rust-owned-local-state-rules.js',import.meta.url),source=rustOwnedLocalStateRules();
 if(process.argv.includes('--check')){if(readFileSync(output,'utf8')!==source)throw new Error('owned local state decisions are stale');}else writeFileSync(output,source);
 if(!process.argv.includes('--javascript-only'))generateNativeOwnedLocalStateRules();
}
