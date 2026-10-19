import { fileURLToPath } from 'node:url';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { checkProjectData, writeProjectData } from './project-data.mjs';

export function rustReadOnlyQueryFixture() {
  const cases = [
    { id: 'inclusive-binary64-boundaries', source: 'fn test(x:f64)->bool{(0f64..=1f64).contains(&x)}fn main(){println!("{}",test(0f64));println!("{}",test(1f64));println!("{}",test(2f64));println!("{}",test(f64::NAN));println!("{}",test(-0f64));println!("{}",test(f64::INFINITY));}', output: ['true','true','false','false','true','false'] },
    { id: 'inverted-inclusive-range', source: 'fn test(x:f64)->bool{(2f64..=1f64).contains(&x)}fn main(){println!("{}",test(1f64));println!("{}",test(2f64));}', output: ['false','false'] },
    { id: 'nan-range-endpoints', source: 'fn test(x:f64)->bool{(f64::NAN..=1f64).contains(&x)||(0f64..=f64::NAN).contains(&x)}fn main(){println!("{}",test(0f64));println!("{}",test(f64::NAN));}', output: ['false','false'] },
    { id: 'standard-string-readonly-owners', source: 'use std::string::String as Text;fn owned(x:Text)->bool{x.is_empty()}fn slice(x:&str)->bool{x.is_empty()}fn borrowed(x:&String)->bool{x.is_empty()}fn main(){println!("{}",owned(Text::new()));println!("{}",slice(""));println!("{}",slice("🦀"));println!("{}",borrowed(&String::new()));}', output: ['true','true','false','true'] },
    { id: 'range-once-evaluation-and-hygiene', source: 'fn lower()->f64{0f64}fn upper()->f64{1f64}fn argument()->f64{0.5f64}fn test(ml_tuple_value0:bool)->bool{ml_tuple_value0&&(lower()..=upper()).contains(&argument())}fn main(){println!("{}",test(true));}', output: ['true'] },
  ];
  const invalid = [
    ['unborrowed-range-item','fn test(x:f64)->bool{(0f64..=1f64).contains(x)}fn main(){}'],
    ['multiply-borrowed-range-item','fn test(x:f64)->bool{(0f64..=1f64).contains(&&x)}fn main(){}'],
    ['borrowed-range-endpoint','fn test(x:f64)->bool{(&0f64..=1f64).contains(&x)}fn main(){}'],
    ['boxed-range-endpoint','fn test(x:f64)->bool{(Box::new(0f64)..=1f64).contains(&x)}fn main(){}'],
    ['fixed-integer-range','fn test(x:i32)->bool{(0i32..=1i32).contains(&x)}fn main(){}'],
    ['ambiguous-unsuffixed-range','fn test(x:f64)->bool{(0..=1).contains(&x)}fn main(){}'],
    ['custom-primitive-owner','enum f64{Value}fn test(x:f64)->bool{(0f64..=1f64).contains(&x)}fn main(){}'],
    ['stored-range-value','fn test()->bool{let range=(0f64..=1f64);true}fn main(){}'],
    ['unrelated-range-method','fn test()->bool{(0f64..=1f64).is_empty()}fn main(){}'],
    ['range-extra-argument','fn test(x:f64)->bool{(0f64..=1f64).contains(&x,&x)}fn main(){}'],
    ['mutable-range-argument','fn test(x:f64)->bool{(0f64..=1f64).contains(&mut x)}fn main(){}'],
    ['custom-string-owner','enum String{Empty}fn test(x:String)->bool{x.is_empty()}fn main(){}'],
    ['aliased-custom-string-owner','enum Fake{Empty}use crate::Fake as String;fn test(x:String)->bool{x.is_empty()}fn main(){}'],
    ['boxed-string-receiver','fn test(x:Box<String>)->bool{x.is_empty()}fn main(){}'],
    ['mutable-string-receiver','fn test(x:&mut String)->bool{x.is_empty()}fn main(){}'],
    ['string-extra-argument','fn test(x:&str)->bool{x.is_empty(true)}fn main(){}'],
    ['constructor-extra-argument','fn test()->bool{String::new(true).is_empty()}fn main(){}'],
    ['unknown-string-capture','fn test(x:f64)->bool{x.is_empty()}fn main(){}'],
  ].map(([id,source]) => ({id,source}));
  return { version: 1, cases: cases.map(entry => {
    const surface = parseRust(entry.source), checked = checkProgram(surface);
    return { ...entry, surface: JSON.parse(JSON.stringify(surface)), javascript: emitJavaScript(checked).text, rust: emitRust(checked).text };
  }), invalid: invalid.map(entry => {
    try { checkProgram(parseRust(entry.source)); } catch (error) {
      return { ...entry, kind: error.kind, reason: error.reason, span: error.span ?? null };
    }
    throw new Error(`${entry.id} unexpectedly entered the readonly query model`);
  }) };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const file = new URL('../../parity/fixtures/rust-read-only-queries.lino', import.meta.url), fixture = rustReadOnlyQueryFixture();
  if (process.argv.includes('--check')) {
    if (!checkProjectData(file, fixture)) throw new Error('the readonly query fixture is stale');
  } else writeProjectData(file, fixture);
  console.log(`Rust readonly queries: ${fixture.cases.length} source cases, ${fixture.invalid.length} explicit refusals`);
}
