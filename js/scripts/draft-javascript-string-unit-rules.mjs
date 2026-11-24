import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/javascript-string-unit-rules.js',moduleName:'javascript_string_unit_rules',sourceDependencies:true,
  model:'JavaScript full UTF16 literal escape and unit construction 1',programAsset:'javascript-string-unit-program'});
