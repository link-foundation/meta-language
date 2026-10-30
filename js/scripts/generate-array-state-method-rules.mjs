import { draftSourceLibrary } from './draft-source-library.mjs';
import { readFileSync, writeFileSync } from 'node:fs';
import { writeProjectData } from './project-data.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft for source-only authoring');
draftSourceLibrary({ sourcePath: 'js/src/translation/array-state-method-rules.js', moduleName:'array_state_method_rules',
  model:'dense shared array range mutations 1', programAsset:'array-state-method-program', sourceFormatting:'preserve' });
const operations=[['fill-array-elements','fill'],['copy-array-elements-within','copyWithin']].map(([identity,method])=>({identity,method,
  source:'('+identity+': (kind array) (method '+method+') (primitive '+identity+') (mutates true) (program (read-reference receiver value) (invoke-array-method '+method+' value arguments result) (write-reference receiver value) (return-value result)))'}));
const files=[];
for(const path of ['js/src/data/reference-operations.lino','rust/src/data/reference-operations.lino']){
  const file=new URL('../../'+path,import.meta.url);let source=readFileSync(file,'utf8');
  for(const operation of operations){
    const current=source.split('\n').find(line=>line.startsWith('('+operation.identity+':'));
    if(current!==undefined&&current!==operation.source)throw new TypeError('an existing qualified array operation has a different complete definition');
    if(current===undefined)source+=(source.endsWith('\n')?'':'\n')+operation.source+'\n';
  }
  writeFileSync(file,source);files.push({path,source});
}
writeProjectData(new URL('../../parity/self-translation/array-state-method-operations.lino',import.meta.url),{
  schemaVersion:1,operations,files,boundary:'Additive complete linked operation definitions. No represented operation or behavior has executed.',
});
