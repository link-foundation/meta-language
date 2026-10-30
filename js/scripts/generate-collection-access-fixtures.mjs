// Author source/contract declarations without loading or invoking a translator.
import { writeProjectData } from './project-data.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft to author fixture declarations');
const source = body => '/** @returns {number} */\nexport function compute(){' + body + '}\n';
writeProjectData(new URL('../../parity/fixtures/collection-access.lino', import.meta.url), {
  schemaVersion: 1, versions: { JavaScript: 'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0', Rust: '>=1.90.0 <1.100.0; edition 2024' },
  cases: [
    { name: 'map reads the actual string key', source: source('const values=new Map([["a",7],["🙂",11]]);return values.get("🙂")??0;') },
    { name: 'missing map key retains undefined', source: source('const values=new Map([["a",7]]);return values.get("b")??17;') },
    { name: 'set membership uses SameValueZero', source: source('const values=new Set([NaN,-0,3]);return(values.has(NaN)?100:0)+(values.has(0)?10:0)+(values.has(4)?1:0);') },
    { name: 'map size counts unique keys', source: source('const values=new Map([[NaN,1],[NaN,2],[-0,3],[0,4]]);return values.size;') },
    { name: 'set size counts canonical keys', source: source('const values=new Set([0,-0,NaN,NaN]);return values.size;') },
    { name: 'map keys retain complete collection length', source: source('const values=new Map([["a",1],["b",2]]);return Array.from(values.keys()).length;') },
    { name: 'set values retain insertion order', source: source('const values=new Set([3,1,3]);const items=Array.from(values.values());return(items[0]??0)*10+(items[1]??0);') },
    { name: 'map entries preserve pair count', source: source('const values=new Map([["a",1],["b",2]]);return Array.from(values.entries()).length;') },
    { name: 'boolean map membership uses typed keys', source: source('const values=new Map([[true,1],[false,2]]);return(values.has(true)?10:0)+(values.get(false)??0);') },
  ],
});
