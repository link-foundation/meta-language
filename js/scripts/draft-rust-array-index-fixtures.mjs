// Author future products and source facts only; never execute a source case.
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {writeProjectData} from './project-data.mjs';
if(process.argv.slice(2).join(' ')!=='--draft')throw new TypeError('use --draft');
const cases=[];
function add(name,body,value='7',ret='i64',prefix=''){cases.push({name,module:'array_index_case_'+cases.length,source:prefix+'pub fn result()->'+ret+'{'+body+'}',value,abort:null});}
for(const type of ['i8','i16','i32','i64','u8','u16','u32','u64','isize','usize'])add('direct '+type+' elements retain their actual source width','let values=vec![7'+type+'];values[0]','7',type);
add('a Boolean read retains its actual value','let values=vec![true,false];if values[0]{7}else{8}');
add('a binary64 read retains its actual value','let values=vec![7.0f64];values[0]','7','f64');
add('a Unit read is a direct Copy value','let values=vec![()];let value=values[0];7');
add('an inferred temporary retains separate element Copy evidence','vec![7][0]');
add('an inferred binding retains separate element Copy evidence','let values=vec![3,4];values[0]+values[1]');
add('a usize suffix is actual index-owner evidence','let values=vec![3i64,7i64];values[1usize]');
add('a typed usize local is actual index-owner evidence','let values=vec![3i64,7i64];let index:usize=1;values[index]');
add('a usize parameter is distinct from an equal-width u64','let values=vec![3i64,7i64];read(&values,1)','7','i64','fn read(values:&[i64],index:usize)->i64{values[index]}');
add('an actual usize cast retains its target source owner','let values=vec![7i64];values[0u32 as usize]');
add('an untyped literal is checked directly in the usize domain','let values=vec![7i64];values[0 as usize]');
add('an immutable vector reference retains its element facts','let values=vec![7i64];read(&values)','7','i64','fn read(values:&Vec<i64>)->i64{values[0]}');
add('an immutable slice retains its actual standard owner','let values=vec![7i64];read(&values)','7','i64','fn read(values:&[i64])->i64{values[0]}');
add('nested immutable vector references are read by automatic dereference','let values=vec![7i64];read(&&values)','7','i64','fn read(values:&&Vec<i64>)->i64{values[0]}');
add('a standard Box vector is read without moving the vector','let values=Box::new(vec![7i64]);values[0]');
add('a boxed slice reference keeps its immutable layers','let values=vec![7i64];read(Box::new(&values))','7','i64','fn read(values:Box<&Vec<i64>>)->i64{values[0]}');
add('a scalar element copy leaves the actual vector available','let values=vec![3i64,4i64];let first=values[0];first+values[1]');
add('an index read copies a borrowed scalar into an ordinary fresh result','let input=vec![7i64];let output=copy(&input);output[0]','7','i64','fn copy(values:&[i64])->Vec<i64>{vec![values[0]]}');
add('a temporary returned vector retains its checked result owner','values()[0]','7','i64','fn values()->Vec<i64>{vec![7]}');
add('a source function supplies its actual usize result','let values=vec![3i64,7i64];values[position()]','7','i64','fn position()->usize{1}');
add('an index source is evaluated once before the checked read','let values=vec![7i64];values[position()]','7','i64','fn position()->usize{0}');
const receiverReason='Rust scalar array index: the receiver needs a proven standard vector or slice of direct scalar Copy elements';
const operandReason='Rust scalar array index: the index needs an unsuffixed integer literal or a proven direct usize source owner';
const refusals=[];
for(const [name,index]of [['u64 is not a usize owner','0u64'],['u32 is not a usize owner','0u32'],['signed indices retain their source refusal','0i64'],['binary64 indices retain their source refusal','0f64'],['Boolean indices retain their source refusal','false'],['a borrowed index requires explicit dereference','&0usize'],['unproved arithmetic needs its source width','0+0']])refusals.push({name,source:'fn bad(values:&[i64])->i64{values['+index+']}fn main(){}',kind:'unsupported',reason:operandReason});
refusals.push({name:'a String element cannot be moved out of an indexed slice',source:'fn bad(values:&[String])->String{values[0]}fn main(){}',kind:'unsupported',reason:receiverReason});
const receivers=[];
for(const [kind,element,model,proven,layers,inferredCopy,accepted]of [
 ['array','fixed','owned vector',true,true,false,true],['array','float','immutable slice',true,true,false,true],['array','bool','owned vector',true,true,false,true],['array','unit','immutable slice',true,true,false,true],['','', 'owned vector',true,true,true,true],
 ['array','string','owned vector',true,true,false,false],['array','named','immutable slice',true,true,true,false],['array','fixed','unproved array',true,true,true,false],['array','fixed','owned vector',false,true,true,false],['array','fixed','owned vector',true,false,true,false],['','','owned vector',true,true,false,false],['string','','owned vector',true,true,true,false],
])receivers.push({kind,element,model,proven,layers,inferredCopy,expected:accepted?'':receiverReason.slice('Rust scalar array index: '.length)});
const operands=[];
for(const [literal,kind,bits,signed,owner,wrappers,proven,accepted]of [
 [true,'',0,false,'',0,true,true],[false,'fixed',64,false,'usize',0,true,true],
 [false,'fixed',64,false,'u64',0,true,false],[false,'fixed',32,false,'usize',0,true,false],[false,'fixed',64,true,'usize',0,true,false],[false,'float',64,false,'usize',0,true,false],[false,'fixed',64,false,'usize',1,true,false],[false,'fixed',64,false,'usize',0,false,false],[false,'',0,false,'',0,true,false],
])operands.push({literal,kind,bits,signed,owner,wrappers,proven,expected:accepted?'':operandReason.slice('Rust scalar array index: '.length)});
const bounds=[{name:'the maximum source usize is retained exactly before bounds checking',source:'pub fn read(values:&[i64])->i64{values[18446744073709551615]}fn main(){}',index:'18446744073709551615'},{name:'a dynamic usize is checked against the actual receiver length',source:'pub fn read(values:&[i64],index:usize)->i64{values[index]}fn main(){}',index:'1'}];
const revision='afb637a2',history=['js/src/translation/rust.js','js/src/translation/rust-source-origins.js','rust/src/translation/rust/expressions.rs','rust/src/translation/rust/expressions/control.rs','rust/src/translation/rust/source_origins.rs'].map(path=>{const bytes=execFileSync('git',['show',revision+':'+path],{maxBuffer:4*1024*1024});return{path,revision,bytes:bytes.length,sha256:createHash('sha256').update(bytes).digest('hex'),source:bytes.toString('utf8')};});
writeProjectData(new URL('../../parity/fixtures/rust-array-indices.lino',import.meta.url),{schemaVersion:1,model:'ordinary Rust scalar array reads 1',cases,refusals,receivers,operands,bounds,history,source:cases.map(row=>'pub mod '+row.module+'{\n'+row.source+'\n}').join('\n')+'\nfn main(){}\n',versions:{JavaScript:'ECMAScript 2025; Node.js >=22.15.0 <23.0.0 or >=24.0.0 <25.0.0',Rust:'>=1.90.0 <1.100.0; edition 2024; target_pointer_width=64'},references:['https://doc.rust-lang.org/reference/expressions/array-expr.html#array-and-slice-indexing-expressions','https://doc.rust-lang.org/std/primitive.usize.html'],assertions:['actualStandardVectorAndSliceReceiversRetainElementCopyFacts','sourceUsizeOwnersDifferFromEqualWidthUnsignedTypes','literalCoordinatesEnterUnsigned64WithoutFloatingPointRounding','boundsChecksAreNeverMarkedProvenOrDiscarded','bothActualShippedDecisionsCompleteOriginalProductsBothCodecsAndFourEmissions'],boundary:'Unexecuted future controls. Direct scalar Copy elements only; standard vectors/slices with immutable reference and Box layers. Unsuffixed integer literals, direct usize values and actual usize casts are admitted. Inferred arithmetic without an established usize owner, index traits, fixed arrays, subslices/ranges, string and product moves, mutable element places, general source aliases and exact platform panic payloads remain obligations. Large indices retain their exact domain before the existing checked array operation.'});
