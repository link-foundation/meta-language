// Update source inventory only. This records no executed requirement observation.
import assert from 'node:assert/strict';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {readFileSync,writeFileSync} from 'node:fs';
import {renderRustTest,renderRustItemImports,resolveExports,rustPublicSurface} from './generate-export-parity.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const file=new URL('../../parity/language-features.lino',import.meta.url),value=readProjectData(file);
const id='dynamic-language-command-archives';
const common=['parity/fixtures/dynamic-language-command.lino','parity/self-translation/dynamic-language-command-rules-native.lino'];
const row={id,required:true,
 javascript:{status:'implemented',evidence:['js/src/dynamic-language-cli.js','js/src/dynamic-language-command-rules.js','js/src/dynamic-language-command-program.js','js/src/grammar-inference.js','js/src/grammar-inference-refinement.js','js/tests/dynamic-language-command.test.js',...common]},
 rust:{status:'implemented',evidence:['rust/src/dynamic_language_cli.rs','rust/src/dynamic_language_cli/arguments.rs','rust/src/dynamic_language_cli/configuration.rs','rust/src/dynamic_language_command_rules.rs','rust/src/dynamic_language_command_program.rs','rust/src/grammar_inference.rs','rust/src/grammar_inference/refinement.rs','rust/tests/dynamic_language_command.rs',...common]},
};
const previous=value.features.findIndex(entry=>entry.id===id);
if(previous<0)value.features.push(row);else value.features[previous]=row;
const storeId='link-cli-complete-archive-store';
const storeControls=['js/tests/dynamic-language-command.test.js','rust/tests/dynamic_language_command.rs',
 'parity/fixtures/dynamic-language-command.lino','experiments/link-cli-archive-reconstruction.mjs',
 'parity/self-translation/link-cli-archive-topology-rules-native.lino'];
const storeRow={id:storeId,required:true,
 javascript:{status:'implemented',evidence:['js/src/meta-language-archive.js','js/src/meta-language-archive-cli.js',
  'js/src/link-cli-archive-topology-rules.js',...storeControls]},
 rust:{status:'implemented',evidence:['rust/src/meta_language_archive/link_cli_store.rs','rust/src/archive_cli.rs',
  'rust/src/link_cli_archive_topology_rules.rs',...storeControls]},
};
const previousStore=value.features.findIndex(entry=>entry.id===storeId);
if(previousStore<0)value.features.push(storeRow);else value.features[previousStore]=storeRow;
// These declarations identify source counterparts; they are not observations.
const source=readFileSync(new URL('../src/index.js',import.meta.url),'utf8');
const modules=new Set(['meta-language-archive.js','dynamic-meta-language.js','dynamic-language-cli.js',
 'dynamic-language-command-program.js','grammar-inference.js','grammar-inference-refinement.js']);
const names=[];
for(const [,bindings,file] of source.matchAll(/export \{([^}]+)\} from '\.\/([^']+)';/gu)){
 if(modules.has(file)) names.push(...bindings.split(',').map(name=>name.trim()).filter(Boolean));
}
const {entries,missing}=resolveExports(names,rustPublicSurface());
assert.deepEqual(missing,[]);
Object.assign(value.javascriptExports,entries);
value.javascriptExports=Object.fromEntries(Object.entries(value.javascriptExports).sort(([first],[second])=>first<second?-1:first>second?1:0));
writeProjectData(file,value);
// Render declarations from the recorded source mapping only. Do not import the
// product index, resolve/execute runtimes or run the generated compiler test.
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity.rs',import.meta.url),renderRustTest(value.javascriptExports));
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity/items.rs',import.meta.url),renderRustItemImports(value.javascriptExports));
