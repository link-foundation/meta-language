// Input/source declarations only; no shifted value or failure is evaluated.
import{readProjectData,writeProjectData}from'./project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft to author declarations');
const types=readProjectData(new URL('../../parity/fixtures/rust-primitive-bitwise.lino',import.meta.url)).types.filter(type=>type.name!=='bool');
const countTypes=[
 {name:'i8',values:['-128','-1','0','1','7','8','15','16','31','32','63','64','127']},
 {name:'u8',values:['0','1','7','8','15','16','31','32','63','64','127','128','129','255']},
 {name:'i32',values:['-1','0','1','7','8','15','16','31','32','63','64','127','128','129','2147483647']},
 {name:'u32',values:['0','1','7','8','15','16','31','32','63','64','127','128','129','4294967295']},
 {name:'i128',values:['-170141183460469231731687303715884105728','-1','0','1','7','8','15','16','31','32','63','64','127','128','129','170141183460469231731687303715884105727']},
 {name:'u128',values:['0','1','7','8','15','16','31','32','63','64','127','128','129','340282366920938463463374607431768211455']},
];
const functions=types.flatMap(type=>countTypes.flatMap(count=>['left','right'].map(direction=>({name:'shift_'+type.name+'_'+count.name+'_'+direction,
 type:type.name,countType:count.name,direction,source:'pub fn shift_'+type.name+'_'+count.name+'_'+direction+'(value:'+type.name+',count:'+count.name+')->'+type.name
 +'{value '+(direction==='left'?'<<':'>>')+' count}\n'}))));
writeProjectData(new URL('../../parity/fixtures/rust-shifts.lino',import.meta.url),{
 schemaVersion:1,model:'Rust checked primitive shift complete boundary product 1',types,countTypes,functions,source:functions.map(row=>row.source).join('\n'),
 precedenceSource:'pub fn shift_precedence(value:u8,count:i8)->u8{value + 1u8 << count & 7u8}\n',
 contextualSource:'pub fn shift_contextual()->u64{1 << 2}\n',
 versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; -C overflow-checks=on'},
 boundary:'Every declared value/count/direction/type cell must execute. Negative and oversized counts retain abort messages; valid shifts retain bit truncation and sign extension. Compare original Rust, emitted Rust and translated JavaScript; compare all four complete emissions and both codecs. This is a declaration, not an execution observation. Overflow-checks-off profiles and overloaded shifts remain separate obligations.',
});
