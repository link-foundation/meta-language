import { isDeepStrictEqual } from 'node:util';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from './project-data.mjs';
import { createTypedProgramLinks } from '../src/typed-program-links.js';

export const SOURCE_HEADER_EMISSION_MODEL = 'primitive borrowed Rust function header 1';
const capture = { rustSourceSignatureModel: 'Rust source function signature 1' };
const specifications = [
  ['direct-float', 'fn keep(value:f64)->f64{value}', 'pub fn keep(value: f64) -> f64', [], 'keep', [[0], [1.25], ['-0'], ['NaN'], ['Infinity']], [0, 1.25, '-0', 'NaN', 'Infinity']],
  ['borrowed-float', 'fn keep(value:&f64)->f64{*value}', 'pub fn keep(value: &f64) -> f64', ['let value = *value;'], 'keep', [[0], [1.25], ['-0'], ['NaN'], ['Infinity']], [0, 1.25, '-0', 'NaN', 'Infinity']],
  ['direct-boolean', 'fn flip(value:bool)->bool{!value}', 'pub fn flip(value: bool) -> bool', [], 'flip', [[true], [false]], [false, true]],
  ['borrowed-boolean', 'fn flip(value:&bool)->bool{!*value}', 'pub fn flip(value: &bool) -> bool', ['let value = *value;'], 'flip', [[true], [false]], [false, true]],
  ['named-lifetime', "fn keep<'a>(value:&'a f64)->f64{*value}", "pub fn keep<'a>(value: &'a f64) -> f64", ['let value = *value;'], 'keep', [[1.25], ['-0']], [1.25, '-0']],
  ['ordered-nested-lifetimes', "fn keep<'outer,'inner>(value:&'outer &'inner f64)->f64{**value}", "pub fn keep<'outer, 'inner>(value: &'outer &'inner f64) -> f64", ['let value = **value;'], 'keep', [[1.25], ['-0']], [1.25, '-0']],
  ['static-lifetime', "fn keep(value:&'static f64)->f64{*value}", "pub fn keep(value: &'static f64) -> f64", ['let value = *value;'], 'keep', [[1.25]], [1.25]],
  ['placeholder-lifetime', "fn keep(value:&'_ f64)->f64{*value}", "pub fn keep(value: &'_ f64) -> f64", ['let value = *value;'], 'keep', [[1.25]], [1.25]],
  ['mixed-ordered-parameters', "fn choose<'a,'b>(flag:&'a bool,value:&'b f64)->f64{if *flag{*value}else{-*value}}", "pub fn choose<'a, 'b>(flag: &'a bool, value: &'b f64) -> f64", ['let flag = *flag;', 'let value = *value;'], 'choose', [[true, 2], [false, 2], [true, '-0']], [2, -2, '-0']],
  ['target-reserved-name', 'fn read(std:&f64)->f64{let value=*std;value+1.0}', 'pub fn read(std_: &f64) -> f64', ['let std_ = *std_;'], 'read', [[2], [0]], [3, 1]],
  ['local-shadowing', 'fn read(value:&f64)->f64{let value=*value+1.0;value*2.0}', 'pub fn read(value: &f64) -> f64', ['let value = *value;'], 'read', [[2], [0]], [6, 2]],
  ['zero-parameters', 'fn one()->f64{1.0}', 'pub fn one() -> f64', [], 'one', [[]], [1]],
];

export function rustSourceHeaderEmissionFixture() {
  return {
    schemaVersion: 1, model: SOURCE_HEADER_EMISSION_MODEL, sourceSignatureModel: capture.rustSourceSignatureModel,
    scope: 'Primitive leaf reference headers and checked value copies; no callers, effects, allocations, borrowed results, pointer observations, resource or general ABI equivalence.',
    cases: specifications.map(([id, definition, header, copies, functionName, inputs, expected]) => {
      const source = `${definition}\nfn main(){}`;
      const { network, root } = createTypedProgramLinks(source, 'Rust', capture);
      return { id, source, header, copies, functionName, inputs, expected, links: network.toLino(), root: root.asU64() };
    }),
    refusals: [
      ['call', 'fn base(x:f64)->f64{x}fn use_base(x:&f64)->f64{base(*x)}fn main(){}'],
      ['borrowed-result', 'fn keep(x:&f64)->&f64{x}fn main(){}'],
      ['boxed-parameter', 'fn keep(x:Box<f64>)->f64{*x}fn main(){}'],
      ['string-owner', 'fn empty(x:&str)->bool{x==""}fn main(){}'],
      ['effect', 'fn keep(x:&f64)->f64{*x}fn main(){println!("{}",true);}'],
      ['abort', 'fn keep(x:&f64)->f64{if *x==0.0{panic!("message")}else{*x}}fn main(){}'],
      ['non-flat', 'mod scope{pub fn keep(x:&f64)->f64{*x}}fn main(){}'],
      ['machine-integer', 'fn keep(x:&u32)->u32{*x}fn main(){}'],
    ].map(([id, source]) => {
      const { network, root } = createTypedProgramLinks(source, 'Rust', capture);
      return { id, source, links: network.toLino(), root: root.asU64() };
    }),
  };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const destination = new URL('../../parity/fixtures/rust-source-header-emission.lino', import.meta.url);
  const generated = rustSourceHeaderEmissionFixture();
  if (process.argv.includes('--check')) {
    if (!isDeepStrictEqual(readProjectData(destination), generated)) throw new Error('source header emission fixtures are stale');
  } else writeProjectData(destination, generated);
  console.log(`Rust source header emission: ${generated.cases.length} independent headers and ${generated.refusals.length} source refusals`);
}
