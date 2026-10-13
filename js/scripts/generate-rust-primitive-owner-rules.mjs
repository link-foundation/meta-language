import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData} from './project-data.mjs';
import {DecoratorSet} from '../src/decorators.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {formatProductRuleSource} from './product-rule-style-corrections.mjs';
const policyFile=new URL('../src/translation/data/rust-primitive-owners.lino',import.meta.url),policy=readProjectData(policyFile);
const source='// Generated from linked standard primitive Rust owner policy. Do not edit.\n'
 +'/** @param {string} model @returns {boolean} */\nexport function acceptRustPrimitiveOwnerModel(model){return model === "" || model === '+JSON.stringify(policy.model)+';}\n'
 +'/** @param {string} path @param {boolean} declared @param {boolean} shadowed @returns {boolean} */\nexport function acceptRustStandardPrimitiveOwner(path,declared,shadowed){return !declared && !shadowed && ('+policy.paths.map(path=>'path === '+JSON.stringify(path)).join(' || ')+');}\n'
 +'/** @param {string} kind @returns {boolean} */\nexport function acceptRustPrimitiveOwnerValueType(kind){return '+policy.kinds.map(kind=>'kind === '+JSON.stringify(kind)).join(' || ')+';}\n'
 +'/** @param {string} kind @returns {boolean} */\nexport function acceptRustPrimitiveOwnerExpression(kind){return '+policy.expressions.map(kind=>'kind === '+JSON.stringify(kind)).join(' || ')+';}\n'
 +'/** @param {string} name @returns {string} */\nexport function readRustPrimitiveOwnerConstantValue(name){'+Object.entries(policy.constantValues).map(([name,value])=>'if(name === '+JSON.stringify(name)+')return '+JSON.stringify(value)+';').join('')+'return "";}\n';
const js=new URL('../src/translation/rust-primitive-owner-rules.js',import.meta.url),native=new URL('../../rust/src/translation/rust/primitive_owner_rules.rs',import.meta.url),copy=new URL('../../rust/src/data/rust-primitive-owners.lino',import.meta.url),style=new URL('../../parity/self-translation/rust-primitive-owner-rules-decorators.lino',import.meta.url);
if(process.argv.includes('--check')){if(readFileSync(js,'utf8')!==source)throw new Error('JS primitive owner rules are stale');}else writeFileSync(js,source);
if(!process.argv.includes('--javascript-only')){
 const definitions=emitRust(checkProgram(parseJavaScript(source))).definitions,raw=definitions.join('\n\n');
 if(process.argv.includes('--update-styles')){
  const actions=[];let current=raw;const replace=(from,to)=>{if(current.includes(from)){actions.push({op:'replace',field:'source',from,to});current=current.split(from).join(to);}};
  for(const parameter of ['model','path','kind','name'])replace(parameter+': String',parameter+': &str');
  for(const match of raw.matchAll(/^pub fn (\w+)/gmu))replace('pub fn '+match[1],'/// Read a shared standard primitive source owner decision.\n#[must_use]\npub fn '+match[1]);
  replace('model == ""','model.is_empty()');replace('String::from("")','String::new()');
  for(const match of [...current.matchAll(/if \((name == "[^"]+")\)/gu)])replace(match[0],'if '+match[1]);
  while(/\} else \{\n\s+if /u.test(current)){
   const start=current.match(/\} else \{\n\s+if /u).index,open=start+7;let depth=0,end=open;
   for(;end<current.length;end++){if(current[end]==='{')depth++;else if(current[end]==='}'&&--depth===0)break;}
   const from=current.slice(start,end+1);replace(from,from.replace(/\} else \{\n\s+if /u,'} else if ').slice(0,-1));
  }
  for(const definition of current.matchAll(/pub fn accept_rust_[\s\S]*?\n\}/gu)){
   const block=definition[0],open=block.indexOf('{');let expression=block.slice(open+1,-1).trim();
   expression=expression.replace(/\((\w+ == "[^"]+"|!\w+|\w+\.is_empty\(\))\)/gu,'$1');
   if(expression.startsWith('(')&&expression.endsWith(')'))expression=expression.slice(1,-1);
   replace(block,block.slice(0,open+1)+'\n'+expression+'\n}');
  }
  writeFileSync(style,new DecoratorSet([{id:'rust-primitive-owner-library-style',level:'emitter',order:0,when:[['format','Rust']],actions}]).toLino());
 }
 const generated='// Generated through the checked JavaScript translator. Do not edit.\n'+formatProductRuleSource(DecoratorSet.fromLino(readFileSync(style,'utf8')).decorate('emitter',{format:'Rust',source:raw}).source);
 if(process.argv.includes('--check')){if(readFileSync(native,'utf8')!==generated||!readFileSync(copy).equals(readFileSync(policyFile)))throw new Error('native primitive owner rules or policy are stale');}else{writeFileSync(native,generated);writeFileSync(copy,readFileSync(policyFile));}
}
