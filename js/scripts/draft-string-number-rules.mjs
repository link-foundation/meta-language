import { draftSourceLibrary } from './draft-source-library.mjs';
import {writeFileSync} from 'node:fs';
import {readPortableLibraryRelocation} from './record-rule-library.mjs';
import {writeProjectData} from './project-data.mjs';
import {emitLean} from '../src/translation/emit-lean.js';
import {emitRocq} from '../src/translation/emit-rocq.js';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const {emitted,program}=draftSourceLibrary({sourcePath:'js/src/translation/string-number-rules.js',moduleName:'string_number_rules',sourceDependencies:true,
  model:'exact JavaScript StringNumericLiteral binary64 conversion 1',programAsset:'string-number-program',sourceFormatting:'preserve'});
const root=new URL('../../',import.meta.url);
const header=emitted.text.split('\n').find(line=>line.startsWith('#![allow('));
if(!header)throw new TypeError('the complete emitted library requires its actual attributes');
const dependencies=emitted.preludes.filter(source=>!/^pub mod ml \{/mu.test(source));
const original=[header,'use crate::ml;',...dependencies,...emitted.definitions].join('\n\n');
const relocation=readPortableLibraryRelocation(original);
const runtime='pub mod ml_string_number {\n'+relocation.source+'\n}\n';
writeFileSync(new URL('rust/src/translation/runtime/string-number.rs',root),runtime);
writeFileSync(new URL('js/src/translation/string-number-runtime.js',root),
  '// Generated from complete checked numeric conversion definitions. Do not edit.\nexport const STRING_NUMBER_PRELUDE = '+JSON.stringify(runtime)+';\n');
writeProjectData(new URL('parity/self-translation/string-number-runtime.lino',root),{
  schemaVersion:1,model:'complete numeric conversion runtime lexical relocation 1',original,
  actions:relocation.actions,edits:relocation.edits,runtime,
  integerOwner:'ordinary emitted crate::ml; no replacement integer or decimal arithmetic',
  boundary:'Complete source construction only; no represented invocation, fixture, target execution, test or native compilation.',
});
// All formal target bodies are emitted from the same actual checked program.
// Stable namespace entry aliases point to the emitter's real binding mappings.
const formal={};
for(const [language,emit,extension] of [['Lean',emitLean,'lean'],['Rocq',emitRocq,'v']]){
  const output=emit(program);
  const aliases=[['convertUnits','convertJavaScriptStringNumberUnits'],['convertInteger','convertJavaScriptBigIntegerNumber'],['convertUtf8','convertJavaScriptUtf8StringNumberUnits']].map(([alias,source])=>{
    const target=output.mappings.find(mapping=>mapping.kind==='function'&&mapping.source===source)?.target;
    if(!target)throw new TypeError('the complete conversion runtime needs its actual '+language+' entry '+source);
    return language==='Lean'?'def '+alias+' := '+target:'Definition '+alias+' := '+target+'.';
  });
  const lines=output.text.split('\n');
  const imports=lines.filter(line=>/^From .* Require Import .*\.$/u.test(line));
  const body=lines.filter(line=>!/^From .* Require Import .*\.$/u.test(line)).join('\n');
  const text=language==='Lean'?'namespace MLStringNumber\n'+body+'\n'+aliases.join('\n')+'\nend MLStringNumber\n'
    :imports.join('\n')+'\nModule MLStringNumber.\n'+body+'\n'+aliases.join('\n')+'\nEnd MLStringNumber.\n';
  formal[language]={text,emitted:output,aliases};
  writeFileSync(new URL('rust/src/translation/runtime/string-number.'+extension,root),text);
}
writeFileSync(new URL('js/src/translation/string-number-formal-runtime.js',root),
  '// Generated from the same complete checked numeric definitions. Do not edit.\n'
  +'export const STRING_NUMBER_LEAN_PRELUDE = '+JSON.stringify(formal.Lean.text)+';\n'
  +'export const STRING_NUMBER_ROCQ_PRELUDE = '+JSON.stringify(formal.Rocq.text)+';\n');
writeProjectData(new URL('parity/self-translation/string-number-formal-runtime.lino',root),{
  schemaVersion:1,model:'complete checked numeric conversion formal runtime emission 1',formal,
  boundary:'Source emission only; no native Lean/Rocq invocation, fixture execution, comparison or observations.',
});
