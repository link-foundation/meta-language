// Independent literal vectors. Values are opaque identity tokens, never keys.
export const stableVectorSortVersions={node:'>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0',model:'complete stable opaque UTF16 keyed vector insertion sort 1'};
export const stableVectorSortFixtures=[
 {name:'empty',input:[],keys:[],values:[]},
 {name:'single',input:[{key:'a',value:7}],keys:['a'],values:[7]},
 {name:'ascending',input:[{key:'a',value:1},{key:'b',value:2},{key:'c',value:3}],keys:['a','b','c'],values:[1,2,3]},
 {name:'reverse',input:[{key:'c',value:3},{key:'b',value:2},{key:'a',value:1}],keys:['a','b','c'],values:[1,2,3]},
 {name:'equal keys preserve opaque order',input:[{key:'same',value:8},{key:'same',value:2},{key:'same',value:5}],keys:['same','same','same'],values:[8,2,5]},
 {name:'interleaved equal keys',input:[{key:'b',value:4},{key:'a',value:1},{key:'b',value:5},{key:'a',value:2}],keys:['a','a','b','b'],values:[1,2,4,5]},
 {name:'last entry inserts first',input:[{key:'b',value:2},{key:'c',value:3},{key:'a',value:1}],keys:['a','b','c'],values:[1,2,3]},
 {name:'middle entry inserts between neighbors',input:[{key:'a',value:1},{key:'c',value:3},{key:'b',value:2}],keys:['a','b','c'],values:[1,2,3]},
 {name:'empty keys and duplicate empty keys',input:[{key:'a',value:3},{key:'',value:1},{key:'',value:2}],keys:['','','a'],values:[1,2,3]},
 {name:'prefixes',input:[{key:'abc',value:3},{key:'a',value:1},{key:'ab',value:2}],keys:['a','ab','abc'],values:[1,2,3]},
 {name:'UTF16 astral key precedes high BMP key',input:[{key:'\ue000',value:2},{key:'😀',value:1}],keys:['😀','\ue000'],values:[1,2]},
 {name:'UTF16 supplementary boundary precedes final BMP code point',input:[{key:'\uffff',value:2},{key:'𐀀',value:1}],keys:['𐀀','\uffff'],values:[1,2]},
 {name:'astral shared prefix suffix order',input:[{key:'😀b',value:3},{key:'😀',value:1},{key:'😀a',value:2}],keys:['😀','😀a','😀b'],values:[1,2,3]},
 {name:'embedded zero code unit',input:[{key:'aa',value:3},{key:'a\0',value:2},{key:'a',value:1}],keys:['a','a\0','aa'],values:[1,2,3]},
 {name:'zero key precedes all other valid scalar keys',input:[{key:'é',value:3},{key:'\0',value:1},{key:'Z',value:2}],keys:['\0','Z','é'],values:[1,2,3]},
 {name:'no Unicode normalization',input:[{key:'é',value:2},{key:'e\u0301',value:1}],keys:['e\u0301','é'],values:[1,2]},
 {name:'case uses code units',input:[{key:'z',value:4},{key:'a',value:3},{key:'Z',value:2},{key:'A',value:1}],keys:['A','Z','a','z'],values:[1,2,3,4]},
 {name:'decimal-looking keys stay lexical',input:[{key:'2',value:3},{key:'10',value:2},{key:'1',value:1}],keys:['1','10','2'],values:[1,2,3]},
 {name:'punctuation and spaces',input:[{key:'_',value:4},{key:'!',value:2},{key:' ',value:1},{key:'0',value:3}],keys:[' ','!','0','_'],values:[1,2,3,4]},
 {name:'payload values never break ties',input:[{key:'x',value:99},{key:'x',value:-1},{key:'x',value:0}],keys:['x','x','x'],values:[99,-1,0]},
 {name:'equal keys remain stable after earlier smaller insertion',input:[{key:'b',value:8},{key:'b',value:3},{key:'a',value:1},{key:'b',value:7}],keys:['a','b','b','b'],values:[1,8,3,7]},
 {name:'alternating three key classes',input:[{key:'c',value:6},{key:'b',value:4},{key:'a',value:1},{key:'c',value:7},{key:'a',value:2},{key:'b',value:5}],keys:['a','a','b','b','c','c'],values:[1,2,4,5,6,7]},
 {name:'BMP and astral ties interleave stably',input:[{key:'\ue000',value:3},{key:'😀',value:1},{key:'\ue000',value:4},{key:'😀',value:2}],keys:['😀','😀','\ue000','\ue000'],values:[1,2,3,4]},
 {name:'whitespace prefix distinction',input:[{key:'\n ',value:3},{key:'\t',value:1},{key:'\n',value:2}],keys:['\t','\n','\n '],values:[1,2,3]}
];
// These JavaScript string boundaries are deliberately separate from actual Rust
// owned String source domains, which never contain lone surrogate code units.
export const stableVectorSortJavaScriptBoundaryFixtures=[
 {name:'lone high before low surrogate',input:[{key:'\udc00',value:2},{key:'\ud800',value:1}],keys:['\ud800','\udc00'],values:[1,2]},
 {name:'lone high is proper prefix of valid surrogate pair',input:[{key:'𐀀',value:2},{key:'\ud800',value:1}],keys:['\ud800','𐀀'],values:[1,2]}
];
