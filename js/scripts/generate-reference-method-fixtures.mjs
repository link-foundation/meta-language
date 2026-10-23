// Only fixture source declarations are authored; no implementation is imported.
import { writeProjectData } from './project-data.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft to author fixture declarations');
const source = body => '/** @returns {number} */\nexport function compute(){' + body + '}\n';
writeProjectData(new URL('../../parity/fixtures/reference-methods.lino', import.meta.url), {
  schemaVersion: 1, versions: { JavaScript: 'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0', Rust: '>=1.90.0 <1.100.0; edition 2024' },
  cases: [
    { name: 'push preserves aliases and returns new length', source: source('const values=[1,2];const alias=values;const length=values.push(3,4);return alias.length*100+length;') },
    { name: 'unshift preserves argument order', source: source('const values=[3];const alias=values;const length=values.unshift(1,2);return (alias[0]??0)*100+(alias[1]??0)*10+length;') },
    { name: 'pop preserves aliases and empty absence', source: source('const values=[2];const alias=values;const value=values.pop();const absent=values.pop();return (value??0)*100+(absent??7)*10+alias.length;') },
    { name: 'shift preserves insertion order', source: source('const values=[1,2,3];const alias=values;const first=values.shift();return (first??0)*100+(alias[0]??0)*10+alias.length;') },
    { name: 'reverse returns the same identity', source: source('const values=[1,2,3];const alias=values.reverse();return (alias===values?100:0)+(values[0]??0);') },
    { name: 'single spread creates a distinct allocation', source: source('const values=[1];const copy=[...values];copy.push(2);return (copy===values?100:0)+values.length*10+copy.length;') },
    { name: 'set membership canonicalizes numeric keys', source: source('const values=new Set([NaN,-0]);const alias=values.add(0);const has=alias.has(NaN);const removed=values.delete(NaN);return(has?100:0)+(removed?10:0)+(alias.has(0)?1:0);') },
    { name: 'map replacement preserves aliases', source: source('const values=new Map([[NaN,1],[-0,2]]);const alias=values;values.set(NaN,3);values.set(0,4);const first=alias.get(NaN)??0;const second=alias.get(-0)??0;return first*100+second*10+(values.delete(NaN)?1:0);') },
    { name: 'map clear changes the original allocation', source: source('const values=new Map([["a",1]]);const alias=values;values.clear(7);return alias.has("a")?1:0;') },
    { name: 'set clear and chained insertion', source: source('const values=new Set([1]);const alias=values.add(2).add(3);values.clear();return alias.has(3)?1:0;') },
    { name: 'push zero arguments returns existing length', source: source('const values=[1,2];return values.push();') },
    { name: 'read-only callback remains valid after mutation', source: source('const values=[1,2];values.push(3);return values.map(value=>value+1).length;') },
  ],
  refusals: [
    { name: 'callback observes shrinking storage', source: source('const values=[1,2];values.push(3);const mapped=values.map(value=>{values.pop();return value;});return mapped.length;'), reason: 'shared array callback' },
    { name: 'live collection iterator', source: source('const values=new Set([1,2]);values.add(3);return Array.from(values.values()).length;'), reason: 'mutable collection iteration' },
  ],
});
