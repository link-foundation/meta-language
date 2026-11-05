// Fixture declarations only; never parse or execute represented source documents.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const allocations=[
 {name:'empty directory retains its requested cursor',first:[1,0],count:0,limit:10,expected:[0,1,0]},
 {name:'low limb carry is exact',first:[4294967295,0],count:2,limit:10,expected:[2,1,1,4294967295,0,0,1]},
 {name:'neighbors above the Number boundary remain distinct',first:[1,2097152],count:3,limit:10,expected:[3,4,2097152,1,2097152,2,2097152,3,2097152]},
 {name:'maximum identity is usable before exhaustion',first:[4294967295,4294967295],count:1,limit:10,expected:[1,-1,-1,4294967295,4294967295]},
 {name:'two final identities exactly exhaust the cursor',first:[4294967294,4294967295],count:2,limit:10,expected:[2,-1,-1,4294967294,4294967295,4294967295,4294967295]},
];
const refusals=[
 {name:'zero has no source identity',first:[0,0],count:0,limit:10,error:'source graph allocation needs a positive unsigned first identity'},
 {name:'no partial allocation on exhaustion',first:[4294967295,4294967295],count:2,limit:10,error:'source graph allocation exceeds the unsigned identity space'},
 {name:'directory record limit remains explicit',first:[1,0],count:3,limit:2,error:'source graph allocation exceeds its identity directory limit'},
 {name:'negative directory length refuses',first:[1,0],count:-1,limit:10,error:'source graph allocation exceeds its identity directory limit'},
];
const documents=[
 {name:'JavaScript comments and exact Unicode source',language:'JavaScript',source:'// source 😀\nexport function add(left, right) { return left + right; }\n'},
 {name:'Rust source with attributes and trivia',language:'Rust',source:'// source\n#[inline]\npub fn add(left: u32, right: u32) -> u32 { left + right }\n'},
 {name:'TypeScript annotations remain concrete',language:'TypeScript',source:'interface Entry { value: string; }\nexport const text: string = "é中😀";\n'},
 {name:'Markdown recursive language regions',language:'Markdown',source:'# Links\n\n```javascript\nconst value = { nested: "markdown" };\n```\n\n> source\n'},
 {name:'JSON exact punctuation and whitespace',language:'JSON',source:'{ "source": [1, true, "é中😀"] }\n'},
 {name:'Python indentation and comments',language:'Python',source:'# source\ndef add(left, right):\n    return left + right\n'},
];
const graphs=[
 {name:'maximum self link',source:'(7: 7)',first:'18446744073709551615',sourceIdentities:['7'],identities:['18446744073709551615'],next:null},
 {name:'repeated dangling reference is preserved',source:'(2: 9 2 9)',first:'18446744073709551614',sourceIdentities:['2','9'],identities:['18446744073709551614','18446744073709551615'],next:null},
 {name:'cycles retain order across the safe boundary',source:'(3: 8 3 8)\n(8: 3)',first:'9007199254740993',sourceIdentities:['3','8'],identities:['9007199254740993','9007199254740994'],next:'9007199254740995'},
];
const programs=[
 {name:'JavaScript checked arithmetic graph',language:'JavaScript',factory:'ordinary',source:'export function result(){const left=3;return left+4;}',expected:'7'},
 {name:'Rust checked fixed integer graph',language:'Rust',factory:'ordinary',source:'pub fn result()->u32{let left=3u32;left+4u32}',expected:'7'},
 {name:'Rust actual source owners and const calls',language:'Rust',factory:'Rust source owners',source:'pub const fn add(left:u8,right:u8)->u8{left+right}pub const VALUE:u8=add(3,4);pub fn result()->bool{matches!(7u8,VALUE)}',expected:'true'},
];
writeProjectData(new URL('../../parity/fixtures/unsigned-source-documents.lino',import.meta.url),{schemaVersion:1,model:'exact unsigned source document coordinate allocation 1',firstIdentity:'9007199254740993',allocations,refusals,documents,graphs,programs,versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024',binary:'MLU64N01 wrapping MLDSNP02'},sourceDefinitionPrograms:['meta-language/source-program/unsigned-source-document-program'],boundary:'Unexecuted source documents, exact directory controls and native/JavaScript snapshots. Fresh graph coordinate allocation preserves every original reference, source metadata, registration and syntax cache. Coordinates do not prove semantic equivalence or merge concepts. Source graphs remain unchanged. Every remaining CLI/schema/algorithm exact-ID integration and whole-language/grammar/concept requirement remains active.'});
