import {draftSourceRuntime} from './draft-source-runtime.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceRuntime({sourcePath:'js/src/translation/string-number-rules.js',moduleName:'string_number_rules',sourceDependencies:true,
  model:'exact JavaScript StringNumericLiteral binary64 conversion 1',programAsset:'string-number-program',sourceFormatting:'preserve',
  runtimeStem:'string-number',constantPrefix:'STRING_NUMBER',formalNamespace:'MLStringNumber',
  entries:[['convertUnits','convertJavaScriptStringNumberUnits'],['convertInteger','convertJavaScriptBigIntegerNumber'],['convertUtf8','convertJavaScriptUtf8StringNumberUnits']]});
