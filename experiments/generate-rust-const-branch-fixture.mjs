// Actual qualified definitions, with independent complete Boolean truth tables.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {fileURLToPath} from 'node:url';
import vm from 'node:vm';
import {writeProjectData,checkProjectData} from '../js/scripts/project-data.mjs';
import {qualifyFamilyLibraryBooleanFunctions} from '../js/scripts/portable-conditional-source-style.mjs';
import {createTypedProgramLinks,decodeTypedProgramLinks} from '../js/src/typed-program-links.js';
import {LinkNetwork} from '../js/src/network.js';
import {emitRust} from '../js/src/translation/emit-rust.js';
import {emitJavaScript} from '../js/src/translation/emit-javascript.js';
import {rustConstBranchPolicy} from './generate-rust-const-branch-rules.mjs';

const sha = source => createHash('sha256').update(source).digest('hex');
const specifications = [
  {name:'decide',parameters:2,ordinary:'pub fn decide(first:bool,second:bool)->bool{if first{second}else{!second}}',
    expected:[true,false,false,true]},
  {name:'nested',parameters:3,ordinary:'pub fn nested(first:bool,second:bool,third:bool)->bool{if first{if second{third}else{!third}}else if third{second}else{false}}',
    expected:[false,false,false,true,true,false,false,true]},
  {name:'combined',parameters:4,ordinary:'pub fn combined(first:bool,second:bool,third:bool,fourth:bool)->bool{if first&&!second{third||fourth}else{!third&&fourth}}',
    expected:[false,true,false,false,false,true,false,false,false,true,true,true,false,true,false,false]},
  {name:'literal',parameters:0,ordinary:'pub fn literal()->bool{if true{false}else{true}}',expected:[false]},
];
export function rustConstBranchInputs(count) {
  return Array.from({length:2 ** count},(_,position) => Array.from({length:count},(_,parameter) => Boolean(position & (1 << (count - parameter - 1)))));
}

const refused = [
  ['floating interface','const fn bad(value:f64)->bool{if true{true}else{value<0.0}}'],
  ['Boolean equality','const fn bad(value:bool)->bool{if value==true{true}else{false}}'],
  ['Boolean inequality','const fn bad(value:bool)->bool{if value!=false{true}else{false}}'],
  ['numeric comparison branch','const fn bad(value:bool)->bool{if value{true}else{1.0<2.0}}'],
  ['numeric condition','const fn bad(value:bool)->bool{if 1.0{value}else{false}}'],
  ['numeric result branch','const fn bad(value:bool)->bool{if value{true}else{1.0}}'],
  ['foreign call in unreachable branch','const fn bad(value:bool)->bool{if true{value}else{foreign()}}'],
  ['local binding','const fn bad(value:bool)->bool{if value{let local=true;local}else{false}}'],
  ['panic macro','const fn bad(value:bool)->bool{if value{panic!("no")}else{false}}'],
  ['explicit early return','const fn bad(value:bool)->bool{if value{return true;}else{false}}'],
  ['borrowed interface','const fn bad(value:&bool)->bool{if *value{true}else{false}}'],
  ['mutable interface','const fn bad(mut value:bool)->bool{if value{true}else{false}}'],
  ['custom primitive owner','enum bool{Value}const fn bad(value:bool)->bool{if true{true}else{false}}'],
  ['imported primitive owner','use core::primitive::bool as Flag;const fn bad(value:Flag)->bool{if value{true}else{false}}'],
  ['conditional compilation','#[cfg(any())]const fn bad(value:bool)->bool{if value{true}else{false}}'],
  ['ordinary function','fn bad(value:bool)->bool{if value{true}else{false}}'],
  ['nonempty main','const fn bad(value:bool)->bool{if value{true}else{false}}fn main(){println!("no");}'],
];

export function rustConstBranchFixture() {
  const model = rustConstBranchPolicy.model;
  const cases = specifications.map(specification => {
    const definition = qualifyFamilyLibraryBooleanFunctions(specification.ordinary);
    assert.ok(definition.startsWith('pub const fn '),'the genuine complete source style retains its const qualification');
    const source = definition+'\nfn main(){}';
    const packet = createTypedProgramLinks(source,'Rust',{rustConstFunctionModel:model});
    const network = LinkNetwork.fromLino(packet.network.toLino());
    const program = decodeTypedProgramLinks(network,packet.root);
    const rust = emitRust(program), javascript = emitJavaScript(program).text;
    const context = {};
    vm.runInNewContext(javascript,context,{timeout:1000});
    const inputs = rustConstBranchInputs(specification.parameters);
    assert.deepEqual(inputs.map(args => context[specification.name](...args)),specification.expected,'the original complete truth table is independent of archive generation');
    assert.ok(rust.definitions[0].startsWith('pub const fn '));
    return {...specification,definition,source,sourceSha256:sha(source),inputs,proof:program.items[0].sourceConstFunction,
      signature:program.items[0].sourceSignature,links:network.toLino(),root:Number(packet.root.value),rustDefinition:rust.definitions[0],javascript};
  });
  const invalid = refused.map(([name,body]) => {
    const source = body.includes('fn main()') ? body : body+'fn main(){}';
    try { createTypedProgramLinks(source,'Rust',{rustConstFunctionModel:model}); }
    catch (error) { return {name,source,kind:error.kind,reason:error.reason??error.message,span:error.span??null}; }
    throw new Error('an unsupported Boolean const source was admitted: '+name);
  });
  return {schemaVersion:1,model,interpretation:rustConstBranchPolicy.interpretation,cases,invalid};
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const path = new URL('./rust-const-branch-fixture.lino',import.meta.url), value = rustConstBranchFixture();
  if (process.argv.includes('--check')) {
    if (!checkProjectData(path,value)) throw new Error('Boolean const branch fixture is stale');
  } else writeProjectData(path,value);
}
