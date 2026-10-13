// Shared linked policy -> pure JavaScript decisions -> checked native code.
import {readFileSync,writeFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {readProjectData} from './project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {DecoratorSet} from '../src/decorators.js';
const file=new URL('../src/translation/data/rust-float-predicates.lino',import.meta.url);
const policy=readProjectData(file);
const source='// Generated from linked Rust float predicate policy. Do not edit by hand.\n'
  +'/** @param {string} method @returns {boolean} */\nexport function acceptRustFloatPredicateMethod(method) {\n  return '
  +Object.keys(policy.templates).map(method=>'method === '+JSON.stringify(method)).join(' || ')+';\n}\n\n'
  +'/** @param {string} method @returns {boolean} */\nexport function acceptRustFloatOperationMethod(method) {\n  return '
  +Object.keys(policy.operationTemplates).map(method=>'method === '+JSON.stringify(method)).join(' || ')+';\n}\n\n'
  +'/** @param {string} kind @param {string} model @param {boolean} proven @param {boolean} indirected @param {boolean} shadowed @param {string} argumentsCount @returns {boolean} */\n'
  +'export function acceptRustFloatPredicateReceiver(kind, model, proven, indirected, shadowed, argumentsCount) {\n  return proven && !indirected && !shadowed && kind === '+JSON.stringify(policy.kind)+' && model === '+JSON.stringify(policy.receiverModel)+' && argumentsCount === '+JSON.stringify(policy.arguments)+';\n}\n';
const javascript=new URL('../src/translation/rust-float-predicate-rules.js',import.meta.url);
const native=new URL('../../rust/src/translation/rust/float_predicate_rules.rs',import.meta.url);
const decorators=new URL('../../parity/self-translation/rust-float-predicate-rules-decorators.lino',import.meta.url);
if(!process.argv.includes('--check'))writeFileSync(javascript,source);else if(readFileSync(javascript,'utf8')!==source)throw new Error('JavaScript float predicate rules are stale');
const raw=emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
if(process.argv.includes('--update-styles')){
  const actions=[];
  const replace=(from,to)=>{if(raw.includes(from))actions.push({op:'replace',field:'source',from,to});};
  // Each classifier consists only of one homogeneous boolean chain over
  // scalar comparisons; removing grouping preserves evaluation and values.
  for (const definition of raw.split('\n\n')) {
    const lines = definition.split('\n').filter(line => line.startsWith('    '));
    if (lines.length !== 1) throw new Error('the scalar predicate policy body changed');
    replace(lines[0].trim(), lines[0].trim().replace(/[()]/gu, ''));
  }
  for(const name of ['method','kind','model','arguments_count'])replace(name+': String',name+': &str');
  for(const match of raw.matchAll(/^pub fn (\w+)/gmu)) {
    const kind = match[1] === 'accept_rust_float_operation_method' ? 'operation' : 'predicate';
    replace('pub fn '+match[1],'/// Read the shared Rust binary64 '+kind+' admission.\n#[must_use]\npub fn '+match[1]);
  }
  writeFileSync(decorators,new DecoratorSet([{id:'rust-float-predicate-library-style',level:'emitter',order:0,when:[['format','Rust']],actions}]).toLino());
}
const styled=DecoratorSet.fromLino(readFileSync(decorators,'utf8')).decorate('emitter',{format:'Rust',source:raw}).source;
const formatted=spawnSync('rustfmt',['--edition','2024','--emit','stdout'],{input:styled,encoding:'utf8'});
if(formatted.error||formatted.status!==0)throw formatted.error??new Error(formatted.stderr);
const generated='// Generated from linked policy through the checked JavaScript translator.\n// Do not edit by hand.\n'+formatted.stdout;
const copy=new URL('../../rust/src/translation/rust/float-predicates.lino',import.meta.url);
if(process.argv.includes('--check')){
 if(readFileSync(native,'utf8')!==generated)throw new Error('Native float predicate rules are stale');
 if(!readFileSync(copy).equals(readFileSync(file)))throw new Error('Native float predicate policy is stale');
}else{writeFileSync(native,generated);writeFileSync(copy,readFileSync(file));}
console.log('Rust float methods: one shared receiver proof, three predicate and four operation templates');
