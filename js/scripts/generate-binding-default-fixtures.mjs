// Serialize declarations only; no initializer, fixture or oracle executes.
import {writeProjectData} from './project-data.mjs';
import {accepted,refused,correctedBindingRefusals,bindingAssertionCorrectionBasis} from '../tests/fixtures/array-binding-cases.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
writeProjectData(new URL('../../parity/fixtures/javascript-array-bindings.lino',import.meta.url),{
 schemaVersion:1,model:'ordered homogeneous JavaScript array bindings 1',accepted,refused,correctedBindingRefusals,bindingAssertionCorrectionBasis,
});
const rows=[
 ['null survives Number initializer','const [value=7]=[null];return value;','null'],
 ['undefined selects Number initializer','const [value=7]=[undefined];return value;','7'],
 ['missing selects structured initializer','const [value=[7]]=[];return value.length;','1'],
 ['present structured value skips initializer','const [value=[7,8]]=[[2]];return value.length;','1'],
 ['missing selects record initializer','const [value={field:7}]=[];return value.field;','7'],
 ['present record preserves field','const [value={field:7}]=[{field:2}];return value.field;','2'],
 ['null fallback survives missing element','const [value=null]=[];return value;','null'],
 ['undefined fallback survives missing element','const [value=undefined]=[];return value;','undefined'],
 ['present Number skips null fallback','const [value=null]=[2];return value;','2'],
 ['present false skips undefined fallback','const [value=undefined]=[false];return value;','false'],
 ['present NaN skips fallback','const [value=7]=[NaN];return Number.isNaN(value);','true'],
 ['present signed zero skips fallback','const [value=7]=[-0];return Object.is(value,-0);','true'],
 ['null does not evaluate initializer','let visits=0;const [value=(visits=1)]=[null];return value===null&&visits===0;','true'],
 ['undefined evaluates initializer once','let visits=0;const [value=(visits+=1)]=[undefined];return value===1&&visits===1;','true'],
 ['default bindings remain ordered','let visits=0;const [first=(visits+=1),second=(visits+=first)]=[];return visits===2&&first===1&&second===2;','true'],
];
const cases=rows.map(([name,body,value],index)=>({name,entry:'bindingDefault'+index,source:'export function compute(){'+body+'}',value,abort:null}));
cases.push({name:'structured initializer skipped for present element',entry:'bindingDefaultLazy',source:'/** @returns {number[]} */ function bindingDefaultFailure(){throw new Error("initializer selected");}\nexport function compute(){const [value=bindingDefaultFailure()]=[[2]];return value.length;}',value:'1',abort:null});
writeProjectData(new URL('../../parity/fixtures/binding-defaults.lino',import.meta.url),{
 schemaVersion:1,model:'JavaScript represented binding defaults and retained null 2',sourceLanguage:'JavaScript',cases,refusals:[],
 source:cases.map(row=>row.source.replace('function compute(','function '+row.entry+'(')).join('\n'),
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0; ECMAScript 2025',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:bindingAssertionCorrectionBasis.source,correctedBindingRefusals,bindingAssertionCorrectionBasis,
 boundary:'Declared actual source defaults, null/undefined tags, structured present domains, lazy ordered initializer effects, retained former inputs/assertions, both runtimes and complete program/definition codecs/emissions. No observations. General iterators, nested pattern protocols, heterogeneous value unions and all earlier requirements remain.',
});
