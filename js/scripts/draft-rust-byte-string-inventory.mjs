// Source declarations only; no execution observation is recorded.
import {writeFileSync} from 'node:fs';
import {readProjectData,writeProjectData} from './project-data.mjs';
import {renderRustTest,renderRustItemImports} from './generate-export-parity.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const file=new URL('../../parity/language-features.lino',import.meta.url),value=readProjectData(file),id='rust-byte-string-source-tokens';
const common=['parity/fixtures/rust-byte-string-literals.lino','parity/fixtures/rust-byte-string-assertion-history.lino','parity/self-translation/rust-byte-string-rules-native.lino'];
const row={id,required:true,javascript:{status:'implemented',evidence:['js/src/translation/rust-byte-string-rules.js','js/src/translation/lexer.js','js/src/rust-byte-string-program.js','js/tests/translation-rust-byte-string-literals.test.js',...common]},
 rust:{status:'implemented',evidence:['rust/src/translation/rust_byte_string_rules.rs','rust/src/translation/lexer/byte_strings.rs','rust/src/rust_byte_string_program.rs','rust/tests/rust_byte_string_literals.rs',...common]}};
const position=value.features.findIndex(row=>row.id===id);if(position<0)value.features.push(row);else value.features[position]=row;
value.javascriptExports.createRustByteStringProgramLinks={rust:'create_rust_byte_string_program_links',kind:'function'};
value.javascriptExports=Object.fromEntries(Object.entries(value.javascriptExports).sort(([first],[second])=>first<second?-1:first>second?1:0));
writeProjectData(file,value);
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity.rs',import.meta.url),renderRustTest(value.javascriptExports));
writeFileSync(new URL('../../rust/tests/unit/javascript_export_parity/items.rs',import.meta.url),renderRustItemImports(value.javascriptExports));
const packageFile=new URL('../package.json',import.meta.url),manifest=JSON.parse((await import('node:fs')).readFileSync(packageFile,'utf8'));
manifest.scripts['draft:rust-package-assets']='node scripts/rust-package-source-assets.mjs --draft';
manifest.scripts['check:rust-package-assets']='node scripts/rust-package-source-assets.mjs --check';
if(!manifest.scripts['check:parity'].includes('check:rust-package-assets'))manifest.scripts['check:parity']+=' && npm run check:rust-package-assets';
writeFileSync(packageFile,JSON.stringify(manifest,null,2)+'\n');
