// Derive both adapters from the existing typed schema and safe Number domain.
import {readFileSync,writeFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {readProjectData,writeProjectData,checkProjectData} from './project-data.mjs';
import {DecoratorSet} from '../src/decorators.js';
import {acceptTupleIndex} from '../src/translation/primitive-value-rules.js';

const schema=readProjectData(new URL('../src/data/typed-program-links-schema.lino',import.meta.url));
const offsets=readProjectData(new URL('../src/translation/data/source-span.lino',import.meta.url));
const policy={schemaVersion:1,model:'safe integral typed metadata 1',numberMaximum:offsets.offsetMaximum,
 roles:[{kind:'fixed',field:'bits',values:schema.machineIntegerWidths}],negativeZero:'refuse',
 orderedPosition:{sourceOperation:'acceptTupleIndex',role:'owned occurrence path',negativeZero:'same position as zero',numberMaximum:offsets.offsetMaximum},
 scope:'only declared integral metadata roles; other scalars retain their existing meaning'};
const policyFile=new URL('../src/translation/data/program-numeric-metadata.lino',import.meta.url);
if(process.argv.includes('--check')){if(!checkProjectData(policyFile,policy))throw new Error('numeric metadata policy is stale');}
else writeProjectData(policyFile,policy);
const source='// Generated from linked integral metadata policy. Do not edit.\n'
 +'/** @param {string} kind @param {string} field @param {number} value @param {boolean} negativeZero @returns {boolean} */\n'
 +'export function acceptProgramIntegralMetadata(kind,field,value,negativeZero){\n'
 +' return ('+policy.roles.map(role=>'kind === '+JSON.stringify(role.kind)+' && field === '+JSON.stringify(role.field)+' && ('+role.values.map(value=>'value === '+value).join(' || ')+')').join(' || ')+')'
 +' && !negativeZero && value >= 0 && value <= '+policy.numberMaximum+' && value % 1 === 0;\n}\n'
 +'/** @param {number} index @param {number} length @returns {boolean} */\n'
 +acceptTupleIndex.toString().replace('function acceptTupleIndex(', 'export function acceptProgramOrderedPosition(')+'\n';
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
  const actions=[];
  for(const match of raw.matchAll(/pub fn (\w+)\([^]*?\n\}/gu)){
  const original=match[0],opening=original.indexOf('{'),body=original.slice(opening+1,-1);
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
  let corrected=body;
  if(match[1]==='accept_program_integral_metadata'){
   const widths=groups.sort((left,right)=>right.length-left.length)[0];
   const values=[...widths.matchAll(/value == \((\d+)f64\)/gu)].map(match=>Number(match[1]));
   if(JSON.stringify(values)!==JSON.stringify(policy.roles[0].values))throw new Error('the emitted width comparison group changed');
   corrected=corrected.replace(widths,'ml_numeric_metadata_widths');
   corrected=corrected.replace(/[()]/gu,'').trim();
   corrected=corrected.replace('ml_numeric_metadata_widths','['+values.map(value=>value+'f64').join(', ')+'].contains(&value)');
  }else corrected=corrected.replace(/[()]/gu,'').trim();
  const maximum=String(policy.numberMaximum).replace(/\B(?=(\d{3})+(?!\d))/gu,'_');
  for(const parameter of['value','index'])corrected=corrected.replace(parameter+' >= 0f64 && '+parameter+' <= '+policy.numberMaximum+'f64','(0.0_f64..='+maximum+'_f64).contains(&'+parameter+')');
  let header=original.slice(0,opening+1);
  for(const parameter of['kind','field'])header=header.replace(parameter+': String',parameter+': &str');
  actions.push({op:'replace',field:'source',from:original,to:'/// Apply a shared integral metadata role decision.\n#[must_use]\n'+header+'\n    '+corrected+'\n}'});
  }
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
console.log('numeric metadata: one shared fixed-width role and the existing ordered tuple-index decision');
