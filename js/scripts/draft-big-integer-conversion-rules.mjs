import {draftSourceRuntime} from './draft-source-runtime.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
draftSourceRuntime({sourcePath:'js/src/translation/big-integer-conversion-rules.js',moduleName:'big_integer_conversion_rules',
  sourceDependencies:true,model:'exact JavaScript BigInt primitive conversion 1',programAsset:'big-integer-conversion-program',
  runtimeStem:'big-integer-conversion',constantPrefix:'BIG_INTEGER_CONVERSION',formalNamespace:'MLBigIntegerConversion',
  entries:[['convertUnits','convertJavaScriptStringBigIntegerUnits'],['convertNumber','convertJavaScriptNumberBigInteger'],['convertBoolean','convertJavaScriptBooleanBigInteger'],['convertUtf8','convertJavaScriptUtf8BigInteger']]});
