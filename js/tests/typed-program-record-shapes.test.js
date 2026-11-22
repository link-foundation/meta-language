// Deferred complete layout, edit, ownership and formal model controls.
import test from 'node:test';
import assert from 'node:assert/strict';
import {assertVersions,recordShapeFixture,observeRecordShape,observeRecordShapePredecessors} from '../../experiments/javascript-properties/record-shape-controls.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {checkProgram,checkSourceLibraryProgram} from '../src/translation/check.js';
import {LinkNetwork} from '../src/network.js';
import {readSuppliedSourceLibraryClosure} from '../src/translation/source-library-closure.js';
import {decodeTypedProgramLinks,encodeTypedProgramLinks} from '../src/typed-program-links.js';
import {encodeTypedProgramLinksWithRecordShapes,encodeTypedProgramLinksWithValueSharing} from '../src/index.js';
assertVersions();
for(const row of recordShapeFixture.programs)test(row.name,()=>observeRecordShape(row));
test('complete preceding source bytes remain available',()=>observeRecordShapePredecessors());
test('explicit sharing entry points refuse empty and unknown qualified models',()=>{
 const program=checkProgram(parseJavaScript(recordShapeFixture.programs[0].source));
 for(const profile of ['',undefined,'unregistered layout 7']){
  assert.throws(()=>encodeTypedProgramLinksWithRecordShapes(program,undefined,profile));
  assert.throws(()=>encodeTypedProgramLinksWithValueSharing(program,undefined,profile));
 }
});

test('direct record layout retains the complete binding graph of captured source modules', () => {
 const sourceClosure = readSuppliedSourceLibraryClosure('entry.js', [
  {path: 'provider.js', source: '/** @param {boolean} flag @returns {boolean} */\nexport function flip(flag) { return !flag; }'},
  {path: 'entry.js', source: 'import { flip } from "./provider.js";\n/** @param {boolean} flag @returns {boolean} */\nexport function apply(flag) { const copied = flag; return flip(copied); }'},
 ], {aliases: true, reExports:'resolved named module providers 1'});
 const {program} = checkSourceLibraryProgram(sourceClosure, {referenceSemantics: true});
 const recordShapes = 'ordered typed program record fields 1';
 const compacted = encodeTypedProgramLinks(program, {recordShapes});
 const direct = encodeTypedProgramLinks(program, {recordShapes, sourceClosure, sourceEntryPath:'entry.js'});
 const reloaded = LinkNetwork.fromLino(direct.network.toLino());
 assert.deepEqual(
  decodeTypedProgramLinks(reloaded, direct.root),
  decodeTypedProgramLinks(compacted.network, compacted.root),
 );
 assert.throws(() => decodeTypedProgramLinks(reloaded, direct.root, {nodeLimit:1}), /resource limit/u);
 const directory = reloaded.fieldRelations().find(relation => relation.parent.equals(direct.root) &&
   relation.label === 'ordinary source archive');
 reloaded.insertFieldRelation(direct.root, directory.label, directory.child);
 assert.throws(() => decodeTypedProgramLinks(reloaded, direct.root), /one bounded source module directory/u);
 assert.throws(() => encodeTypedProgramLinks(program, {recordShapes, sourceClosure: {...sourceClosure}}),
  /original captured module composition/u);
});

test('a closed constructor match retains one effectful default in a captured module', () => {
 const source = `/** @typedef {{$:'selected',value:boolean}|{$:'other'}|{$:'last'}} Choice */
/** @param {Choice} input @returns {boolean} */
export function read(input) {
 if (input.$ === 'selected') return input.value;
 return fallback();
}
/** @returns {boolean} */
function fallback() { console.log('default'); return false; }`;
 const closure = readSuppliedSourceLibraryClosure('entry.js', [{path:'entry.js',source}],
  {aliases:true,reExports:'resolved named module providers 1'});
 const {program} = checkSourceLibraryProgram(closure, {referenceSemantics:true});
 const entry = program.declarations.get('read');
 assert.equal(entry.body.k, 'match');
 assert.deepEqual(entry.body.cases.map(arm => arm.pattern.k), ['ctor', 'wild']);
 assert.equal(entry.body.cases[1].body.fn, 'fallback');
 const archive = encodeTypedProgramLinks(program, {recordShapes:'ordered typed program record fields 1',
  sourceClosure:closure,sourceEntryPath:'entry.js'});
 assert.deepEqual(decodeTypedProgramLinks(LinkNetwork.fromLino(archive.network.toLino()), archive.root), program);
});
