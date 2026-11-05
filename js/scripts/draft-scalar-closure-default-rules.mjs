import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/scalar-closure-default-rules.js',moduleName:'scalar_closure_default_rules',sourceDependencies:true,model:'ordinary closure defaults through ordered existing parameter entries 1',programAsset:'scalar-closure-default-program'});
