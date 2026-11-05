// Literal declarations and independently readable original source only.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const fields=['alpha','bravo','charlie','delta','echo','foxtrot','golf','hotel','india','juliet','kilo','lima'];
function source({other=false,traced=false,failure='',floating=false}){
 const annotation='/** @typedef {{'+["$:'record'",...fields.map((name,index)=>name+':'+(floating&&index===0?'number':'string'))].join(',')+'}'+(other?'|{$:"other",value:string}':'')+'} Row */\n';
 const marked=(label,expression)=>traced?'marked('+JSON.stringify(label)+',()=>'+expression+')':expression;
 const record=index=>'({$:"record",'+fields.map((name,slot)=>name+':'+(slot===0?marked('field:alpha',failure==='field'?'failure()':floating?'numeric':JSON.stringify('alternative '+index)):slot===1?marked('field:bravo','value'):'value')).join(',')+'})';
 let body=other?'fallback(value)':record(5);
 for(let index=4;index>=0;index-=1)body=marked('condition:'+index,failure==='condition'&&index===0?'failure()':'choice==='+index)+'?'+record(index)+':('+body+')';
 return annotation+(traced?'function marked(label,read){console.log(label);return read();}\n':'')
  +(failure?'/** @returns {'+(failure==='condition'?'boolean':'string')+'} */ function failure(){throw new Error('+JSON.stringify(failure)+');}\n':'')
  +(other?'/** @param {string} value @returns {Row} */ function fallback(value){return {$:"other",value};}\n':'')
  +'/** @param {number} choice @param {string} value '+(floating?'@param {number} numeric ':'')+'@returns {Row} */\nexport function compute(choice,value'+(floating?',numeric':'')+'){return '+body+';}\n';
}
const cases=[
 {name:'one complete constructor with shared field bindings',other:false,failure:'',floating:false,expectShared:true},
 {name:'common constructor retains a different fallback body',other:true,failure:'',floating:false,expectShared:true},
 {name:'condition failure precedes every constructor argument',other:false,failure:'condition',floating:false,expectShared:true},
 {name:'first argument failure prevents later arguments',other:false,failure:'field',floating:false,expectShared:true},
 {name:'bare floating fields retain their original sites',other:false,failure:'',floating:true,expectShared:false}
].map(row=>({...row,source:source(row),tracedSource:source({...row,traced:true}),choices:[0,1,4,5,9]}));
writeProjectData(new URL('../../parity/fixtures/conditional-constructor-sharing.lino',import.meta.url),{
 schemaVersion:1,model:'closed value constructor selection 1',versions:{node:'>=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',rust:'>=1.90.0 <1.100.0; edition 2024'},fields,cases,
 owners:[
  {name:'complete actual owner coordinates',owner:'Example',constructor:'alternative',fields:2,owners:['Example','Example'],constructors:['alternative','alternative'],arities:[2,2],expected:true},
  {name:'different owners refuse',owner:'Example',constructor:'alternative',fields:2,owners:['Example','Other'],constructors:['alternative','alternative'],arities:[2,2],expected:false},
  {name:'different constructors refuse',owner:'Example',constructor:'alternative',fields:2,owners:['Example','Example'],constructors:['alternative','other'],arities:[2,2],expected:false},
  {name:'missing actual arguments refuse',owner:'Example',constructor:'alternative',fields:2,owners:['Example','Example'],constructors:['alternative','alternative'],arities:[2,1],expected:false},
  {name:'incomplete owner columns refuse',owner:'Example',constructor:'alternative',fields:2,owners:['Example','Example'],constructors:['alternative'],arities:[2,2],expected:false},
  {name:'unknown owner refuses',owner:'',constructor:'alternative',fields:2,owners:['Example','Example'],constructors:['alternative','alternative'],arities:[2,2],expected:false}
 ],
 selections:[
  {classes:[0,0],expected:[0]}, {classes:[0,1],expected:[0,1]},
  {classes:[0,1,1],expected:[1,0]}, {classes:[2,1,2,1],expected:[0,1,3]},
  {classes:[0,1,2],expected:[0,1,2]}, {classes:[0,0,1,0,2],expected:[0,2,4]},
  {classes:[],expected:[]},{classes:[0],expected:[]},{classes:[-1,0],expected:[]},
  {classes:[0,2],expected:[]},{classes:[0,0.5],expected:[]},{classes:Array(257).fill(0),expected:[]}
 ],
 boundary:'Unexecuted future ownership, selection, condition/field order, scope and complete typed-codec controls. Source fingerprints bind construction records, not semantic equivalence. Bare floating fields, ordinary references, open/function-bearing types and unsupported constructor metadata are excluded.'});
writeProjectData(new URL('../../parity/translation/conditional-constructor-sharing-profile.lino',import.meta.url),{
 schemaVersion:1,model:'closed value constructor selection 1',
 scope:'Source-authoring normalization of actual checked JavaScript algebraic value constructors. The ordinary typed-program gate and its resource limits remain unchanged.',
 obligations:['resolve every constructor through its actual checked declaration','capture the original complete condition tree once','retain every other leaf body','retain selected field evaluation order','group only same-scope variables, exact literals and actual nullary value constructors','retain ordinary references and bare floating allocation sites','preserve source and field origins','bind input/output construction records with typed UTF16 structural fingerprints'],
 limits:{alternatives:256,constructorFields:256,conditionalDepth:64,structuralDepth:240,sourceWork:1000000,fingerprintVisits:100000,fingerprintBytes:2097152},
 assumptions:['the source program has already passed its ordinary checker','constructors denote immutable algebraic values in the selected JavaScript value model','field classes are sound in the actual unchanged lexical scope','the captured ordinal selects the same original alternative','no field argument is evaluated before that captured choice'],
 proofs:[{language:'Lean',versionRange:'>=4.34.1 <4.35.0',source:'experiments/constructor-sharing/ConstructorSelection.lean'},
 {language:'Rocq',versionRange:'>=9.3.0 <9.4.0; rocq-stdlib >=9.2.0 <9.3.0',source:'experiments/constructor-sharing/ConstructorSelection.v'}],
 theoremBoundary:'Algebraic default-selection and constructor congruence laws under explicit class-soundness assumptions. They do not certify the complete source adapter, JavaScript representation or floating-point boxing.',
 workflow:'.github/workflows/ci.yml',job:'native-translations',
 boundary:'Unexecuted source draft and future controls. Construction hashes, classes and source receipts are not observations or semantic equivalence certificates.'});
