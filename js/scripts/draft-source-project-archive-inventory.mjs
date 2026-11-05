import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const file=new URL('../../parity/language-features.lino',import.meta.url),data=readProjectData(file),id='source-project-complete-concrete-archives-and-append',common=['parity/fixtures/source-project-archive.lino','experiments/source-project-archive/products.mjs'];
const row={id,required:true,javascript:{status:'implemented',evidence:['js/src/source-project-archive-rules.js','js/src/source-project-archive.js','js/src/rust-source-project.js','js/tests/source-project-archive.test.js','js/tests/rust-source-project.test.js',...common]},rust:{status:'implemented',evidence:['rust/src/source_project_archive_rules.rs','rust/src/source_project_archive.rs','rust/src/rust_source_project.rs','rust/tests/unit/source_project_archive.rs','rust/tests/unit/rust_source_project.rs',...common]}};
const index=data.features.findIndex(row=>row.id===id);if(index<0)data.features.push(row);else data.features[index]=row;writeProjectData(file,data);
const path=new URL('../../rust/tests/unit/mod.rs',import.meta.url);let modules=readFileSync(path,'utf8');if(!modules.includes('mod source_project_archive;'))writeFileSync(path,modules+'\nmod source_project_archive;\n');
