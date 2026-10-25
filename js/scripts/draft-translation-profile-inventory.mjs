// Append concrete source/test entry points without claiming execution evidence.
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
export function draftTranslationProfileInventory({id,fixture,programAsset,javascript,rust,nativeTest}){
 const root=new URL('../../',import.meta.url),file=new URL('parity/language-features.lino',root),value=readProjectData(file);
 const common=['parity/fixtures/'+fixture+'.lino','experiments/javascript-program/'+fixture+'.mjs'];
 const row={id,required:true,javascript:{status:'implemented',evidence:[...javascript,...common]},rust:{status:'implemented',evidence:[...rust,...common]}};
 const at=value.features.findIndex(prior=>prior.id===id);if(at<0)value.features.push(row);else value.features[at]=row;writeProjectData(file,value);
 const matrixFile=new URL('js/tests/translation-source-exceptions.test.js',root);let matrix=readFileSync(matrixFile,'utf8');
 const addition="['"+fixture+"',()=>createSourceDefinitionProgramLinks('meta-language/source-program/"+programAsset+"')],",anchor="['optional-callables',()=>createSourceDefinitionProgramLinks('meta-language/source-program/optional-callable-program')],";
 if(!matrix.includes(addition)){if(matrix.split(anchor).length!==2)throw new TypeError('translation source matrix needs its unique actual insertion anchor');matrix=matrix.replace(anchor,()=>anchor+addition);writeFileSync(matrixFile,matrix);}
 const modulesFile=new URL('rust/tests/unit/mod.rs',root);let modules=readFileSync(modulesFile,'utf8');const declaration='mod '+nativeTest+';';
 if(!modules.includes(declaration)){modules+='\n'+declaration+'\n';writeFileSync(modulesFile,modules);}
}
