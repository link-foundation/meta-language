import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';

const positives = [
  ['named-text-input', `fn equal<'a>(text:&'a str)->bool{text=="Rust"} fn main(){println!("{}",equal("Rust"));}`, ['true']],
  ['independent-input-lifetimes', `fn same<'left,'right>(left:&'left str,right:&'right str)->bool{left==right} fn main(){println!("{}",same("meta","meta"));}`, ['true']],
  ['borrowed-scalar-input', `fn read<'value>(value:&'value u64)->u64{*value} fn main(){let value=7u64;println!("{}",read(&value));}`, ['7']],
  ['nested-input-lifetimes', `fn read<'outer,'inner,>(value:&'outer &'inner u64)->u64{**value} fn main(){let value=11u64;println!("{}",read(&&value));}`, ['11']],
  ['owned-result-from-borrow', `fn owned<'text>(text:&'text str)->String{text.to_string()} fn main(){println!("{}",owned("language"));}`, ['language']],
  ['reused-names-are-distinct-scopes', `fn first<'a>(value:&'a u64)->u64{*value} fn second<'a>(value:&'a u64)->u64{first(value)+1} fn main(){let value=13u64;println!("{}",second(&value));}`, ['14']],
  ['module-local-lifetime-scope', `mod values{pub fn read<'a>(value:&'a u64)->u64{*value}} use crate::values::read; fn main(){let value=17u64;println!("{}",read(&value));}`, ['17']],
  ['explicit-placeholder-input', `fn read(value:&'_ u64)->u64{*value} fn main(){let value=19u64;println!("{}",read(&value));}`, ['19']],
];
const main = ' fn main(){println!("x");}';
const negatives = [
  ['undeclared-input-lifetime', `fn read(value:&'a u64)->u64{*value}`],
  ['previous-function-scope-does-not-leak', `fn first<'a>(value:&'a u64)->u64{*value} fn second(value:&'a u64)->u64{*value}`],
  ['module-scope-does-not-leak', `mod values{pub fn first<'a>(value:&'a u64)->u64{*value}} fn second(value:&'a u64)->u64{*value}`],
  ['duplicate-lifetime-binding', `fn read<'a,'a>(value:&'a u64)->u64{*value}`],
  ['static-is-not-a-binding', `fn read<'static>(value:&'static u64)->u64{*value}`],
  ['placeholder-is-not-a-binding', `fn read<'_>(value:&'_ u64)->u64{*value}`],
  ['keyword-is-not-a-binding', `fn read<'fn>(value:&'fn u64)->u64{*value}`],
  ['bounded-lifetime', `fn read<'a:'static>(value:&'a u64)->u64{*value}`],
  ['where-lifetime-bound', `fn read<'a>(value:&'a u64)->u64 where 'a:'static{*value}`],
  ['type-parameter', `fn read<T>(value:u64)->u64{value}`],
  ['mixed-type-parameter', `fn read<'a,T>(value:&'a u64)->u64{*value}`],
  ['constant-parameter', `fn read<const N:usize>(value:u64)->u64{value}`],
  ['mutable-input-reference', `fn read<'a>(value:&'a mut u64)->u64{*value}`],
  ['body-region-annotation', `fn read<'a>(value:&'a u64)->u64{let other:&'a u64=value;*other}`],
  ['returned-named-reference', `fn read<'a>(value:&'a u64)->&'a u64{value}`],
  ['returned-elided-reference-with-binder', `fn read<'a>(value:&'a u64)->&u64{value}`],
  ['incompatible-return-lifetimes', `fn read<'a,'b>(left:&'a u64,right:&'b u64)->&'a u64{right}`],
  ['unproven-reference-call', `fn read<'a>(value:&'a u64)->u64{*value} fn other()->u64{read(1)}`],
];

export function rustFunctionLifetimeFixture() {
  const cases = positives.map(([id, source, output]) => {
    const surface = parseRust(source), program = checkProgram(surface);
    return { id, source, output, surface: JSON.parse(JSON.stringify(surface)), javascript: emitJavaScript(program).text, rust: emitRust(program).text };
  });
  const invalid = negatives.map(([id, body]) => {
    const source = body + main;
    try { checkProgram(parseRust(source)); } catch (error) {
      return { id, source, kind: error.kind, reason: error.reason, span: error.span ?? null };
    }
    throw new Error(`source admission unexpectedly accepted ${id}`);
  });
  return {
    schemaVersion: 1, id: 'rust-function-lifetimes',
    sourceModel: 'immutable Rust value projection',
    premises: ['immutable reference origins are checked by the existing source-origin rules', 'lifetime binders are scoped to their declaring function', 'reference results with lifetime binders require a separate return-lifetime proof'],
    cases, invalid,
  };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const file = new URL('../../parity/fixtures/rust-function-lifetimes.lino', import.meta.url);
  const value = rustFunctionLifetimeFixture();
  if (process.argv.includes('--check')) {
    if (!checkProjectData(file, value)) throw new Error('Rust function lifetime fixtures are stale');
  } else writeProjectData(file, value);
}
