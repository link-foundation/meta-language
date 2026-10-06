// Prints the shape of program.main for a module with and without statements.
import { checkProgram } from '../js/src/translation/check.js';
import { parseJavaScript } from '../js/src/translation/javascript.js';
import { parseRust } from '../js/src/translation/rust.js';
for (const text of ['/** @param {number} a @returns {number} */\nexport function add(a) { return a; }', 'console.log(1);']) {
  try { const p = checkProgram(parseJavaScript(text)); console.log(JSON.stringify(p.main)?.slice(0, 200)); } catch (e) { console.log('err', e.kind, e.message); }
}
for (const text of ['/// doc\npub fn add(a: f64) -> f64 { a }', 'fn main() { println!("1"); }']) {
  try { const p = checkProgram(parseRust(text)); console.log(JSON.stringify(p.main)?.slice(0, 200)); } catch (e) { console.log('err', e.kind, e.message); }
}
