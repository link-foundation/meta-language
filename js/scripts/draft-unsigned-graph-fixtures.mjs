// Literal graph/command/fact declarations only. Never interpret represented data.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const roots=[];
function facts(name,ids,requested,expected,extra={}){roots.push({name,ids,types:ids.map(()=>'Semantic'),languages:ids.map(()=>'meta-language typed program'),terms:ids.map(()=>'typed program'),definitions:ids.map(()=>'1'),requested,limit:10,expected,...extra});}
facts('no declared root refuses',[],'',['error','a graph needs an actual declared schema-1 typed program root']);
facts('one exact root above safe Number range',['9007199254740993'],'',['root','9007199254740993']);
facts('largest exact root remains an actual identity',['18446744073709551615'],'',['root','18446744073709551615']);
facts('multiple roots require an explicit choice',['7','9007199254740993'],'',['error','a graph with multiple typed program roots needs an explicit root']);
facts('explicit choice retains original exact coordinates',['7','9007199254740993'],'9007199254740993',['root','9007199254740993']);
facts('explicit original spelling normalizes only coordinates',['7'],'00007',['root','7']);
facts('missing explicit identity refuses',['7'],'8',['error','the selected link is not an actual declared typed program root']);
facts('invalid requested identity refuses',['7'],'0',['error','a typed graph root needs an exact positive unsigned identity']);
facts('metadata columns remain aligned',['7'],'',['error','typed root facts need complete aligned metadata columns'],{types:[]});
facts('duplicate identity facts refuse',['7','7'],'',['error','typed root facts need canonical ordered unsigned identities']);
facts('unordered identity facts refuse',['8','7'],'',['error','typed root facts need canonical ordered unsigned identities']);
facts('noncanonical identity facts refuse',['07'],'',['error','typed root facts need canonical ordered unsigned identities']);
facts('out of range identity facts refuse',['18446744073709551616'],'',['error','typed root facts need canonical ordered unsigned identities']);
facts('root record budget is explicit',['7','8'],'',['error','typed root facts exceed their record limit'],{limit:1});
for(const [key,value]of [['types','Concept'],['languages','Rust'],['terms','not a root'],['definitions','2']])facts('a different '+key+' field is not a schema-1 root',['7'],'',['error','a graph needs an actual declared schema-1 typed program root'],{[key]:[value]});
const bounds=[
 {args:[0,0,33,1],expected:''},{args:[33,1,33,1],expected:''},
 {args:[34,1,33,1],expected:'saved graph input exceeds its byte limit'},
 {args:[0,2,33,1],expected:'saved graph input exceeds its record limit'},
 {args:[0,0,32,1],expected:'invalid saved graph resource bounds'},
 {args:[-1,0,33,1],expected:'saved graph input needs exact nonnegative byte and record counts'},
];
const surface=[...['','(1: 1)','(9007199254740993: 18446744073709551615)','(1: 1 (meta: (t: concept) (term: x%28y%29)))'].map(value=>({value,expected:''})),...['word','(name: 1)','(0: 1)','(1: 0)','(18446744073709551616: 1)','(1: (2: 2))','(1: 1 (meta: (term: "raw source")))','(1: 1'].map(value=>({value,expected:'saved graph text needs identified flat unsigned links with canonical metadata'}))];
const commands=[
 {args:[],expected:['error']},{args:['--help'],expected:['help']},
 {args:['saved.graph'],expected:['saved.graph','text','text','','','']},
 {args:['saved.graph','--input-binary','--binary'],expected:['saved.graph','binary','binary','','','']},
 {args:['saved.graph','--to','JavaScript','--root','9007199254740993','--rebase','1'],expected:['saved.graph','text','text','JavaScript','9007199254740993','1']},
 {args:['--','--saved'],expected:['--saved','text','text','','','']},
 ...[['saved.graph','--binary','--to','JavaScript'],['saved.graph','--root','1'],['saved.graph','--input-binary','--input-binary'],['saved.graph','--to','Rust','--root','0'],['saved.graph','--rebase','18446744073709551616'],['saved.graph','--to','Python'],['saved.graph','second.graph']].map(args=>({args,expected:['error']})),
];
const io=[
 {name:'exact text graph conversion retains a repeated self link',raw:'(9007199254740993: 9007199254740993 9007199254740993)',args:['saved.graph'],exitCode:0,kind:'graph'},
 {name:'text graph exports binary with exact coordinates',raw:'(9007199254740993: 9007199254740993)',args:['saved.graph','--binary'],exitCode:0,kind:'graph'},
 {name:'binary graph exports text',raw:'(9007199254740993: 9007199254740993)',args:['saved.graph','--input-binary'],exitCode:0,kind:'graph'},
 {name:'binary graph snapshot keeps allocator history and syntax cache',raw:'(7: 7)',history:true,args:['saved.graph','--input-binary','--binary'],exitCode:0,kind:'graph'},
 {name:'explicit rebase may allocate the final identity',raw:'(7: 7)',args:['saved.graph','--rebase','18446744073709551615','--binary'],exitCode:0,kind:'graph'},
 ...['JavaScript','Rust','Lean','Rocq'].map(target=>({name:'saved typed graph emits '+target,args:['saved.graph','--to',target],exitCode:0,kind:'emission',target,expected:target==='JavaScript'?'7':null})),
 {name:'edited text graph emits its actual value',edit:'9',args:['saved.graph','--to','JavaScript'],exitCode:0,kind:'emission',target:'JavaScript',expected:'9'},
 {name:'edited binary graph retains value through rebase',edit:'9',args:['saved.graph','--input-binary','--to','JavaScript','--root','$root','--rebase','1'],exitCode:0,kind:'emission',target:'JavaScript',expected:'9'},
 {name:'ambiguous saved roots refuse without a choice',multiple:true,args:['saved.graph','--to','JavaScript'],exitCode:1,kind:'failure',error:'a graph with multiple typed program roots needs an explicit root'},
 {name:'explicit second root is a separate valid entry point',multiple:true,args:['saved.graph','--to','JavaScript','--root','$second'],exitCode:0,kind:'emission',target:'JavaScript',expected:'7'},
 {name:'absent requested root refuses',args:['saved.graph','--to','JavaScript','--root','18446744073709551615'],exitCode:1,kind:'failure',error:'the selected link is not an actual declared typed program root'},
];
writeProjectData(new URL('../../parity/fixtures/unsigned-graph-command.lino',import.meta.url),{schemaVersion:1,model:'exact saved graph command and actual typed root selection 1',roots,bounds,surface,commands,io,typedSource:'pub fn result()->u8{7}',firstIdentity:'9007199254740993',versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024',typedProgramSchema:1,binary:'MLU64N01 wrapping MLDSNP02'},sourceDefinitionPrograms:['meta-language/source-program/unsigned-graph-command-program'],boundary:'Unexecuted root facts, identified-snapshot surface controls, saved-graph transports and future actual native/JavaScript products. Edited semantic values must drive emission. Multiple actual roots require explicit selection; source parsers and source restoration are never graph-read inputs. Coordinate rebasing proves no semantic equivalence. Wider whole-language, concept and dynamic graph requirements remain active.'});
