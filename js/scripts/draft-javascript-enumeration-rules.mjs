import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/javascript-enumeration-rules.js',moduleName:'javascript_enumeration_rules',sourceDependencies:true,model:'actual source object enumeration and ordinary order library operand roles 1'});
