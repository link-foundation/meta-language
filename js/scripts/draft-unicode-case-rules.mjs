import {draftSourceRuntime} from './draft-source-runtime.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceRuntime({sourcePath:'js/src/translation/unicode-case-rules.js',moduleName:'unicode_case_rules',sourceDependencies:true,
 model:'Unicode 16 full default UTF16 case conversion 1',programAsset:'unicode-case-program',runtimeStem:'unicode-case',constantPrefix:'UNICODE_CASE',formalNamespace:'MLUnicodeCase',entries:[['mapCase','mapUnicodeCaseUtf16Text']]});
