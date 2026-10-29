// Declarative inputs only; no translator or represented function is invoked.
import { writeProjectData } from './project-data.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft to author fixture declarations');
const source = body => '/** @returns {number} */\nexport function compute(){' + body + '}\n';
writeProjectData(new URL('../../parity/fixtures/record-writes.lino', import.meta.url), {
  schemaVersion: 1, versions: { JavaScript: 'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0', Rust: '>=1.90.0 <1.100.0; edition 2024' },
  cases: [
    { name: 'an alias reads a direct own field write', source: source('const item={value:1};const alias=item;alias.value=7;return item.value;') },
    { name: 'nested record identities survive writes', source: source('const item={child:{value:1}};const alias=item.child;alias.value=13;return item.child.value;') },
    { name: 'numeric compound operations retain the actual shared allocation', source: source('const item={value:10};const alias=item;item.value+=2;alias.value-=3;item.value*=4;item.value/=2;item.value%=7;return alias.value;') },
    { name: 'prefix and postfix statement updates share fields', source: source('const item={value:8};const alias=item;++item.value;alias.value++;--item.value;alias.value--;return item.value;') },
    { name: 'exponentiation applies Number special values', source: source('const item={value:1};item.value**=Infinity;return Number.isNaN(item.value)?1:0;') },
    { name: 'NaN to zero exponent returns one', source: source('const item={value:NaN};item.value**=0;return item.value;') },
    { name: 'simple string assignment preserves text', source: source('const item={value:"a"};const alias=item;item.value="🙂";return alias.value.length;') },
    { name: 'simple Boolean assignment retains the field type', source: source('const item={value:false};item.value=true;return item.value?1:0;') },
    { name: 'simple BigInt assignment remains exact', source: source('const item={value:1n};item.value=9007199254740993n;return item.value===9007199254740993n?1:0;') },
    { name: 'replacing a reference field keeps its new alias', source: source('const old={value:1};const next={value:7};const holder={child:old};holder.child=next;next.value=19;return holder.child.value*10+old.value;') },
    { name: 'compound reads before right side mutates the receiver', source: source('const item={value:2};const update=()=>{item.value=40;return 3;};item.value+=update();return item.value;') },
    { name: 'nested right side writes release previous storage borrows', source: source('const item={value:1,other:2};const update=()=>{item.other=9;return 7;};item.value=update();return item.value*10+item.other;') },
    { name: 'signed zero survives a simple field assignment', source: source('const item={value:1};item.value=-0;return 1/item.value===-Infinity?1:0;') },
    { name: 'generated field temporary names avoid source captures', source: source('const ml_field_owner1=7;const ml_field_value2=11;const item={value:1};item.value=ml_field_owner1;item.value+=ml_field_value2;return item.value;') },
    // The former refusal input is retained as a success through checked concatenation.
    { name: 'string compound writes use checked concatenation', source: source('const item={value:"a"};item.value+="b";return item.value.length;') },
    { name: 'string compound writes convert exact integers', source: source('const item={value:"a"};item.value+=9007199254740993n;return item.value==="a9007199254740993"?1:0;') },
    { name: 'string compound writes retain UTF16 units', source: source('const item={value:"\\uD800"};item.value+="\\0\\uDC00";return item.value.length;') },
    { name: 'BigInt compound writes retain exact values and aliases', source: source('const item={value:9007199254740993n};const alias=item;item.value+=2n;alias.value-=3n;item.value*=4n;item.value/=2n;item.value%=7n;return item.value===1n?1:0;') },
    { name: 'BigInt compound negative division truncates', source: source('const item={value:-7n};item.value/=3n;return item.value===-2n?1:0;') },
    { name: 'BigInt statement increments retain their domain', source: source('const item={value:9007199254740993n};const alias=item;++item.value;alias.value++;--item.value;alias.value--;return item.value===9007199254740993n?1:0;') },
    { name: 'BigInt compound exponentiation uses its checked kernel', source: source('const item={value:3n};item.value**=4n;return item.value===81n?1:0;') },
    { name: 'BigInt previous value precedes mutation by the right operand', source: source('const item={value:2n};const update=()=>{item.value=40n;return 3n;};item.value+=update();return item.value===5n?1:0;') },
  ],
  refusals: [
    { name: 'dynamic field creation keeps descriptor obligation', source: source('const item={value:1};item.extra=2;return item.value;'), reason: 'field' },
    { name: 'array index writes keep live slot obligation', source: source('const item=[1,2];const alias=item;alias[0]=7;return item[0]??0;'), reason: 'live slots' },
  ],
});
