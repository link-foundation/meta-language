import {draftSourceLibrary} from './draft-source-library.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/unsigned-link-identity-rules.js',moduleName:'unsigned_link_identity_rules',directory:'',model:'complete unsigned 64-bit link identity and reversible binary envelope 1',programAsset:'unsigned-link-identity-program'});
