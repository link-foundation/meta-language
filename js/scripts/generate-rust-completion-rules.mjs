// Shared linked completion premises, translated through the checked core.
import {readFileSync,writeFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {readProjectData} from './project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {DecoratorSet} from '../src/decorators.js';
const policyFile=new URL('../src/translation/data/rust-completions.lino',import.meta.url);
const policy=readProjectData(policyFile);
const source='// Generated from linked Rust completion policy. Do not edit by hand.\n'
 +'/** @param {string} model @returns {boolean} */\nexport function acceptRustCompletionModel(model) {\n  return model === "" || model === '+JSON.stringify(policy.model)+';\n}\n\n'
 +'/** @param {boolean} proven @param {boolean} indirected @returns {boolean} */\nexport function acceptRustReturnOwnership(proven, indirected) {\n  return proven && !indirected;\n}\n\n'
 +'/** @param {string} thenKind @param {string} otherwiseKind @param {boolean} hasElse @param {boolean} continuation @returns {string} */\nexport function readRustConditionalCompletion(thenKind, otherwiseKind, hasElse, continuation) {\n'
 +policy.actions.map(row=>'  if (thenKind === '+JSON.stringify(row.then)+' && otherwiseKind === '+JSON.stringify(row.otherwise)+' && hasElse === '+row.hasElse+' && continuation === '+row.continuation+') return '+JSON.stringify(row.action)+';').join('\n')
 +'\n  return "unsupported completion";\n}\n\n'
 +'/** @param {string} action @param {string} thenKind @param {string} otherwiseKind @returns {string} */\nexport function readRustCompletionKind(action, thenKind, otherwiseKind) {\n'
 +policy.resultKinds.map(row=>'  if (action === '+JSON.stringify(row.action)+' && thenKind === '+JSON.stringify(row.then)+' && otherwiseKind === '+JSON.stringify(row.otherwise)+') return '+JSON.stringify(row.result)+';').join('\n')
 +'\n  return "unsupported completion";\n}\n\n'
 +'/** @param {string} model @param {string} macroName @param {string} firstKind @param {string} firstValue @param {string} secondKind @param {string} count @returns {boolean} */\nexport function acceptRustCompletionPanicArguments(model, macroName, firstKind, firstValue, secondKind, count) {\n  return model === '+JSON.stringify(policy.model)+' && macroName === '+JSON.stringify(policy.panic.macro)+' && firstKind === '+JSON.stringify(policy.panic.firstKind)+' && firstValue === '+JSON.stringify(policy.panic.template)+' && secondKind === '+JSON.stringify(policy.panic.secondKind)+' && count === '+JSON.stringify(policy.panic.arity)+';\n}\n';
const javascript=new URL('../src/translation/rust-completion-rules.js',import.meta.url);
const native=new URL('../../rust/src/translation/rust/completion_rules.rs',import.meta.url);
const decorators=new URL('../../parity/self-translation/rust-completion-rules-decorators.lino',import.meta.url);
if(process.argv.includes('--check')){if(readFileSync(javascript,'utf8')!==source)throw new Error('JavaScript completion rules are stale');}else writeFileSync(javascript,source);
const raw=emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
if(process.argv.includes('--update-styles')){
 const actions=[];
 let styled=raw;
 const replace=(from,to)=>{if(styled.includes(from)){actions.push({op:'replace',field:'source',from,to});styled=styled.split(from).join(to);}};
 for(const name of ['model','then_kind','otherwise_kind','macro_name','first_kind','first_value','second_kind','count','action'])replace(name+': String',name+': &str');
 for(const match of raw.matchAll(/^pub fn (\w+)/gmu))replace('pub fn '+match[1],'/// Read the shared closed Rust function completion policy.\n#[must_use]\npub fn '+match[1]);
 // These decisions consist of scalar boolean chains and a closed String
 // result. Flatten grouping and nested else-if without changing execution.
 for(const line of styled.split('\n')){
  if(!line.trim().startsWith('if (') && !/^\s*\(*model/u.test(line) && !/^\s*\(*proven/u.test(line))continue;
  replace(line,line.replace(/[()]/gu,''));
 }
 for(const name of ['has_else','continuation']){
  replace(name+' == false','!'+name);replace(name+' == true',name);
 }
 for(const name of ['model','otherwise_kind'])replace(name+' == ""',name+'.is_empty()');
 // Record the exact nested-conditional flattening as an emitter decorator.
 while(/\} else \{\n\s+if /u.test(styled)){
  const match=styled.match(/\} else \{\n(\s+)if /u),start=match.index;
  const open=start+'} else '.length;
  let depth=0,end=open;
  for(;end<styled.length;end++){if(styled[end]==='{')depth++;else if(styled[end]==='}'&&--depth===0)break;}
  const from=styled.slice(start,end+1),to=from.replace(/\} else \{\n\s+if /u,'} else if ').slice(0,-1);
  replace(from,to);
 }
 writeFileSync(decorators,new DecoratorSet([{id:'rust-completion-library-style',level:'emitter',order:0,when:[['format','Rust']],actions}]).toLino());
}
const styled=DecoratorSet.fromLino(readFileSync(decorators,'utf8')).decorate('emitter',{format:'Rust',source:raw}).source;
const formatted=spawnSync('rustfmt',['--edition','2024','--emit','stdout'],{input:styled,encoding:'utf8'});
if(formatted.error||formatted.status!==0)throw formatted.error??new Error(formatted.stderr);
const generated='// Generated from linked completion policy through the checked JavaScript translator.\n// Do not edit by hand.\n'+formatted.stdout;
const nativePolicy=new URL('../../rust/src/translation/rust/completions.lino',import.meta.url);
if(process.argv.includes('--check')){
 if(readFileSync(native,'utf8')!==generated)throw new Error('Native completion rules are stale');
 if(!readFileSync(nativePolicy).equals(readFileSync(policyFile)))throw new Error('Native completion policy is stale');
}else{writeFileSync(native,generated);writeFileSync(nativePolicy,readFileSync(policyFile));}
console.log('Rust completions: closed function profile and '+policy.actions.length+' conditional decisions');
