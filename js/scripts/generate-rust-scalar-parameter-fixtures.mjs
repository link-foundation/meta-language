import { fileURLToPath } from 'node:url';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';

const sources = [
  ['binary64-value', 'fn advance(mut index:f64)->f64{index+1.0}', 'fn main(){println!("{}",advance(2.0)==3.0);}', ['true'], true],
  ['boolean-value', 'fn invert(mut flag:bool)->bool{!flag}', 'fn main(){println!("{}",invert(false));println!("{}",invert(true));}', ['true', 'false'], true],
  ['fixed-width-value', 'fn keep(mut value:i32)->i32{value}', 'fn main(){println!("{}",keep(-2));println!("{}",keep(4));}', ['-2', '4'], true],
  ['two-unwritten-parameters', 'fn choose(mut flag:bool,mut left:i32,right:i32)->i32{if flag{left}else{right}}', 'fn main(){println!("{}",choose(true,3,7));println!("{}",choose(false,3,7));}', ['3', '7'], false],
  ['opaque-comment-and-string', 'fn same(mut value:i32)->bool{/* nested /* = } */ comment */ "="=="=" && value==value}', 'fn main(){println!("{}",same(2));}', ['true'], false],
  ['recursive-copy-call', 'fn choose(mut flag:bool)->bool{if flag{choose(false)}else{true}}', 'fn main(){println!("{}",choose(true));}', ['true'], false],
  ['match-arms', 'fn invert(mut flag:bool)->bool{match flag{true=>false,false=>true}}', 'fn main(){println!("{}",invert(false));println!("{}",invert(true));}', ['true', 'false'], false],
];
const writes = ['=', '+=', '-=', '*=', '/=', '%=', '&=', '|=', '^=', '<<=', '>>='];
const refused = [
  ...writes.map(operator => [`assignment-${operator}`, `fn changed(mut value:i32)->i32{value${operator}1;value}`]),
  ['nested-assignment', 'fn changed(mut value:i32,flag:bool)->i32{if flag{value=1;}value}'],
  ['unproved-local-initializer', 'fn changed(mut value:i32)->i32{let other=value;other}'],
  ['borrowed-scalar', 'fn borrowed(mut value:&f64)->f64{*value}'],
  ['boxed-scalar', 'fn boxed(mut value:Box<i32>)->i32{*value}'],
  ['owned-string', 'fn text(mut value:String)->String{value}'],
  ['tuple-product', 'fn product(mut value:(i32,bool))->i32{value.0}'],
  ['nominal-value', 'enum Value{Item} fn item(mut value:Value)->Value{value}'],
  ['shadowed-primitive-before', 'enum f64{Item} fn item(mut value:f64)->f64{value}'],
  ['shadowed-primitive-after', 'fn item(mut value:f64)->f64{value} enum f64{Item}'],
];

export function rustScalarParameterFixture() {
  return {
    schemaVersion: 1, id: 'rust-unwritten-scalar-parameters',
    premises: ['the parameter has an unshadowed direct scalar Copy source origin', 'the complete balanced function body contains no assignment punctuation', 'the body still passes the ordinary closed portable-core parser and source-origin checks', 'the declaration keyword changes permission only; actual mutation, references and unproved value kinds remain refused'],
    cases: sources.map(([id, definition, main, expectedOutput, nativeOracle]) => {
      const source = definition + ' ' + main;
      const surface = parseRust(source), checked = checkProgram(surface);
      const rust = emitRust(checked);
      const definitions = rust.definitions.join('\n\n');
      const replay = checkProgram(parseRust(definitions + ' ' + main));
      return { id, source, expectedOutput, nativeOracle, surface: JSON.parse(JSON.stringify(surface)), javascript: emitJavaScript(checked).text, definitions, replayJavaScript: emitJavaScript(replay).text };
    }),
    invalid: refused.map(([id, definition]) => {
      const source = definition + ' fn main(){}';
      const start = source.indexOf('mut');
      try { checkProgram(parseRust(source)); } catch (error) {
        if (error.kind !== 'unsupported' || error.reason !== 'mutable parameter: mutation is outside the portable core' || error.span?.start !== start || error.span?.end !== start + 3) throw error;
        return { id, source, kind: error.kind, reason: error.reason, span: error.span };
      }
      throw new Error(`unproved mutable parameter unexpectedly admitted: ${id}`);
    }),
  };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const file = new URL('../../parity/fixtures/rust-scalar-parameters.lino', import.meta.url);
  const value = rustScalarParameterFixture();
  if (process.argv.includes('--check')) checkProjectData(file, value);
  else writeProjectData(file, value);
}
