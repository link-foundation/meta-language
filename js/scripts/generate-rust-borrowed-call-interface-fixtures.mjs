import { isDeepStrictEqual } from 'node:util';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from './project-data.mjs';
import { createTypedProgramLinks } from '../src/typed-program-links.js';

export const RUST_BORROWED_CALL_INTERFACE_MODEL = 'closed primitive borrowed Rust call interface 1';
const capture = { rustSourceSignatureModel: 'Rust source function signature 1' };
const specifications = [
  ['add-one', 'fn leaf(value:&f64)->f64{*value+1.0}fn caller(value:f64)->f64{leaf(&value)}', ['pub fn leaf(value: &f64) -> f64', 'pub fn caller(value: f64) -> f64'], ['let ml_borrowed_argument = value;'], 'crate::leaf(&(ml_borrowed_argument))', [[3], ['-0'], ['NaN'], ['Infinity']], [4, 1, 'NaN', 'Infinity']],
  ['boolean', 'fn leaf(value:&bool)->bool{!*value}fn caller(value:bool)->bool{leaf(&value)}', ['pub fn leaf(value: &bool) -> bool', 'pub fn caller(value: bool) -> bool'], ['let ml_borrowed_argument = value;'], 'crate::leaf(&(ml_borrowed_argument))', [[true], [false]], [false, true]],
  ['ordered-mixed', 'fn leaf(flag:&bool,value:&f64)->f64{if *flag{*value}else{-*value}}fn caller(flag:bool,value:f64)->f64{leaf(&flag,&(value+1.0))}', ['pub fn leaf(flag: &bool, value: &f64) -> f64', 'pub fn caller(flag: bool, value: f64) -> f64'], ['let ml_borrowed_argument = flag;', 'let ml_borrowed_argument_2 = (value + 1f64);'], 'crate::leaf(&(ml_borrowed_argument), &(ml_borrowed_argument_2))', [[true, 2], [false, 2]], [3, -3]],
  ['borrowed-caller', 'fn leaf(value:&f64)->f64{*value}fn caller(value:&f64)->f64{leaf(value)}', ['pub fn leaf(value: &f64) -> f64', 'pub fn caller(value: &f64) -> f64'], ['let ml_borrowed_argument = value;'], 'crate::leaf(&(ml_borrowed_argument))', [[1.25], ['-0'], ['NaN']], [1.25, '-0', 'NaN']],
  ['named-lifetimes', "fn leaf<'a>(value:&'a f64)->f64{*value}fn caller<'b>(value:&'b f64)->f64{leaf(value)}", ["pub fn leaf<'a>(value: &'a f64) -> f64", "pub fn caller<'b>(value: &'b f64) -> f64"], ['let ml_borrowed_argument = value;'], 'crate::leaf(&(ml_borrowed_argument))', [[1.25], ['-0']], [1.25, '-0']],
  ['nested-elided', 'fn leaf(value:& &f64)->f64{**value}fn caller(value:f64)->f64{leaf(& &value)}', ['pub fn leaf(value: & &f64) -> f64', 'pub fn caller(value: f64) -> f64'], ['let ml_borrowed_argument = value;'], 'crate::leaf(&(&(ml_borrowed_argument)))', [[1.25], ['-0']], [1.25, '-0']],
  ['nested-named', "fn leaf<'outer,'inner>(value:&'outer &'inner f64)->f64{**value}fn caller<'value>(value:&'value f64)->f64{leaf(&value)}", ["pub fn leaf<'outer, 'inner>(value: &'outer &'inner f64) -> f64", "pub fn caller<'value>(value: &'value f64) -> f64"], ['let ml_borrowed_argument = value;'], 'crate::leaf(&(&(ml_borrowed_argument)))', [[1.25]], [1.25]],
  ['mixed-reference-depth', 'fn leaf(first:f64,second:&f64)->f64{first-*second}fn caller(first:f64,second:f64)->f64{leaf(first,&second)}', ['pub fn leaf(first: f64, second: &f64) -> f64', 'pub fn caller(first: f64, second: f64) -> f64'], ['let ml_borrowed_argument = first;', 'let ml_borrowed_argument_2 = second;'], 'crate::leaf(ml_borrowed_argument, &(ml_borrowed_argument_2))', [[7, 3], [3, 7]], [4, -4]],
  ['parameter-hygiene', 'fn leaf(first:&f64,second:&f64)->f64{*first-*second}fn caller(ml_borrowed_argument:f64,ml_borrowed_argument_2:f64)->f64{leaf(&ml_borrowed_argument,&ml_borrowed_argument_2)}', ['pub fn leaf(first: &f64, second: &f64) -> f64', 'pub fn caller(ml_borrowed_argument: f64, ml_borrowed_argument_2: f64) -> f64'], ['let ml_borrowed_argument_3 = ml_borrowed_argument;', 'let ml_borrowed_argument_4 = ml_borrowed_argument_2;'], 'crate::leaf(&(ml_borrowed_argument_3), &(ml_borrowed_argument_4))', [[7, 3]], [4]],
  ['argument-local-hygiene', 'fn leaf(value:&f64)->f64{*value}fn caller(value:f64)->f64{leaf(&{let ml_borrowed_argument=value+1.0;ml_borrowed_argument})}', ['pub fn leaf(value: &f64) -> f64', 'pub fn caller(value: f64) -> f64'], ['let ml_borrowed_argument_2 = {'], 'crate::leaf(&(ml_borrowed_argument_2))', [[3]], [4]],
  ['conditional-argument', 'fn leaf(value:&f64)->f64{*value}fn caller(flag:bool,value:f64)->f64{leaf(&if flag{value}else{-value})}', ['pub fn leaf(value: &f64) -> f64', 'pub fn caller(flag: bool, value: f64) -> f64'], ['let ml_borrowed_argument = if flag {'], 'crate::leaf(&(ml_borrowed_argument))', [[true, 2], [false, 2]], [2, -2]],
  ['zero-arguments', 'fn leaf()->f64{1.0}fn caller()->f64{leaf()}', ['pub fn leaf() -> f64', 'pub fn caller() -> f64'], [], 'crate::leaf()', [[]], [1]],
];
const refusals = [
  ['static-target', "fn leaf(value:&'static f64)->f64{*value}fn caller(value:&'static f64)->f64{leaf(value)}fn main(){}"],
  ['borrowed-result', 'fn leaf(value:&f64)->&f64{value}fn caller(value:&f64)->&f64{leaf(value)}fn main(){}'],
  ['boxed-target', 'fn leaf(value:Box<f64>)->f64{*value}fn caller(value:Box<f64>)->f64{leaf(value)}fn main(){}'],
  ['main-effect', 'fn leaf(value:&f64)->f64{*value}fn caller(value:f64)->f64{leaf(&value)}fn main(){println!("{}",true);}'],
  ['leaf-effect', 'fn leaf(value:&f64)->f64{if *value==0.0{panic!("failure")}else{*value}}fn caller(value:f64)->f64{leaf(&value)}fn main(){}'],
  ['call-chain', 'fn leaf(value:&f64)->f64{*value}fn caller(value:f64)->f64{leaf(&value)}fn relay(value:f64)->f64{caller(value)}fn main(){}'],
  ['recursion', 'fn leaf(value:f64)->f64{leaf(value)}fn caller(value:f64)->f64{leaf(value)}fn main(){}'],
  ['nested-argument-call', 'fn leaf(value:&f64)->f64{*value}fn other(value:f64)->f64{value}fn caller(value:f64)->f64{leaf(&other(value))}fn main(){}'],
  ['nested-body-call', 'fn leaf(value:f64)->f64{value}fn other(value:f64)->f64{if value>0.0{leaf(value)}else{-value}}fn caller(value:f64)->f64{leaf(value)}fn main(){}'],
  ['no-caller', 'fn leaf(value:&f64)->f64{*value}fn main(){}'],
  ['non-flat', 'mod scope{pub fn leaf(value:&f64)->f64{*value}pub fn caller(value:f64)->f64{leaf(&value)}}fn main(){}'],
  ['machine-owner', 'fn leaf(value:&u32)->u32{*value}fn caller(value:u32)->u32{leaf(&value)}fn main(){}'],
];

