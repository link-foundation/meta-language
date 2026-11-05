// CI-only reverse transport: source/native layout graphs share every target.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {readStructuredDataLino} from '../../js/src/structured-data-links.js';
import {LinkNetwork} from '../../js/src/network.js';
import {LinkId} from '../../js/src/primitives.js';
import {decodeTypedProgramLinks,emitTypedProgramLinks} from '../../js/src/typed-program-links.js';
import {assertVersions} from './controls.mjs';
assertVersions();
const input=readStructuredDataLino(readFileSync(0,'utf8'));
for(const row of input.programs){const first=LinkNetwork.fromLino(row.legacy),second=LinkNetwork.fromLino(row.native),a=LinkId.from(row.legacyRoot),b=LinkId.from(row.nativeRoot);
 assert.deepEqual(decodeTypedProgramLinks(second,b),decodeTypedProgramLinks(first,a));
 for(const target of ['JavaScript','Rust','Lean','Rocq'])assert.deepEqual(emitTypedProgramLinks(second,b,target),emitTypedProgramLinks(first,a,target));
}
process.stdout.write('native record layout reload complete\n');
