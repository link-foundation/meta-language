import {readFileSync,writeFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {readProjectData} from './project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {DecoratorSet} from '../src/decorators.js';
const policyFile=new URL('../src/translation/data/rust-source-signatures.lino',import.meta.url),policy=readProjectData(policyFile);
const source='// Generated from linked Rust source signature policy.\n'
 +'/** @param {string} model @returns {boolean} */\nexport function acceptRustSourceSignatureModel(model) { return model === "" || model === '+JSON.stringify(policy.model)+'; }\n'
 +'/** @param {string} wrapper @returns {boolean} */\nexport function acceptRustSourceSignatureWrapper(wrapper) {return '+policy.wrappers.map(value=>'wrapper === '+JSON.stringify(value)).join(' || ')+';}\n'
 +'/** @param {string} kind @returns {boolean} */\nexport function acceptRustSourceSignatureOwnerKind(kind) {return '+policy.ownerKinds.map(value=>'kind === '+JSON.stringify(value)).join(' || ')+';}\n'
 +'/** @param {string} kind @returns {boolean} */\nexport function acceptRustSourceSignatureCatalogOwner(kind) {return '+policy.typeOwnerKinds.map(value=>'kind === '+JSON.stringify(value)).join(' || ')+';}\n'
 +'/** @param {string} kind @param {string} path @param {string} bits @param {boolean} signed @returns {boolean} */\nexport function acceptRustSourceSignaturePrimitive(kind, path, bits, signed) {\n'
 +Object.entries(policy.primitives).map(([path,type])=>'  if (path === '+JSON.stringify(path)+') return kind === '+JSON.stringify(type.kind)+(type.bits?' && bits === '+JSON.stringify(String(type.bits))+' && signed === '+type.signed:'')+';').join('\n')+'\n  return false;\n}\n';
const javascript=new URL('../src/translation/rust-source-signature-rules.js',import.meta.url),native=new URL('../../rust/src/translation/source_signature_rules.rs',import.meta.url),copy=new URL('../../rust/src/data/rust-source-signatures.lino',import.meta.url),style=new URL('../../parity/self-translation/rust-source-signature-rules-decorators.lino',import.meta.url);
if(process.argv.includes('--check')){if(readFileSync(javascript,'utf8')!==source)throw new Error('source signature JS rules are stale');}else writeFileSync(javascript,source);
const raw=emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
if(process.argv.includes('--update-styles')){
 const actions=[];let current=raw;
 const replace=(from,to)=>{if(current.includes(from)){actions.push({op:'replace',field:'source',from,to});current=current.split(from).join(to);}};
 for(const name of ['model','wrapper','kind','path','bits'])replace(name+': String',name+': &str');
 for(const match of raw.matchAll(/^pub fn (\w+)/gmu))replace('pub fn '+match[1],'/// Read the shared owned source signature admission.\n#[must_use]\npub fn '+match[1]);
 for(const line of current.split('\n'))if(/^\s*(?:if )?\(/u.test(line))replace(line,line.replace(/[()]/gu,''));
 replace('model == ""','model.is_empty()');replace('signed == false','!signed');replace('signed == true','signed');
 while(/\} else \{\n\s+if /u.test(current)){
  const start=current.match(/\} else \{\n\s+if /u).index,open=start+7;let depth=0,end=open;
  for(;end<current.length;end++){if(current[end]==='{')depth++;else if(current[end]==='}'&&--depth===0)break;}
  const from=current.slice(start,end+1),to=from.replace(/\} else \{\n\s+if /u,'} else if ').slice(0,-1);replace(from,to);
 }
 writeFileSync(style,new DecoratorSet([{id:'rust-source-signature-library-style',level:'emitter',order:0,when:[['format','Rust']],actions}]).toLino());
}
const decorated=DecoratorSet.fromLino(readFileSync(style,'utf8')).decorate('emitter',{format:'Rust',source:raw}).source;
const result=spawnSync('rustfmt',['--edition','2024','--emit','stdout'],{input:decorated,encoding:'utf8'});if(result.error||result.status!==0)throw result.error??new Error(result.stderr);
const generated='// Generated through the checked JavaScript translator. Do not edit.\n'+result.stdout;
if(process.argv.includes('--check')){if(readFileSync(native,'utf8')!==generated||!readFileSync(copy).equals(readFileSync(policyFile)))throw new Error('native source signature rules or policy are stale');}else{writeFileSync(native,generated);writeFileSync(copy,readFileSync(policyFile));}
console.log('Rust source signatures: owner, wrapper and primitive decisions are shared');
