// Literal controls only; no source program or shared planner is executed.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[
 {name:'no unresolved source',count:3,failed:[],importers:[1,2],providers:[0,1],limit:10000,expected:[]},
 {name:'failed source without dependents',count:2,failed:[0],importers:[],providers:[],limit:10000,expected:[]},
 {name:'one actual dependent',count:2,failed:[0],importers:[1],providers:[0],limit:10000,expected:[1,0,0]},
 {name:'transitive chain keeps disconnected source available',count:4,failed:[0],importers:[1,2],providers:[0,1],limit:21,expected:[1,0,0,2,1,0]},
 {name:'exact budget is required for the whole closure',count:4,failed:[0],importers:[1,2],providers:[0,1],limit:20,expected:[-1]},
 {name:'cycles preserve their original root and terminate',count:3,failed:[0],importers:[1,2,0],providers:[0,1,2],limit:10000,expected:[1,0,0,2,1,0]},
 {name:'duplicate edges never duplicate a dependency gap',count:2,failed:[0,0],importers:[1,1],providers:[0,0],limit:10000,expected:[1,0,0]},
 {name:'multiple roots choose a reproducible shortest witness',count:4,failed:[0,1],importers:[2,2,3],providers:[0,1,2],limit:10000,expected:[2,0,0,3,2,0]},
 {name:'root order remains explicit',count:4,failed:[1,0],importers:[2,2,3],providers:[0,1,2],limit:10000,expected:[2,1,1,3,2,1]},
 {name:'own unresolved source keeps its original diagnostic',count:3,failed:[0,1],importers:[1,2],providers:[0,1],limit:10000,expected:[2,1,1]},
 {name:'self dependency does not manufacture a witness',count:1,failed:[0],importers:[0],providers:[0],limit:10000,expected:[]},
 {name:'budget exhaustion preserves no partially certified closure',count:3,failed:[0],importers:[1,2],providers:[0,1],limit:1,expected:[-1]}
];
writeProjectData(new URL('../../parity/fixtures/project-dependency-gaps.lino',import.meta.url),{schemaVersion:1,cases,source:'export function value(){return 7n;}',sourceIssue:'computed module specifier requires a source interpretation',versions:{node:'>=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',rust:'>=1.90.0 <1.100.0; edition 2024'},boundary:'Unexecuted literal graph decisions and actual source controls. A dependency witness withholds executable output; it does not prove translation or loading semantics. Complete original diagnostics, draft code and source inventories remain intact.'});
