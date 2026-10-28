// Literal plan inputs and independent expected outputs; never invoke the rule.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const root=new URL('../../',import.meta.url),base={owner:'Value',declared:['first','second','third'],owners:['Value','Value','Value'],constructors:['first','second','third'],literals:[1,0,0]};
const rows=[
 {name:'one distinct true arm retains its original index',...base,expected:[0,0]},
 {name:'one distinct false arm retains its original index',...base,literals:[0,1,1],expected:[1,0]},
 {name:'all false cases still retain a wildcard scrutinee match',...base,literals:[0,0,0],expected:[0]},
 {name:'all true cases still retain a wildcard scrutinee match',...base,literals:[1,1,1],expected:[1]},
 {name:'an equal-sized grouping deterministically selects false',owner:'Value',declared:['first','second','third','fourth'],owners:['Value','Value','Value','Value'],constructors:['first','second','third','fourth'],literals:[0,1,0,1],expected:[0,1,3]},
 {name:'a reordered complete inventory retains original occurrence indices',...base,constructors:['third','first','second'],literals:[0,1,0],expected:[0,1]},
 {name:'a missing constructor refuses',...base,owners:['Value','Value'],constructors:['first','second'],literals:[1,0],expected:[]},
 {name:'a duplicate constructor refuses',...base,constructors:['first','second','second'],expected:[]},
 {name:'a foreign constructor refuses',...base,constructors:['first','second','other'],expected:[]},
 {name:'a counterfeit owner refuses',...base,owners:['Other','Value','Value'],expected:[]},
 {name:'a duplicate declaration refuses',...base,declared:['first','second','second'],expected:[]},
 {name:'a non-Boolean body refuses',...base,literals:[1,0,2],expected:[]},
 {name:'an absent owner refuses',...base,owner:'',expected:[]},
 {name:'a missing declaration name refuses',...base,declared:['first','second',''],expected:[]},
 {name:'no repeated Boolean result leaves both arms intact',owner:'Value',declared:['first','second'],owners:['Value','Value'],constructors:['first','second'],literals:[0,1],expected:[]},
 ];
const prefix='/** @typedef {{$:"first"}|{$:"second"}|{$:"third"}} Value */\n/** @param {{value:number}} holder @param {Value} value @returns {Value} */ function observe(holder,value){holder.value+=1;return value;}\n';
const programs=['first','second','third'].map(tag=>({name:'sharing retains scrutinee effects for '+tag,source:prefix+'/** @returns {Value} */ function make(){return {$:'+JSON.stringify(tag)+'};}\n/** @param {{value:number}} holder @param {Value} value @returns {boolean} */ function inspect(holder,value){return observe(holder,value).$==="first";}\n/** @returns {number} */ function compute(){const holder={value:0};const value=make();const result=inspect(holder,value);return holder.value+(result?10:20);}',expected:tag==='first'?11:21}));
const generalRows=[
 {name:'a noncandidate effectful arm retains its original position',...base,classes:[0,-1,0],expected:[0,1]},
 {name:'a distinct closed outcome retains its original position',...base,classes:[0,1,0],expected:[0,1]},
 {name:'a repeated second outcome becomes the exact default',...base,classes:[0,1,1],expected:[1,0]},
 {name:'no closed candidate is invented',...base,classes:[-1,-1,-1],expected:[]},
 {name:'unique outcomes remain unchanged',...base,classes:[0,1,2],expected:[]},
 {name:'fractional outcome identities refuse',...base,classes:[0,0,0.5],expected:[]},
 {name:'out of inventory outcome identities refuse',...base,classes:[0,0,3],expected:[]},
 {name:'unknown negative outcome identities refuse',...base,classes:[0,0,-2],expected:[]},
 {name:'all identical outcomes retain a scrutinee match',...base,classes:[0,0,0],expected:[0]},
 {name:'closed outcomes retain owner qualification',...base,owners:['Other','Value','Value'],classes:[0,0,0],expected:[]},
 {name:'closed outcomes retain exact constructor coverage',...base,constructors:['first','first','third'],classes:[0,0,0],expected:[]},
 ];
const bindingRows=[
 {name:'only unused named slots are removed',bindings:['first','second',''],references:['second'],expected:[0]},
 {name:'every actually referenced slot remains',bindings:['first','second'],references:['first','second'],expected:[]},
 {name:'conservative shadowing references retain the slot',bindings:['shadowed'],references:['shadowed'],expected:[]},
 {name:'already ignored slots stay ignored',bindings:['',''],references:[],expected:[]},
 {name:'all unused slots retain their exact indices',bindings:['first','second','third'],references:['other'],expected:[0,1,2]},
 ];
const aliasRows=[
 {name:'an unused lexical field alias can disappear',nameOfBinding:'copy',value:'field',available:['field'],references:[],expected:true},
 {name:'a referenced lexical alias remains',nameOfBinding:'copy',value:'field',available:['field'],references:['copy'],expected:false},
 {name:'an unproved lexical read is never dropped',nameOfBinding:'copy',value:'outside',available:['field'],references:[],expected:false},
 {name:'an unnamed alias is not invented',nameOfBinding:'',value:'field',available:['field'],references:[],expected:false},
 {name:'a shadowing reference conservatively retains an alias',nameOfBinding:'copy',value:'field',available:['field'],references:['copy'],expected:false},
 ];
const projectionRows=[
 {name:'a repeated field keeps its exact slot',constructors:1,counts:[2,0],expected:[0]},
 {name:'two used fields keep their exact order',constructors:1,counts:[1,1],expected:[0,1]},
 {name:'a single projection remains in place',constructors:1,counts:[1,0],expected:[]},
 {name:'multiple constructors require their original match',constructors:2,counts:[2],expected:[]},
 {name:'unused fields are never loaded eagerly',constructors:1,counts:[0,0],expected:[]},
 {name:'fractional occurrence counts refuse',constructors:1,counts:[2,0.5],expected:[]},
 {name:'negative occurrence counts refuse',constructors:1,counts:[2,-1],expected:[]},
 {name:'oversized occurrence work refuses',constructors:1,counts:[100001],expected:[]},
 ];
const pairSource='/** @typedef {{$:"pair",left:number,right:number}} Pair */\n/** @returns {Pair} */ function makePair(){return {$:"pair",left:2,right:3};}\n/** @param {Pair} value @returns {number} */ function projectPair(value){return value.left+value.right+value.left;}\n/** @returns {number} */ function compute(){return projectPair(makePair());}';
const projectionPrograms=[
 {name:'actual immutable parameter projections preserve their complete values',source:pairSource,expected:7,referenceSemantics:false,shared:true},
 {name:'reference semantics retain their original projections',source:pairSource,expected:7,referenceSemantics:true,shared:false},
 {name:'deferred closure projections remain inside their closures',source:'/** @typedef {{$:"pair",left:number,right:number}} Pair */\n/** @returns {Pair} */ function makePair(){return {$:"pair",left:2,right:3};}\n/** @param {Pair} value @returns {number} */ function projectPair(value){return (()=>value.left)()+(()=>value.right)();}\n/** @returns {number} */ function compute(){return projectPair(makePair());}',expected:5,referenceSemantics:false,shared:false},
 ];
writeProjectData(new URL('parity/fixtures/boolean-constructor-match.lino',root),{schemaVersion:1,model:'complete finite Boolean constructor match sharing 1',generalModel:'closed constructor match sharing 1',nodeVersionRange:'>=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',rustVersionRange:'>=1.90.0 <1.100.0',rows,generalRows,bindingRows,aliasRows,projectionRows,programs,projectionPrograms});
