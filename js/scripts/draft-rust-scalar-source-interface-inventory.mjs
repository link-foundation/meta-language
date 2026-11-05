// Source inventory only: this does not record observations or touch old inputs.
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),file=new URL('parity/language-features.lino',root),value=readProjectData(file),id='retained-scalar-rust-source-interface';
const common=['parity/fixtures/rust-scalar-source-interface.lino','experiments/rust-records/scalar-source-interface.mjs'];
const row={id,required:true,javascript:{status:'implemented',evidence:['js/src/translation/rust-scalar-source-interface-rules.js','js/src/translation/rust-scalar-source-interface.js','js/src/translation/emit-rust.js','js/tests/translation-rust-scalar-source-interface.test.js',...common]},rust:{status:'implemented',evidence:['rust/src/translation/rust_scalar_source_interface_rules.rs','rust/src/translation/scalar_source_interface.rs','rust/src/translation/emit_rust.rs','rust/tests/unit/translation_rust_scalar_source_interface.rs',...common]}};
const at=value.features.findIndex(prior=>prior.id===id);if(at<0)value.features.push(row);else value.features[at]=row;writeProjectData(file,value);
const module=new URL('rust/tests/unit/mod.rs',root),declaration='mod translation_rust_scalar_source_interface;',source=readFileSync(module,'utf8');if(!source.includes(declaration))writeFileSync(module,source+'\n'+declaration+'\n');
