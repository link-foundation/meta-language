// Input/expected declarations only. No order operation or oracle executes.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const units=text=>Array.from({length:text.length},(_,index)=>text.charCodeAt(index));
const cases=[];
function add(name,operation,names,input,expected,error=null,extra={}){cases.push({name,operation,names:names.map(units),input,expected,error,...extra});}
add('empty own field set','create',[],[],[]);
add('non-numeric insertion differs from canonical layout','create',['a','b','c'],[2,0,1],[2,0,1]);
add('numeric own names are ordered numerically','create',['10','2','a','z'],[3,2,0,1],[1,0,3,2]);
add('canonical index endpoint and non-index spellings','create',['01','1','4294967294','4294967295','__proto__','a','b'],[6,5,4,3,2,1,0],[1,2,6,5,4,3,0]);
add('empty property name remains a real field','create',['','a'],[1,0],[1,0]);
add('UTF16 layout includes isolated surrogates','create',['a','\ud800','\ue000'],[2,1,0],[2,1,0]);
add('astral name layout uses UTF16 ordering','create',['😀','\ue000'],[1,0],[1,0]);
add('NUL property name remains exact','keys',['\0','a'],[1,0],[units('a'),units('\0')]);
add('ordinary keys retain insertion order','keys',['a','b','c'],[2,0,1],[units('c'),units('a'),units('b')]);
add('surrogate key vectors retain exact code units','keys',['a','\ud800','\ue000'],[2,1,0],[units('\ue000'),units('\ud800'),units('a')]);
add('overwrites never move first insertion','merge',['a','b','c'],[[2],[0],[2],[1]],[2,0,1]);
add('spread chunks merge with ordinary fields','merge',['a','b','c','d'],[[2,0],[1],[0,3],[2]],[2,0,1,3]);
add('spread merge still places numeric keys first','merge',['10','2','a','b'],[[3],[0],[2],[1],[3]],[1,0,3,2]);
add('empty spread chunks are permitted','merge',['a'],[[],[0],[]],[0]);
add('empty full spread inventory','merge',[],[[],[]],[]);
add('spread order rebases into its new canonical layout','rebase',['10','2','a','z'],[0,2,1],[0,3,2],null,{sourceNames:['10','a','z'].map(units)});
add('spread own order remains numeric first','rebase',['10','2','a','z'],[1,0],[1,0],null,{sourceNames:['10','2'].map(units)});
add('duplicate insertion index refuses','create',['a','b'],[0,0],null,'invalid ordered record field sequence');
add('missing insertion index refuses','create',['a','b'],[0],null,'invalid ordered record field sequence');
add('out of range insertion index refuses','create',['a'],[1],null,'invalid ordered record field sequence');
add('negative insertion index refuses','create',['a'],[-1],null,'invalid ordered record field sequence');
add('fractional insertion index refuses','create',['a'],[0.5],null,'invalid ordered record field sequence');
add('noncanonical field layout refuses','create',['b','a'],[0,1],null,'invalid ordered record field sequence');
add('duplicate field name refuses','create',['a','a'],[0,1],null,'invalid ordered record field sequence');
add('merge cannot manufacture missing field','merge',['a','b'],[[1]],null,'invalid ordered record field sequence');
add('merge cannot manufacture an invalid source position','merge',['a'],[[1]],null,'invalid ordered record source field');
add('noncanonical stored numeric order refuses','keys',['10','2'],[0,1],null,'invalid ordered record property order');
add('rebase requires every actual source field in target','rebase',['a'],[0],null,'an ordered record spread field is missing from its target',{sourceNames:['b'].map(units)});
add('rebase requires canonical actual source order','rebase',['10','2'],[0,1],null,'invalid ordered record spread source',{sourceNames:['10','2'].map(units)});
cases.push({name:'nonintegral UTF16 field unit refuses',operation:'create',names:[[65.5]],input:[0],expected:null,error:'invalid ordered record field sequence'});
cases.push({name:'out of range UTF16 field unit refuses',operation:'create',names:[[65536]],input:[0],expected:null,error:'invalid ordered record field sequence'});
writeProjectData(new URL('../../parity/fixtures/javascript-record-order.lino',import.meta.url),{
 schemaVersion:1,model:'canonical record layout with independent own-field order values 1',cases,
 versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
 sourceBasis:'https://tc39.es/ecma262/2025/multipage/ordinary-and-exotic-objects-behaviours.html#sec-ordinaryownpropertykeys',
 assertions:['ownOrderDistinctFromStructuralLayout','numericKeysFirstByNumericValue','StringKeysFirstInsertionIncludingOverwrites','exactUtf16NamesIncludingIsolatedSurrogates','spreadRebaseRetainsActualSourceOrder','everyOwnFieldPresentExactlyOnce','invalidSequenceNeverGetsReplacementFields','nativeFallibleSourceErrorParity','independentObjectKeysControlsForEveryValidCase','bothCodecsFourCompleteDefinitionEmissions'],
 boundary:'Unexecuted declarations. Dense exact UTF16 unit vectors and canonical nonnegative finite Number field positions (positive zero) describe this value protocol. Symbols/descriptors, dynamic add/delete and ordered source object/parameter/branch/alias integration remain required, as do all earlier requirements.',
});
