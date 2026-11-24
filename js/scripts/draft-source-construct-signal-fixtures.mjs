// Literal expectations only; the inspected source is never imported/executed.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const originals=[
 {name:'empty directory',source:'',needles:[],expected:[]},
 {name:'empty signals never consume source',source:'abc',needles:['','missing'],expected:[0,-1,-1,0,-1,-1]},
 {name:'LF source coordinates and distinct signals',source:'a\naa\na',needles:['a','aa',''],expected:[4,0,1,1,2,2,0,-1,-1]},
 {name:'nonoverlapping occurrences retain source order',source:'aaaa',needles:['aa','aaa','b'],expected:[2,0,1,1,0,1,0,-1,-1]},
 {name:'UTF16 offsets differ from scalar coordinates',source:'😀\ntext\r\nclass C{}',needles:['class ','😀','\n'],expected:[1,9,3,1,0,1,2,2,1]},
 {name:'isolated units remain literal source data',source:'\ud800\nMap\udfff',needles:['Map','\udfff'],expected:[1,2,2,1,5,2]},
 {name:'comments and literals stay explicit false-positive candidates',source:'// class example\nconst text="class example";',needles:['class '],expected:[2,3,1]}
];
const units=value=>Array.from({length:value.length},(_,index)=>value.charCodeAt(index));
const cases=originals.map(({source,needles,...row})=>({...row,sourceUnits:units(source),needleUnits:needles.map(units)}));
writeProjectData(new URL('../../parity/fixtures/source-construct-signals.lino',import.meta.url),{schemaVersion:1,model:'literal source construct candidate statistics 1',cases,versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0; UTF16 offsets and LF-delimited lines',Rust:'>=1.90.0 <1.100.0; edition 2024'},assertions:['literalNonoverlappingCountsPreserved','firstUtf16OffsetsAndLfLinesExact','emptyAndAbsentSignalsNeverLoop','commentsAndStringsNeverClaimSemanticCoverage','bothRuntimesAndTransportedSourceDefinitionsRetainAllCoordinates'],boundary:'Unexecuted independent literal controls. Counts are lexical candidates, never parse/type/translation evidence. No inspected source, fixture or product executes during authoring.'});
