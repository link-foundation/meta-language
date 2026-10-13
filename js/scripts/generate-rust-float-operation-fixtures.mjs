// Finite bit oracles are curated independently of the translator and Math.
// NaN is compared by class because this profile does not expose payload bits.
import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { parseRust } from '../src/translation/rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';

const methods = ['abs', 'floor', 'ceil', 'trunc'];
const inputs = [
  ['positive-zero', '0', '0.0f64', ['0000000000000000','0000000000000000','0000000000000000','0000000000000000']],
  ['negative-zero', '-0', '-0.0f64', ['0000000000000000','8000000000000000','8000000000000000','8000000000000000']],
  ['positive-half', '0.5', '0.5f64', ['3fe0000000000000','0000000000000000','3ff0000000000000','0000000000000000']],
  ['negative-half', '-0.5', '-0.5f64', ['3fe0000000000000','bff0000000000000','8000000000000000','8000000000000000']],
  ['positive-one-and-half', '1.5', '1.5f64', ['3ff8000000000000','3ff0000000000000','4000000000000000','3ff0000000000000']],
  ['negative-one-and-half', '-1.5', '-1.5f64', ['3ff8000000000000','c000000000000000','bff0000000000000','bff0000000000000']],
  ['positive-subnormal', '5e-324', '5e-324f64', ['0000000000000001','0000000000000000','3ff0000000000000','0000000000000000']],
  ['negative-subnormal', '-5e-324', '-5e-324f64', ['0000000000000001','bff0000000000000','8000000000000000','8000000000000000']],
  ['maximum-finite', '1.7976931348623157e308', 'f64::MAX', ['7fefffffffffffff','7fefffffffffffff','7fefffffffffffff','7fefffffffffffff']],
  ['negative-maximum-finite', '-1.7976931348623157e308', '-f64::MAX', ['7fefffffffffffff','ffefffffffffffff','ffefffffffffffff','ffefffffffffffff']],
  ['positive-infinity', 'Infinity', 'f64::INFINITY', ['7ff0000000000000','7ff0000000000000','7ff0000000000000','7ff0000000000000']],
  ['negative-infinity', '-Infinity', 'f64::NEG_INFINITY', ['7ff0000000000000','fff0000000000000','fff0000000000000','fff0000000000000']],
  ['nan', 'NaN', 'f64::NAN', ['NaN','NaN','NaN','NaN']],
];
const cases = methods.map((method, index) => ({
  id: `primitive-${method}`, source: `fn apply(value:f64)->f64{value.${method}()} fn main(){}`,
  originalJavaScript: `/** @param {number} value */ function apply(value){return Math.${method}(value);}`,
  observations: inputs.map(([id, javascript, rust, bits]) => ({id, javascriptArguments: [javascript], originalRustArguments: rust, emittedRustArguments: rust, expectedBits: bits[index]})),
}));
for (const [index, method] of methods.entries()) cases.push({
  id: `receiver-call-once-${method}`, source: `fn provider()->f64{-0.5} fn apply()->f64{provider().${method}()} fn main(){}`,
  originalJavaScript: `function provider(){return -0.5;} function apply(){return Math.${method}(provider());}`,
  receiverCalls: 1,
  observations: [{id:'negative-half-call',javascriptArguments:[], originalRustArguments:'', emittedRustArguments:'', expectedBits:inputs[3][3][index]}],
});
cases.push({id:'operation-and-predicate-chain', source:'fn apply(value:f64)->f64{value.abs().floor().ceil().trunc()} fn classify(value:f64)->bool{value.abs().is_finite()} fn main(){}',
  originalJavaScript:'/** @param {number} value */function apply(value){return Math.trunc(Math.ceil(Math.floor(Math.abs(value))));}/** @param {number} value */function classify(value){return Number.isFinite(Math.abs(value));}',
  observations:[{id:'negative-one-and-half-chain',javascriptArguments:['-1.5'], originalRustArguments:'-1.5f64', emittedRustArguments:'-1.5f64', expectedBits:'3ff0000000000000'}],
});
cases.push({id:'typed-negative-zero-binding',source:'fn apply()->f64{let value:f64=-0.0;value.trunc()} fn main(){}',
  originalJavaScript:'function apply(){const value=-0;return Math.trunc(value);}',
  observations:[{id:'negative-zero-binding',javascriptArguments:[], originalRustArguments:'', emittedRustArguments:'',expectedBits:'8000000000000000'}],
});
cases.push({id:'explicit-primitive-literal',source:'fn apply()->f64{1.5f64.floor()} fn main(){}',
  originalJavaScript:'function apply(){return Math.floor(1.5);}',
  observations:[{id:'positive-one-and-half-literal',javascriptArguments:[],originalRustArguments:'',emittedRustArguments:'',expectedBits:'3ff0000000000000'}],
});
cases.push({id:'explicit-immutable-reference-dereference',source:'fn apply(value:&f64)->f64{(*value).ceil()} fn main(){}',
  originalJavaScript:'/** @param {number} value */function apply(value){return Math.ceil(value);}',
  observations:[{id:'negative-half-reference',javascriptArguments:['-0.5'],originalRustArguments:'&(-0.5f64)',emittedRustArguments:'-0.5f64',expectedBits:'8000000000000000'}],
});
cases.push({id:'explicit-negative-zero-literal',source:'fn apply()->f64{(-0.0f64).ceil()} fn main(){}',
  originalJavaScript:'function apply(){return Math.ceil(-0);}',
  observations:[{id:'negative-zero-literal',javascriptArguments:[],originalRustArguments:'',emittedRustArguments:'',expectedBits:'8000000000000000'}],
});
cases.push({id:'primitive-unary-negation',source:'fn apply(value:f64)->f64{(-value).abs()} fn main(){}',
  originalJavaScript:'/** @param {number} value */function apply(value){return Math.abs(-value);}',
  observations:[{id:'negated-negative-half',javascriptArguments:['-0.5'],originalRustArguments:'-0.5f64',emittedRustArguments:'-0.5f64',expectedBits:'3fe0000000000000'}],
});
for (const [id, rustOperation, javascriptOperation, expected] of [
  ['array-length-absolute-value', '.abs()', 'Math.abs(values.length)', ['0000000000000000','3ff0000000000000','4008000000000000']],
  ['array-length-finite-predicate', '.is_finite()', 'Number.isFinite(values.length)', ['true','true','true']],
  ['array-length-operation-chain', '.abs().ceil().trunc()', 'Math.trunc(Math.ceil(Math.abs(values.length)))', ['0000000000000000','3ff0000000000000','4008000000000000']],
]) cases.push({id, source:`fn apply(values:&[f64])->${id==='array-length-finite-predicate'?'bool':'f64'}{(values.len() as f64)${rustOperation}} fn main(){}`,
  originalJavaScript:`/** @param {number[]} values */function apply(values){return ${javascriptOperation};}`,
  observations:[
    {id:'empty',javascriptArguments:[[]],originalRustArguments:'&vec![]',emittedRustArguments:'vec![]',expectedBits:expected[0]},
    {id:'singleton',javascriptArguments:[['1']],originalRustArguments:'&vec![1f64]',emittedRustArguments:'vec![1f64]',expectedBits:expected[1]},
    {id:'three-specials',javascriptArguments:[['NaN','-Infinity','-0']],originalRustArguments:'&vec![f64::NAN,f64::NEG_INFINITY,-0f64]',emittedRustArguments:'vec![f64::NAN,f64::NEG_INFINITY,-0f64]',expectedBits:expected[2]},
  ],
});
const refused = methods.flatMap(method => [
  [`${method}-wrong-arity`, `fn apply(value:f64)->f64{value.${method}(1f64)}`],
  [`${method}-unproved-receiver`, `fn apply()->f64{1.0.${method}()}`],
  [`${method}-indirected-receiver`, `fn apply(value:&f64)->f64{value.${method}()}`],
]);
refused.push(
  ['shadowed-length-f64-owner','mod f64{} fn apply(values:&[u32])->f64{(values.len() as f64).abs()}'],
  ['empty-predicate-is-not-f64','fn apply(values:&[f64])->f64{values.is_empty().abs()}'],
  ['f32-receiver','fn apply(value:f32)->f64{value.abs()}'],
  ['f32-suffix','fn apply()->f64{1.0f32.floor()}'],
  ['fixed-integer-receiver','fn apply(value:i64)->i64{value.abs()}'],
  ['owned-string-receiver','fn apply(value:String)->f64{value.ceil()}'],
  ['custom-receiver','enum Value{V} fn apply(value:Value)->f64{value.trunc()}'],
  ['shadowed-primitive','enum f64{V} fn apply(value:f64)->f64{value.abs()}'],
  ['aliased-custom-primitive','enum Value{V} use crate::Value as f64; fn apply(value:f64)->f64{value.floor()}'],
  ['module-shadowed-primitive','mod f64{} fn apply(value:f64)->f64{value.ceil()}'],
  ['unproved-binding','fn apply()->f64{let value=1.0;value.trunc()}'],
  ['unproved-unary-literal','fn apply()->f64{(-1.0).abs()}'],
  ['indirected-unary-result','fn apply(value:&f64)->f64{(-value).floor()}'],
  ['custom-unary-result','enum Value{V} fn apply(value:Value)->f64{(-value).ceil()}'],
  ['shadowed-unary-result','enum f64{V} fn apply(value:f64)->f64{(-value).trunc()}'],
  ['aliased-unary-result','enum Value{V} use crate::Value as f64; fn apply(value:f64)->f64{(-value).abs()}'],
  ['f32-unary-result','fn apply(value:f32)->f64{(-value).floor()}'],
  ...['round','signum','sin','cos','sqrt'].map(method => [`separate-${method}-model`, `fn apply(value:f64)->f64{value.${method}()}`]),
);
export function rustFloatOperationFixture() {
  return {schemaVersion:1, id:'rust-float-operations', receiverProof:'rust-float-predicates',
    premises:['the existing direct unshadowed primitive binary64 receiver proof','result owner/model evidence is inherited from the admitted receiver','each receiver occurs once in its linked template','non-NaN numbers are observed by exact bits, including signed zero; NaN by class; booleans by exact value','array length expressions explicitly convert to unshadowed primitive f64 under the existing Number length model'],
    cases:cases.map(entry => {
      const surface=parseRust(entry.source), program=checkProgram(surface);
      const original=checkProgram(parseJavaScript(entry.originalJavaScript));
      const rustDefinitions=emitRust(program).definitions.join('\n\n');
      const replay=checkProgram(parseRust(rustDefinitions+'\nfn main(){}'));
      const reverseDefinitions=emitRust(original).definitions.join('\n\n');
      const reverseReplay=checkProgram(parseRust(reverseDefinitions+'\nfn main(){}'));
      return {...entry,surface:JSON.parse(JSON.stringify(surface)),javascript:emitJavaScript(program).text,
        javascriptDefinitions:emitJavaScript(program).definitions.join('\n\n'),rustDefinitions, reverseDefinitions,
        replayJavaScript:emitJavaScript(replay).text, reverseReplayJavaScript:emitJavaScript(reverseReplay).text};
    }),
    invalid:refused.map(([id,body])=>{
      const source=body+' fn main(){}';
      try{checkProgram(parseRust(source));}catch(error){return{id,source,kind:error.kind,reason:error.reason,span:error.span??null};}
      throw new Error('unexpected floating operation admission: '+id);
    }),
    edit:{case:'primitive-floor',from:'floor',to:'ceil',input:'0.5',expectedBits:'3ff0000000000000'},
  };
}
if(process.argv[1]===fileURLToPath(import.meta.url)) {
  const path=new URL('../../parity/fixtures/rust-float-operations.lino',import.meta.url), value=rustFloatOperationFixture();
  if(process.argv.includes('--check')) {if(!checkProjectData(path,value))throw new Error('Rust float operation fixtures are stale');}
  else writeProjectData(path,value);
}
