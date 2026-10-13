import {readFileSync,writeFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {readProjectData} from './project-data.mjs';
import {DecoratorSet} from '../src/decorators.js';
const policyFile=new URL('../src/translation/data/rust-scalar-loops.lino',import.meta.url),policy=readProjectData(policyFile);
const choices=(name,parameter,values)=>'/** @param {string} '+parameter+' @returns {boolean} */\nexport function '+name+'('+parameter+') {return '+values.map(value=>parameter+' === '+JSON.stringify(value)).join(' || ')+';}\n';
const source='// Generated from linked scalar loop source policy.\n'
 +choices('acceptRustScalarLoopModel','model',['',policy.model])
 +choices('acceptScalarLoopLanguage','language',policy.languages)
 +choices('acceptScalarLoopValueKind','kind',Object.keys(policy.types))
 +choices('acceptScalarLoopBinaryOperation','operation',policy.operations.binary)
 +choices('acceptScalarLoopUnaryOperation','operation',policy.operations.unary)
 +choices('acceptScalarLoopMathOperation','operation',policy.operations.math)
 +'/** @param {string} kind @param {string} owner @returns {boolean} */\nexport function acceptScalarLoopPrimitiveOwner(kind,owner) {return '+Object.entries(policy.types).map(([kind,owner])=>'kind === '+JSON.stringify(kind)+' && owner === '+JSON.stringify(owner)).join(' || ')+';}\n';
const javascript=new URL('../src/translation/scalar-loop-rules.js',import.meta.url),native=new URL('../../rust/src/translation/scalar_loop_rules.rs',import.meta.url),copy=new URL('../../rust/src/data/rust-scalar-loops.lino',import.meta.url),style=new URL('../../parity/self-translation/scalar-loop-rules-decorators.lino',import.meta.url);
if(process.argv.includes('--check')){if(readFileSync(javascript,'utf8')!==source)throw new Error('scalar loop JS rules are stale');}else writeFileSync(javascript,source);
const {parseJavaScript}=await import('../src/translation/javascript.js');
const {checkProgram}=await import('../src/translation/check.js');
const {emitRust}=await import('../src/translation/emit-rust.js');
const raw=emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
if(process.argv.includes('--update-styles')){
 const actions=[];let current=raw;
 const replace=(from,to)=>{if(current.includes(from)){actions.push({op:'replace',field:'source',from,to});current=current.split(from).join(to);}};
 for(const name of ['model','language','kind','operation','owner'])replace(name+': String',name+': &str');
 for(const match of raw.matchAll(/^pub fn (\w+)/gmu))replace('pub fn '+match[1],'/// Read the shared closed scalar loop admission.\n#[must_use]\npub fn '+match[1]);
 for(const line of current.split('\n'))if(/^\s*(?:if )?\(/u.test(line))replace(line,line.replace(/[()]/gu,''));
 replace('model == ""','model.is_empty()');replace('signed == false','!signed');replace('signed == true','signed');
 while(/\} else \{\n\s+if /u.test(current)){
  const start=current.match(/\} else \{\n\s+if /u).index,open=start+7;let depth=0,end=open;
  for(;end<current.length;end++){if(current[end]==='{')depth++;else if(current[end]==='}'&&--depth===0)break;}
  const from=current.slice(start,end+1),to=from.replace(/\} else \{\n\s+if /u,'} else if ').slice(0,-1);replace(from,to);
 }
 writeFileSync(style,new DecoratorSet([{id:'scalar-loop-library-style',level:'emitter',order:0,when:[['format','Rust']],actions}]).toLino());
}
const decorated=DecoratorSet.fromLino(readFileSync(style,'utf8')).decorate('emitter',{format:'Rust',source:raw}).source;
const result=spawnSync('rustfmt',['--edition','2024','--emit','stdout'],{input:decorated,encoding:'utf8'});if(result.error||result.status!==0)throw result.error??new Error(result.stderr);
const generated='// Generated through the checked JavaScript translator. Do not edit.\n'+result.stdout;
if(process.argv.includes('--check')){if(readFileSync(native,'utf8')!==generated||!readFileSync(copy).equals(readFileSync(policyFile)))throw new Error('native scalar loop rules or policy are stale');}else{writeFileSync(native,generated);writeFileSync(copy,readFileSync(policyFile));}
console.log('Scalar loops: model, owner and operation decisions are shared');
