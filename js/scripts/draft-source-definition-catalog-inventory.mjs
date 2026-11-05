// Source/API declarations only; never import the product index or run controls.
import {readFileSync,writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {renderRustTest,renderRustItemImports} from './generate-export-parity.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const file=new URL('../../parity/language-features.lino',import.meta.url),value=readProjectData(file),id='qualified-complete-source-definition-catalog';
const common=['parity/fixtures/source-definition-catalog.lino','parity/self-translation/source-definition-catalog-rules-native.lino'];
const row={id,required:true,javascript:{status:'implemented',evidence:['js/src/source-definition-catalog.js','js/src/source-definition-catalog-rules.js','js/src/source-definition-cli.js','js/tests/source-definition-catalog.test.js',...common]},rust:{status:'implemented',evidence:['rust/src/source_definition_catalog.rs','rust/src/source_definition_catalog_rules.rs','rust/src/source_definition_cli.rs','rust/tests/source_definition_catalog.rs',...common]}};
const index=value.features.findIndex(row=>row.id===id);if(index<0)value.features.push(row);else value.features[index]=row;
for(const [javascript,rust]of Object.entries({sourceDefinitionCatalog:'source_definition_catalog',sourceDefinitionEntry:'source_definition_entry',createSourceDefinitionProgramLinks:'create_source_definition_program_links',createSourceDefinitionFormalFunctionLinks:'create_source_definition_formal_function_links',runSourceDefinitionCommand:'run_source_definition_command'}))value.javascriptExports[javascript]={rust,kind:'function'};
value.javascriptExports=Object.fromEntries(Object.entries(value.javascriptExports).sort(([a],[b])=>a<b?-1:a>b?1:0));
writeProjectData(file,value);
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity.rs',import.meta.url),renderRustTest(value.javascriptExports));
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity/items.rs',import.meta.url),renderRustItemImports(value.javascriptExports));
const packageFile=new URL('../package.json',import.meta.url),manifest=JSON.parse(readFileSync(packageFile,'utf8'));
manifest.scripts['draft:source-definition-catalog']='node scripts/draft-source-definition-catalog.mjs --draft';
manifest.scripts['check:source-definition-catalog']='node scripts/draft-source-definition-catalog.mjs --check';
if(!manifest.scripts['check:parity'].includes('check:source-definition-catalog'))manifest.scripts['check:parity']+=' && npm run check:source-definition-catalog';
writeFileSync(packageFile,JSON.stringify(manifest,null,2)+'\n');
