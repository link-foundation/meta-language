// Deferred controls; no product or fixture execution during draft authoring.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {parseJavaScript} from '../src/translation/javascript.js';
import {findJavaScriptLexicalDeclarationConflict} from '../src/translation/lexical-declaration-rules.js';
const fixture=readProjectData(new URL('../../parity/fixtures/lexical-declarations.lino',import.meta.url));
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; '+fixture.versions.JavaScript);
for(const row of fixture.cases)test(row.name+' remains a valid lexical scope',()=>assert.doesNotThrow(()=>parseJavaScript(row.source)));
for(const row of fixture.refusals)test(row.name+' is an early error',()=>assert.throws(()=>parseJavaScript(row.source),error=>error.kind===row.kind&&error.reason===row.reason&&error.span?.end>error.span?.start));
test('actual declaration columns distinguish immediate body and nested scopes',()=>{
 const parents=[-1,0,1,2],boundaries=[0,2,0,0];
 assert.deepEqual(findJavaScriptLexicalDeclarationConflict(parents,boundaries,[1],['value'],[2],['value'],[1]),[0,1]);
 assert.deepEqual(findJavaScriptLexicalDeclarationConflict(parents,boundaries,[1],['value'],[3],['value'],[1]),[]);
 assert.deepEqual(findJavaScriptLexicalDeclarationConflict(parents,boundaries,[],[],[2,2],['value','value'],[1,1]),[1,0]);
 assert.deepEqual(findJavaScriptLexicalDeclarationConflict(parents,boundaries,[],[],[0,0],['f','f'],[0,0]),[]);
 assert.deepEqual(findJavaScriptLexicalDeclarationConflict(parents,boundaries,[],[],[0,0],['f','f'],[0,1]),[1,0]);
 assert.throws(()=>findJavaScriptLexicalDeclarationConflict(parents,[],[],[],[],[],[]),RangeError);
});
