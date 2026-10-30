// Literal command/source declarations only; no represented program is parsed.
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const commands=[
 {args:[],expected:['error']},{args:['--help'],expected:['help']},
 {args:['input.source','--language','Markdown'],expected:['concrete source','input.source','Markdown','1','','','text']},
 {args:['input.source','--language','Rust','--typed','--first-id','9007199254740993','--binary'],expected:['typed source','input.source','Rust','9007199254740993','','','binary']},
 {args:['input.source','--language','Rust','--rust-source-owners','--source-version','1.90.0','--to','JavaScript'],expected:['Rust source owners','input.source','Rust','1','JavaScript','1.90.0','text']},
 {args:['input.source','--language','JavaScript','--to','Rust'],expected:['typed source','input.source','JavaScript','1','Rust','','text']},
 {args:['--language','Rust','--','--input'],expected:['concrete source','--input','Rust','1','','','text']},
 {args:['input.source','--language','Rust','--first-id','00001'],expected:['concrete source','input.source','Rust','00001','','','text']},
 ...[['input.source'],['input.source','--language'],['input.source','--language','Rust','--typed','--rust-source-owners'],['input.source','--language','Rust','--binary','--to','JavaScript'],['input.source','--language','Markdown','--typed'],['input.source','--language','Rust','--source-version','1.90.0'],['input.source','--language','JavaScript','--rust-source-owners'],['input.source','--language','Rust','--first-id','0'],['input.source','--language','Rust','--first-id','18446744073709551616'],['input.source','--language','Rust','--binary','--binary'],['input.source','--language','Rust','--language','JavaScript'],['input.source','--language','Rust','--to','Python'],['input.source','--language','Rust','second.source']].map(args=>({args,expected:['error']})),
];
const identities=[...['1','00001','9007199254740993','18446744073709551615'].map(value=>({value,expected:true})),...['','0','00000','-1','+1','1.0',' 1','1e3','18446744073709551616','999999999999999999999'].map(value=>({value,expected:false}))];
const source='export function result(){const left=3;return left+4;}',rust='pub fn result()->u8{7}';
const io=[
 {name:'CST is the default',args:['input.source','--language','Markdown'],source:'# Links\n\n```javascript\nconst value = 7;\n```\n',kind:'concrete',binary:false},
 {name:'exact CST binary keeps the original source',args:['input.source','--language','JavaScript','--first-id','9007199254740993','--binary'],source,kind:'concrete',binary:true},
 {name:'checked typed Links can use exact IDs',args:['input.source','--language','JavaScript','--typed','--first-id','9007199254740993'],source,kind:'typed',binary:false},
 {name:'typed binary retains its actual root',args:['input.source','--language','Rust','--typed','--binary','--first-id','9007199254740993'],source:rust,kind:'typed',binary:true},
 {name:'JavaScript source emits checked Rust',args:['input.source','--language','JavaScript','--to','Rust'],source,kind:'emission',binary:false,target:'Rust'},
 {name:'Rust source emits checked JavaScript',args:['input.source','--language','Rust','--to','JavaScript'],source:rust,kind:'emission',binary:false,target:'JavaScript',expected:'7'},
 {name:'Rust source owners retain a selected version',args:['input.source','--language','Rust','--rust-source-owners','--source-version','1.90.0','--to','JavaScript','--first-id','9007199254740993'],source:rust,kind:'emission',binary:false,target:'JavaScript',expected:'7'},
];
io.push(
 {name:'JavaScript source emits checked Lean',args:['input.source','--language','JavaScript','--to','Lean'],source,kind:'emission',binary:false,target:'Lean'},
 {name:'JavaScript source emits checked Rocq',args:['input.source','--language','JavaScript','--to','Rocq'],source,kind:'emission',binary:false,target:'Rocq'},
 {name:'Rust owner graph text retains a selected version',args:['input.source','--language','Rust','--rust-source-owners','--source-version','1.90.0','--first-id','9007199254740993'],source:rust,kind:'typed',binary:false},
 {name:'Rust owner graph binary retains a selected version',args:['input.source','--language','Rust','--rust-source-owners','--source-version','1.90.0','--binary','--first-id','9007199254740993'],source:rust,kind:'typed',binary:true},
);
writeProjectData(new URL('../../parity/fixtures/unsigned-source-command.lino',import.meta.url),{schemaVersion:1,model:'exact source command interpretation 1',commands,identities,io,versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},sourceDefinitionPrograms:['meta-language/source-program/unsigned-source-command-program'],boundary:'Unexecuted shared plan, process/file transport and actual future native/JavaScript products. Source authoring writes declarations only. CST remains default; checked typed source and source-owner/version choices are explicit. Unsupported source semantics retain the ordinary frontend refusal and all remaining whole-language/global-concept requirements.'});
