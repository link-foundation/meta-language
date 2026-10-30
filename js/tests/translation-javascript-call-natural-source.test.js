import test from 'node:test';
import {assertVersions,observeNaturalSource} from '../../experiments/javascript-properties/call-natural-source-controls.mjs';
assertVersions();
test('every declared shared natural rule, source builder and compiler control',()=>{observeNaturalSource();});
