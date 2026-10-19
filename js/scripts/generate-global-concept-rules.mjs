#!/usr/bin/env node
// Translate every closed global-concept operation group. Separate emission
// keeps historical proof contexts independent of new identity helpers.
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {readFileSync,writeFileSync} from 'node:fs';
import {DecoratorSet} from '../src/decorators.js';
import {decorateEmitted} from '../src/grammar-emitters/common.js';
import {checkProgram} from '../src/translation/check.js';
import {emitRust} from '../src/translation/emit-rust.js';
import {parseJavaScript} from '../src/translation/javascript.js';
import {readGlobalConceptIdentityStyles,readGlobalConceptIdentityCompilerStyles} from './global-concept-identity-styles.mjs';
import {readProjectData} from './project-data.mjs';

const sourcePath=new URL('../src/global-concept-rules.js',import.meta.url);
const decoratorsPath=new URL('../../parity/self-translation/global-concept-rules-decorators.lino',import.meta.url);
const identityStylesPath=new URL('../../parity/self-translation/global-concept-identity-decorators.lino',import.meta.url);
const identityFeedbackPath=new URL('../../parity/self-translation/global-concept-identity-compiler-feedback.lino',import.meta.url);
const identityCompilerStylesPath=new URL('../../parity/self-translation/global-concept-identity-compiler-decorators.lino',import.meta.url);
const decorators=DecoratorSet.fromLino(readFileSync(decoratorsPath,'utf8'));
const source=readFileSync(sourcePath,'utf8'),boundary='/** @param {string[]} identities';
assert.equal(source.split(boundary).length,2,'the closed identity operation group has one boundary');
const offset=source.indexOf(boundary),proofSource=source.slice(0,offset),identitySource=source.slice(offset);
assert.equal(proofSource+identitySource,source,'the closed groups retain every source byte');
const whole=checkProgram(parseJavaScript(source)),proof=checkProgram(parseJavaScript(proofSource)),identity=checkProgram(parseJavaScript(identitySource));
const exported=new Set([...source.matchAll(/^export function (\w+)\(/gmu)].map(match=>match[1]));
const declarations=program=>program.items.filter(item=>exported.has(item.fullName)).map(({k,fullName})=>({k,fullName}));
assert.deepEqual([...declarations(proof),...declarations(identity)],declarations(whole),'every checked source declaration is retained exactly once');
assert.equal(proof.items.length+identity.items.length,whole.items.length,'all automatically lowered declarations remain present');
const format=input=>{
  const result=spawnSync('rustfmt',['--edition','2024','--emit','stdout'],{input,encoding:'utf8',maxBuffer:4*1024*1024});
  if(result.error||result.status!==0)throw result.error??new Error(result.stderr);
  return result.stdout;
};
const families=[
  {program:proof,file:'global_concept_rules.rs',styles:decorators},
  {program:identity,file:'global_concept_identity_rules.rs',styles:new DecoratorSet([
    {id:'global-concept-identity-module',level:'emitter',order:0,when:[['format','Rust']],actions:[{op:'replace',field:'line',from:'crate::',to:'crate::global_concept_identity_rules::'}]},
    ...decorators.decorators.filter(({id})=>id!=='global-concept-module'),
  ])},
];
for(const family of families){
  let final=format(decorateEmitted('Rust',{source:emitRust(family.program).text},family.styles).source);
  for(let round=0;round<10;round++){
    const next=format(family.styles.decorate('emitter',{format:'Rust',scope:'formatted-module',source:final}).source);
    if(next===final)break;
    final=next;
    if(round===9)throw new Error('global concept style decorators do not converge');
  }
  if(family.program===identity){
    const linked=readGlobalConceptIdentityStyles(final).toLino();
    if(process.argv.includes('--check'))assert.equal(readFileSync(identityStylesPath,'utf8'),linked,'global concept identity styles are fresh');
    else writeFileSync(identityStylesPath,linked);
    final=format(DecoratorSet.fromLino(linked).decorate('emitter',{format:'Rust',scope:'formatted-module',source:final}).source);
  }
  let text='// Generated from js/src/global-concept-rules.js by\n// js/scripts/generate-global-concept-rules.mjs. Do not edit by hand.\n'+final;
  if(family.program===identity){
    const styles=readGlobalConceptIdentityCompilerStyles(text,readProjectData(identityFeedbackPath));
    const linked=styles.toLino();
    if(process.argv.includes('--check'))assert.equal(readFileSync(identityCompilerStylesPath,'utf8'),linked,'identity compiler styles retain exact CI feedback');
    else writeFileSync(identityCompilerStylesPath,linked);
    text=format(styles.decorate('emitter',{format:'Rust',scope:'formatted-module',source:text}).source);
  }
  const output=new URL('../../rust/src/'+family.file,import.meta.url);
  if(process.argv.includes('--check'))assert.equal(readFileSync(output,'utf8'),text,'generated global concept rules are fresh: '+family.file);
  else writeFileSync(output,text);
}
console.log('global concept proof and identity rules retain every checked declaration in separate generated modules');
