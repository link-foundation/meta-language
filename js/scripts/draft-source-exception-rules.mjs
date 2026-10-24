import {draftSourceLibrary} from './draft-source-library.mjs';
import {writeFileSync} from 'node:fs';
import {SOURCE_EXCEPTION_PRELUDE} from '../src/translation/source-exception-runtime.js';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceLibrary({sourcePath:'js/src/translation/source-exception-rules.js',moduleName:'source_exception_rules',
  model:'JavaScript exception class and message intrinsic operations 1',programAsset:'source-exception-program'});
writeFileSync(new URL('../../rust/src/translation/runtime/source-exception.rs',import.meta.url),SOURCE_EXCEPTION_PRELUDE+'\n');
