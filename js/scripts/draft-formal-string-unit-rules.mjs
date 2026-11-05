import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/formal-string-unit-rules.js',moduleName:'formal_string_unit_rules',sourceDependencies:true,
 model:'formal target UTF16 unit representation and transport 1',programAsset:'formal-string-unit-program'});
