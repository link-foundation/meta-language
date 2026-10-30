// Future compiler-phase controls. Never imported or executed by source authors.
import assert from 'node:assert/strict';
import test from 'node:test';
import {parseAuthoredJavaScriptModule} from '../scripts/source-module-syntax.mjs';
test('source generation parses actual module syntax without resolving or evaluating it',()=>{
 const metadata=parseAuthoredJavaScriptModule('import {missing} from "unresolvable-module";export function actual(){throw new Error("not called");}throw new Error("not evaluated");','authored syntax source');
 assert.equal(metadata.versionRange,'>=22.15.0 <23.0.0 or >=24.0.0 <25.0.0');
 assert.equal(metadata.inputEncoding,'UTF16LE code units');
 assert.equal(metadata.phase,'parse without linking, resolving imports or evaluation');
});
test('source parsing transports original isolated UTF16 code units without UTF8 replacement',()=>{
 assert.equal(parseAuthoredJavaScriptModule('export const actual="'+String.fromCharCode(0xd800)+'";','actual source units').inputEncoding,'UTF16LE code units');
});
test('duplicate actual declarations cannot become alternate translated semantics',()=>{
 assert.throws(()=>parseAuthoredJavaScriptModule('export function actual(){while(false){const name=1;const name=2;}return 0;}','duplicate authored bindings'),/Identifier.*name.*already been declared/su);
});
