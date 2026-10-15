// Derive both adapters from the existing typed schema and safe Number domain.
import {readFileSync,writeFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {readProjectData,writeProjectData,checkProjectData} from './project-data.mjs';
import {DecoratorSet} from '../src/decorators.js';

const schema=readProjectData(new URL('../src/data/typed-program-links-schema.lino',import.meta.url));
const offsets=readProjectData(new URL('../src/translation/data/source-span.lino',import.meta.url));
const policy={schemaVersion:1,model:'safe integral typed metadata 1',numberMaximum:offsets.offsetMaximum,
 roles:[{kind:'fixed',field:'bits',values:schema.machineIntegerWidths}],negativeZero:'refuse',
 scope:'only declared integral metadata roles; other scalars retain their existing meaning'};
const policyFile=new URL('../src/translation/data/program-numeric-metadata.lino',import.meta.url);
if(process.argv.includes('--check')){if(!checkProjectData(policyFile,policy))throw new Error('numeric metadata policy is stale');}
else writeProjectData(policyFile,policy);
const source='// Generated from linked integral metadata policy. Do not edit.\n'
 +'/** @param {string} kind @param {string} field @param {number} value @param {boolean} negativeZero @returns {boolean} */\n'
 +'export function acceptProgramIntegralMetadata(kind,field,value,negativeZero){\n'
 +' return ('+policy.roles.map(role=>'kind === '+JSON.stringify(role.kind)+' && field === '+JSON.stringify(role.field)+' && ('+role.values.map(value=>'value === '+value).join(' || ')+')').join(' || ')+')'
 +' && !negativeZero && value >= 0 && value <= '+policy.numberMaximum+' && value % 1 === 0;\n}\n';
const javascript=new URL('../src/program-numeric-metadata-rules.js',import.meta.url);
if(process.argv.includes('--check')){if(readFileSync(javascript,'utf8')!==source)throw new Error('numeric metadata JavaScript rules are stale');}
else writeFileSync(javascript,source);
if(!process.argv.includes('--javascript-only')){
 const {parseJavaScript}=await import('../src/translation/javascript.js');
 const {checkProgram}=await import('../src/translation/check.js');
 const {emitRust}=await import('../src/translation/emit-rust.js');
 const raw=emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
 const style=new URL('../../parity/self-translation/program-numeric-metadata-rules-decorators.lino',import.meta.url);
 if(process.argv.includes('--update-styles')){
  const actions=[];let current=raw;
  const replace=(from,to)=>{if(!current.includes(from))throw new Error('numeric metadata style target is absent: '+from);actions.push({op:'replace',field:'source',from,to});current=current.split(from).join(to);};
  for(const name of ['kind','field'])replace(name+': String',name+': &str');
  const opening=current.indexOf('{'),body=current.slice(opening+1,-1);
  // The complete classifier contains only pure scalar conjunctions and one
  // equal-result width disjunction. Factoring that disjunction into a fixed
  // membership query preserves NaN and signed-zero decisions exactly.
  const stack=[],groups=[];
  for(let index=0;index<body.length;index++){
   if(body[index]==='(')stack.push(index);
   else if(body[index]===')'){
    const start=stack.pop(),group=body.slice(start,index+1);
    if(group.includes('||')&&!group.includes('&&'))groups.push(group);
   }
  }
  const widths=groups.sort((left,right)=>right.length-left.length)[0];
  const values=[...widths.matchAll(/value == \((\d+)f64\)/gu)].map(match=>Number(match[1]));
  if(JSON.stringify(values)!==JSON.stringify(policy.roles[0].values))throw new Error('the emitted width comparison group changed');
  let corrected=body.replace(widths,'ml_numeric_metadata_widths').replace(/[()]/gu,'').trim();
  corrected=corrected.replace('ml_numeric_metadata_widths','['+values.map(value=>value+'f64').join(', ')+'].contains(&value)');
  const maximum=String(policy.numberMaximum).replace(/\B(?=(\d{3})+(?!\d))/gu,'_');
  corrected=corrected.replace('value >= 0f64 && value <= '+policy.numberMaximum+'f64','(0.0_f64..='+maximum+'_f64).contains(&value)');
  replace(body,'\n    '+corrected+'\n');
  replace('pub fn accept_program_integral_metadata','/// Apply the shared integral metadata role policy.\n#[must_use]\npub fn accept_program_integral_metadata');
  writeFileSync(style,new DecoratorSet([{id:'program-numeric-metadata-library-style',level:'emitter',order:0,when:[['format','Rust']],actions}]).toLino());
 }
 const decorated=DecoratorSet.fromLino(readFileSync(style,'utf8')).decorate('emitter',{format:'Rust',source:raw}).source;
 const formatted=spawnSync('rustfmt',['--edition','2024','--emit','stdout'],{input:decorated,encoding:'utf8'});
 if(formatted.error||formatted.status!==0)throw formatted.error??new Error(formatted.stderr);
 const generated='// Generated through the checked JavaScript translator. Do not edit.\n'+formatted.stdout;
 const native=new URL('../../rust/src/typed_program_links/program_numeric_metadata_rules.rs',import.meta.url),copy=new URL('../../rust/src/data/program-numeric-metadata.lino',import.meta.url);
 if(process.argv.includes('--check')){if(readFileSync(native,'utf8')!==generated||!readFileSync(copy).equals(readFileSync(policyFile)))throw new Error('native numeric metadata rules or policy are stale');}
 else{writeFileSync(native,generated);writeFileSync(copy,readFileSync(policyFile));}
}
console.log('numeric metadata: one fixed-width role derived from the shared typed schema');