export function rustBorrowedCallInterfaceFixture() {
  const archive = source => { const { network, root } = createTypedProgramLinks(source, 'Rust', capture); return { links: network.toLino(), root: root.asU64() }; };
  return {
    schemaVersion: 1, model: RUST_BORROWED_CALL_INTERFACE_MODEL, sourceSignatureModel: capture.rustSourceSignatureModel,
    scope: 'Actual flat primitive leaf/direct caller programs; retained owned signatures, ordered pure argument copies and local borrows. No call chains, recursion, effects, imports, borrowed results, custom owners, pointer observations, heap, resource or general ABI equivalence.',
    cases: specifications.map(([id, definition, headers, bindings, invocation, inputs, expected]) => {
      const source = `${definition}\nfn main(){}`;
      return { id, source, headers, bindings, invocation, caller: 'caller', inputs, expected, ...archive(source) };
    }),
    refusals: refusals.map(([id, source]) => ({ id, source, ...archive(source) })),
  };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const destination = new URL('../../parity/fixtures/rust-borrowed-call-interface.lino', import.meta.url);
  const generated = rustBorrowedCallInterfaceFixture();
  if (process.argv.includes('--check')) {
    if (!isDeepStrictEqual(readProjectData(destination), generated)) throw new Error('borrowed call interface fixtures are stale');
  } else writeProjectData(destination, generated);
  console.log(`Rust borrowed call interface: ${generated.cases.length} actual programs and ${generated.refusals.length} source refusals`);
}
