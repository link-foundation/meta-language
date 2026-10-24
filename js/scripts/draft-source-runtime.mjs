// Source author only: complete bodies come from actual checked definitions.
import {writeFileSync} from 'node:fs';
import {draftSourceLibrary} from './draft-source-library.mjs';
import {readPortableLibraryRelocation} from './record-rule-library.mjs';
import {readPortableSourceParameterStyles} from './portable-source-library-style.mjs';
import {writeProjectData} from './project-data.mjs';
import {emitLean} from '../src/translation/emit-lean.js';
import {emitRocq} from '../src/translation/emit-rocq.js';
import {emitJavaScript} from '../src/translation/emit-javascript.js';
import {tokenize} from '../src/translation/lexer.js';

export function draftSourceRuntime({runtimeStem,constantPrefix,formalNamespace,entries,...sourceOptions}) {
  if(!/^[a-z][a-z0-9-]*$/u.test(runtimeStem)||!/^[A-Z][A-Z_0-9]*$/u.test(constantPrefix)||!/^ML[A-Za-z0-9]+$/u.test(formalNamespace))throw new TypeError('source runtime names need portable target identities');
  const {program,emitted,...rest}=draftSourceLibrary(sourceOptions);
  const root=new URL('../../',import.meta.url);
  const header=emitted.text.split('\n').find(line=>line.startsWith('#![allow('));
  if(!header)throw new TypeError('the complete emitted library requires its actual attributes');
  const dependencies=emitted.preludes.filter(source=>!/^pub mod ml \{/mu.test(source));
  const original=[header,'use crate::ml;',...dependencies,...emitted.definitions].join('\n\n');
  const relocation=readPortableLibraryRelocation(original);
  const loans=readPortableSourceParameterStyles(relocation.source,{publicNames:[...relocation.source.matchAll(/^pub fn (\w+)\(/gmu)].map(match=>match[1])});
  const runtime='pub mod ml_'+runtimeStem.replaceAll('-','_')+' {\n'+loans.source+'\n}\n';
  writeFileSync(new URL('rust/src/translation/runtime/'+runtimeStem+'.rs',root),runtime);
  writeFileSync(new URL('js/src/translation/'+runtimeStem+'-runtime.js',root),
    '// Generated from complete checked source definitions. Do not edit.\nexport const '+constantPrefix+'_PRELUDE = '+JSON.stringify(runtime)+';\n');
  writeProjectData(new URL('parity/self-translation/'+runtimeStem+'-runtime.lino',root),{
    schemaVersion:1,model:'complete source runtime lexical relocation and loans 1',original,
    actions:[...relocation.actions,...loans.actions],edits:relocation.edits,runtime,
    integerOwner:'ordinary emitted crate::ml; no replacement integer arithmetic',
    boundary:'Complete source construction only; no represented invocation, fixture, target execution, test or native compilation.',
  });
  const formal={};
  const javascript=emitJavaScript(program);
  let javascriptBody=javascript.text;
  const javascriptTokens=tokenize(javascriptBody,'JavaScript').tokens;
  const exportTokens=[];
  let braceDepth=0;
  for(const [index,token] of javascriptTokens.entries()){
    if(braceDepth===0&&token.kind==='identifier'&&token.value==='export'&&['function','const','let','class','async'].includes(javascriptTokens[index+1]?.value))exportTokens.push(token);
    if(token.kind==='punct'&&token.value==='{')braceDepth+=1;
    if(token.kind==='punct'&&token.value==='}')braceDepth-=1;
  }
  for(const token of exportTokens.reverse())javascriptBody=javascriptBody.slice(0,token.start)+javascriptBody.slice(token.end);
  const javascriptEntries=entries.map(([alias,source])=>{
    const target=javascript.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===source)?.target;
    if(!target)throw new TypeError('the complete JavaScript runtime needs its actual entry '+source);
    return alias+': '+target;
  });
  const javascriptRuntime='const ml_'+runtimeStem.replaceAll('-','_')+' = (() => {\n'+javascriptBody+'\nreturn { '+javascriptEntries.join(', ')+' };\n})();\n';
  writeFileSync(new URL('rust/src/translation/runtime/'+runtimeStem+'.js',root),javascriptRuntime);
  writeFileSync(new URL('js/src/translation/'+runtimeStem+'-javascript-runtime.js',root),
    '// Generated from complete checked source definitions. Do not edit.\nexport const '+constantPrefix+'_JAVASCRIPT_PRELUDE = '+JSON.stringify(javascriptRuntime)+';\n');
  writeProjectData(new URL('parity/self-translation/'+runtimeStem+'-javascript-runtime.lino',root),{
    schemaVersion:1,model:'complete checked source JavaScript runtime emission 1',
    emitted:{language:javascript.language,text:javascript.text,mappings:javascript.mappings,assumptions:javascript.assumptions,encodings:javascript.encodings,theorems:javascript.theorems,entry:javascript.entry,preludes:javascript.preludes,definitions:javascript.definitions},
    exportTokens:exportTokens.map(({kind,value,start,end,raw})=>({kind,value,start,end,raw})),entries:javascriptEntries,runtime:javascriptRuntime,
    boundary:'Source construction only; no represented calls, fixture behavior, oracle, test or observations.',
  });
  for(const [language,emit,extension] of [['Lean',emitLean,'lean'],['Rocq',emitRocq,'v']]){
    const output=emit(program);
    const aliases=entries.map(([alias,source])=>{
      if(!/^[a-z][A-Za-z0-9]*$/u.test(alias))throw new TypeError('a formal entry alias needs one portable source name');
      const target=output.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===source)?.target;
      if(!target)throw new TypeError('the complete runtime needs its actual '+language+' entry '+source);
      return language==='Lean'?'def '+alias+' := '+target:'Definition '+alias+' := '+target+'.';
    });
    const lines=output.text.split('\n');
    const imports=lines.filter(line=>/^From .* Require Import .*\.$/u.test(line));
    const body=lines.filter(line=>!/^From .* Require Import .*\.$/u.test(line)).join('\n');
    const text=language==='Lean'?'namespace '+formalNamespace+'\n'+body+'\n'+aliases.join('\n')+'\nend '+formalNamespace+'\n'
      :imports.join('\n')+'\nModule '+formalNamespace+'.\n'+body+'\n'+aliases.join('\n')+'\nEnd '+formalNamespace+'.\n';
    formal[language]={text,emitted:output,aliases};
    writeFileSync(new URL('rust/src/translation/runtime/'+runtimeStem+'.'+extension,root),text);
  }
  writeFileSync(new URL('js/src/translation/'+runtimeStem+'-formal-runtime.js',root),
    '// Generated from the same complete checked source definitions. Do not edit.\n'
    +'export const '+constantPrefix+'_LEAN_PRELUDE = '+JSON.stringify(formal.Lean.text)+';\n'
    +'export const '+constantPrefix+'_ROCQ_PRELUDE = '+JSON.stringify(formal.Rocq.text)+';\n');
  writeProjectData(new URL('parity/self-translation/'+runtimeStem+'-formal-runtime.lino',root),{
    schemaVersion:1,model:'complete checked source formal runtime emission 1',formal,
    boundary:'Source emission only; no native Lean/Rocq invocation, fixture execution, comparison or observations.',
  });
  return{program,emitted,...rest};
}
