// Normative input declarations only. No frontend, target or oracle is run.
import { writeProjectData } from './project-data.mjs';
if (process.argv.slice(2).join(' ') !== '--draft') throw new TypeError('use --draft to author fixture declarations');
const types = [
  { name:'i8', bits:8, signed:true, values:['-128','-1','0','1','127'] },
  { name:'u8', bits:8, signed:false, values:['0','1','7','255'] },
  { name:'i16', bits:16, signed:true, values:['-32768','-1','0','1','32767'] },
  { name:'u16', bits:16, signed:false, values:['0','1','7','65535'] },
  { name:'i32', bits:32, signed:true, values:['-2147483648','-1','0','1','2147483647'] },
  { name:'u32', bits:32, signed:false, values:['0','1','7','4294967295'] },
  { name:'i64', bits:64, signed:true, values:['-9223372036854775808','-1','0','1','9223372036854775807'] },
  { name:'u64', bits:64, signed:false, values:['0','1','7','18446744073709551615'] },
  { name:'i128', bits:128, signed:true, values:['-170141183460469231731687303715884105728','-1','0','1','170141183460469231731687303715884105727'] },
  { name:'u128', bits:128, signed:false, values:['0','1','7','340282366920938463463374607431768211455'] },
  { name:'bool', bits:0, signed:false, values:['false','true'] },
];
const operations = [{name:'conjoin',symbol:'&'},{name:'disjoin',symbol:'|'},{name:'exclude',symbol:'^'},{name:'complement',symbol:'!'}];
const functions = types.flatMap(type => operations.map(operation => ({ type:type.name, name:'f_' + type.name + '_' + operation.name, symbol:operation.symbol,
  source:'pub fn f_' + type.name + '_' + operation.name + '(left:' + type.name + ',right:' + type.name + ')->' + type.name + '{'
    + (operation.symbol === '!' ? '!left' : 'left ' + operation.symbol + ' right') + '}\n' })));
writeProjectData(new URL('../../parity/fixtures/rust-primitive-bitwise.lino', import.meta.url), {
  schemaVersion:1, model:'total Rust Boolean and fixed integer bitwise product 1', types, functions,
  source:functions.map(row=>row.source).join('\n'),
  precedenceSource:'pub fn precedence(left:u8,right:u8)->u8{left | right ^ 3u8 & 5u8}\n',
  contextualSource:'pub fn contextual()->u64{1 & 2}\n',
  eagerSource:'pub fn fail()->bool{panic!("eager right")}\npub fn eager_and()->bool{false & fail()}\npub fn eager_or()->bool{true | fail()}\npub fn eager_xor()->bool{false ^ fail()}\n',
  versions:{JavaScript:'Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024'},
  boundary:'Every declared primitive type/operator/input pair must execute against original Rust and translated JavaScript/native Rust, with complete four-target graph parity and both codecs. Contextual literal typing, precedence and eager Boolean failure are independent controls. No execution observation. Shifts, trait overloads and compound Rust local state remain separate obligations.',
});
