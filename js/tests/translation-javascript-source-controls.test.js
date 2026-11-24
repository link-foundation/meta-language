// Deferred original source, compiler and full machine-state assertions.
import test from 'node:test';import assert from 'node:assert/strict';
import {compileJavaScriptCallSource} from '../src/translation/javascript-call-source.js';
import {assertVersions} from '../../experiments/javascript-properties/controls.mjs';
import {observeCallSourceControl} from '../../experiments/javascript-properties/call-source-control-controls.mjs';
import {javascriptSourceControlCases,javascriptSourceControlRefusals} from './fixtures/javascript-source-controls.mjs';
assertVersions();
for(const row of javascriptSourceControlCases)test('deferred source controls: '+row.name,()=>observeCallSourceControl(row));
for(const row of javascriptSourceControlRefusals)test('deferred source control refusal: '+row.name,()=>{const actual=compileJavaScriptCallSource(row.expression,[]);assert.notEqual(actual.error,'');assert.equal(actual.error,row.reason);assert.deepEqual(actual.instructions,[]);assert.equal(actual.registers,0);});
