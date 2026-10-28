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
writeProjectData(new URL('parity/fixtures/boolean-constructor-match.lino',root),{schemaVersion:1,model:'complete finite Boolean constructor match sharing 1',nodeVersionRange:'>=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',rustVersionRange:'>=1.90.0 <1.100.0',rows,programs});
