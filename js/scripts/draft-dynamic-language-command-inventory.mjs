// Update source inventory only. This records no executed requirement observation.
import {readProjectData,writeProjectData} from './project-data.mjs';
import {writeFileSync} from 'node:fs';
import {renderRustTest,renderRustItemImports} from './generate-export-parity.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const file=new URL('../../parity/language-features.lino',import.meta.url),value=readProjectData(file);
const id='dynamic-language-command-archives';
const common=['parity/fixtures/dynamic-language-command.lino','parity/self-translation/dynamic-language-command-rules-native.lino'];
const row={id,required:true,
 javascript:{status:'implemented',evidence:['js/src/dynamic-language-cli.js','js/src/dynamic-language-command-rules.js','js/src/dynamic-language-command-program.js','js/tests/dynamic-language-command.test.js',...common]},
 rust:{status:'implemented',evidence:['rust/src/dynamic_language_cli.rs','rust/src/dynamic_language_cli/arguments.rs','rust/src/dynamic_language_cli/configuration.rs','rust/src/dynamic_language_command_rules.rs','rust/src/dynamic_language_command_program.rs','rust/tests/dynamic_language_command.rs',...common]},
};
const previous=value.features.findIndex(entry=>entry.id===id);
if(previous<0)value.features.push(row);else value.features[previous]=row;
// These declarations identify source counterparts; they are not observations.
value.javascriptExports.runDynamicLanguageCommand={rust:'run_dynamic_language_command',kind:'function'};
value.javascriptExports.createDynamicLanguageCommandProgramLinks={rust:'create_dynamic_language_command_program_links',kind:'function'};
value.javascriptExports=Object.fromEntries(Object.entries(value.javascriptExports).sort(([first],[second])=>first<second?-1:first>second?1:0));
writeProjectData(file,value);
// Render declarations from the recorded source mapping only. Do not import the
// product index, resolve/execute runtimes or run the generated compiler test.
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity.rs',import.meta.url),renderRustTest(value.javascriptExports));
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity/items.rs',import.meta.url),renderRustItemImports(value.javascriptExports));
