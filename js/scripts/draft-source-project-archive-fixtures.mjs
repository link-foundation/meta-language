// Complete source bytes, independent graph invariants and predecessor context.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const sources=[
 {path:'src/program.js',language:'JavaScript',source:'// 😀 original trivia\r\nexport class Holder { #value = 7; read(){ return this.#value; } }\r\nconst pattern = /[a-z]+/gu;\n'},
 {path:'src/types.ts',language:'TypeScript',source:'export interface Holder<T> { readonly value?: T; }\nexport type Names<T> = { [K in keyof T]: K };\n'},
 {path:'src/lib.rs',language:'Rust',source:'//! 😀 complete generic source\npub struct Holder<T>(pub Option<T>);\nimpl<T> Holder<T> { pub fn new(value:T)->Self { Self(Some(value)) } }\n'},
 {path:'docs/guide.md',language:'Markdown',source:'# Source snapshot 😀\r\n\r\n```javascript\nconst value = { nested: [1, 2] };\n```\n\n> original **content**\n'},
 {path:'data/source.json',language:'JSON',source:'{ "text": "😀", "escaped": "\\uD800", "values": [null, true, 7] }\n'}
];
const cases=[
 {name:'complete priority-language source project',inputs:sources,options:{}},
 {name:'empty concrete roots remain real graph roots',inputs:sources.slice(0,4).map(row=>({...row,source:''})),options:{}},
 {name:'abstract views retain full concrete provenance',inputs:sources,options:{projection:'AbstractSyntax'}},
 {name:'Unicode paths and raw escape spellings stay exact',inputs:[{path:'𐐀/é.js',language:'JavaScript',source:'/* 😀 */ export const value = "\\uD800";\r\n'},{path:'語/raw.rs',language:'Rust',source:'const TEXT:&str=r###"quotes \" and braces {} [] 😀"###;\n'}],options:{}},
 {name:'source recovery nodes stay observable',inputs:[{path:'broken.js',language:'JavaScript',source:'function broken( { return 3;'},{path:'broken.rs',language:'Rust',source:'fn broken( {'},{path:'broken.ts',language:'TypeScript',source:'interface Broken { value: ;'}],options:{},unclean:true},
 {name:'original language aliases retain canonical source contexts',inputs:[{path:'alias.js',language:'js',source:'const value=7;\n'},{path:'alias.rs',language:'rs',source:'fn main(){}\n'}],options:{}}
];
const extensions=[{name:'new languages append without reparsing old graphs',initial:[sources[0]],added:sources.slice(1),options:{}},{name:'old abstract selection remains the default after extension',initial:[sources[0]],added:[sources[2]],options:{projection:'AbstractSyntax'}}];
const refusals=[
 {name:'empty source inventory',inputs:[],options:{},message:'bounded nonempty source inventory'},
 {name:'duplicate source identities',inputs:[sources[0],sources[0]],options:{},message:'duplicate source project path'},
 {name:'unknown source language',inputs:[{...sources[0],language:'unknown source language'}],options:{},message:'unknown source language'},
 {name:'unresolved parent path',inputs:[{...sources[0],path:'../source.js'}],options:{},message:'invalid original source input'},
 {name:'bounded source bytes',inputs:[sources[0]],options:{byteLimit:1},message:'source byte limit exceeded'},
 {name:'bounded original file inventory',inputs:sources,options:{fileLimit:1},message:'bounded nonempty source inventory'},
 {name:'unknown syntax selection',inputs:[sources[0]],options:{projection:'missing'},message:'unknown source syntax projection'},
 {name:'clean admission never erases recovery information',inputs:cases[4].inputs,options:{requireClean:true},message:'unclean original concrete syntax'}
];
const rootCases=[{kinds:['syntax','syntax','other'],identifiers:[1,2,3],firsts:[0,1,1],counts:[1,0,0],targets:[2],expected:[0]},{kinds:['syntax','syntax'],identifiers:[5,8],firsts:[0,0],counts:[0,0],targets:[],expected:[0,1]},{kinds:['other'],identifiers:[1],firsts:[0],counts:[0],targets:[],expected:[]}];
const revision='4f946613',history=['js/src/rust-source-project.js','rust/src/rust_source_project.rs','js/src/index.js','rust/src/lib.rs'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return {path,revision,source:bytes.toString('utf8'),bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex')};});
writeProjectData(new URL('../../parity/fixtures/source-project-archive.lino',import.meta.url),{schemaVersion:1,model:'complete source project concrete graphs 1',cases,extensions,refusals,rootCases,history,versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024',parsers:'exact default catalog parser digests retained by each actual original graph; web-tree-sitter ==0.27.0'},assertions:['defaultCompleteConcreteGraphsAndExplicitRoots','exactTokensTriviaFieldsHierarchySpansAndRecoveryRetained','completeRegisteredTermsSelectedSyntaxAndAllocatorState','sourceFreeTextBinaryReloadWithoutCurrentCatalogDispatch','optionalAbstractViewNeverRemovesOriginalConcreteData','extensionPreservesEveryPrecedingGraphAndSourceIdentity','equalNamesAndBytesNeverCertifyConceptEquivalence','existingRustSemanticPacketsReuseCompleteArchiveStorage','completeSourceHistoryAndNativeGraphProducts'],boundary:'Future graph, codec, native and source invariants only. No source fixture, parser product, archive replay, behavior comparison, code test or observation ran. Source snapshots preserve the selected parser graphs; full native grammar independence, every grammar definition/concept meaning, exact-ID integration and the preceding translation requirements remain mandatory.'});
