// Source declaration registration only; no runtime or fixture imports.
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const file=new URL('../../parity/language-features.lino',import.meta.url),value=readProjectData(file),id='javascript-actual-lexical-declaration-early-errors';
const common=['parity/fixtures/lexical-declarations.lino','parity/self-translation/lexical-declaration-rules-native.lino'];
const row={id,required:true,javascript:{status:'implemented',evidence:['js/src/translation/lexical-declaration-rules.js','js/src/translation/javascript.js','js/tests/translation-lexical-declarations.test.js','js/tests/translation-source-exceptions.test.js','experiments/javascript-program/lexical-declarations.mjs',...common]},rust:{status:'implemented',evidence:['rust/src/translation/lexical_declaration_rules.rs','rust/src/translation/javascript/lexical_bindings.rs','rust/tests/unit/translation_javascript_lexical_declarations.rs',...common]}};
const index=value.features.findIndex(row=>row.id===id);if(index<0)value.features.push(row);else value.features[index]=row;writeProjectData(file,value);
const matrixFile=new URL('../tests/translation-source-exceptions.test.js',import.meta.url);let matrix=readFileSync(matrixFile,'utf8');
if(!matrix.includes("['lexical-declarations',()=>createSourceDefinitionProgramLinks(")){
 matrix="import {createSourceDefinitionProgramLinks} from '../src/source-definition-catalog.js';\n"+matrix;
 const anchor="['array-formal-parameters',createJavaScriptArrayFormalParameterProgramLinks],";
 if(matrix.split(anchor).length!==2)throw new TypeError('actual unique lexical matrix anchor is required');
 matrix=matrix.replace(anchor,()=>anchor+"['lexical-declarations',()=>createSourceDefinitionProgramLinks('meta-language/source-program/lexical-declaration-program')],");writeFileSync(matrixFile,matrix);
}
const modulesFile=new URL('../../rust/tests/unit/mod.rs',import.meta.url);let modules=readFileSync(modulesFile,'utf8');
if(!modules.includes('mod translation_javascript_lexical_declarations;')){modules+='\nmod translation_javascript_lexical_declarations;\n';writeFileSync(modulesFile,modules);}
