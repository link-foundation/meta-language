import { checkProjectData, writeProjectData } from './project-data.mjs';
import { fileURLToPath } from 'node:url';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';

const positives = [
  ['owned-recursive-box', 'enum Tree{Leaf(u64),Branch(Box<Tree>,Box<Tree>)} fn sum(tree:Tree)->u64{match tree{Tree::Leaf(value)=>value,Tree::Branch(left,right)=>sum(*left)+sum(*right)}} fn main(){println!("{}",sum(Tree::Branch(Box::new(Tree::Leaf(4)),Box::new(Tree::Leaf(5)))));}', ['9']],
  ['borrowed-constructor-field', 'enum Packet{Value(u64)} fn read(packet:&Packet)->u64{match packet{Packet::Value(value)=>*value}} fn main(){let packet=Packet::Value(7);println!("{}",read(&packet));}', ['7']],
  ['forward-box-field', 'fn sum(tree:Tree)->u64{match tree{Tree::Leaf(value)=>value,Tree::Branch(left,right)=>sum(*left)+sum(*right)}} enum Tree{Leaf(u64),Branch(Box<Tree>,Box<Tree>)} fn main(){println!("{}",sum(Tree::Branch(Box::new(Tree::Leaf(4)),Box::new(Tree::Leaf(5)))));}', ['9']],
  ['nested-reference-box', 'fn read(value:& &Box<u64>)->u64{***value} fn main(){let boxed=Box::new(11u64);println!("{}",read(&&boxed));}', ['11']],
  ['direct-box-allocation', 'fn value()->u64{*Box::new(13)} fn main(){println!("{}",value());}', ['13']],
  ['qualified-standard-box', 'fn read(value:std::boxed::Box<u64>)->u64{*value} fn main(){println!("{}",read(std::boxed::Box::new(17)));}', ['17']],
  ['let-shadow-origin', 'fn read(value:Box<u64>)->u64{let value=*value;value+1} fn main(){println!("{}",read(Box::new(3)));}', ['4']],
  ['module-qualified-field', 'mod nodes{pub enum Tree{Leaf(u64),Branch(Box<Tree>,Box<Tree>)}} use crate::nodes::Tree as T; fn sum(tree:T)->u64{match tree{T::Leaf(value)=>value,T::Branch(left,right)=>sum(*left)+sum(*right)}} fn main(){println!("{}",sum(T::Branch(Box::new(T::Leaf(4)),Box::new(T::Leaf(5)))));}', ['9']],
  ['borrowed-box-coercion', 'enum Packet{Value(u64)} fn read(packet:&Packet)->u64{match packet{Packet::Value(value)=>*value}} fn main(){let packet=Box::new(Packet::Value(19));println!("{}",read(&packet));}', ['19']],
  ['box-return-origin', 'fn boxed()->Box<u64>{Box::new(23)} fn read()->u64{*boxed()} fn main(){println!("{}",read());}', ['23']],
  ['borrowed-box-clone', '#[derive(Clone)] enum Packet{Value(u64)} fn copied(packet:&Box<Packet>)->Box<Packet>{packet.clone()} fn read(packet:&Packet)->u64{match packet{Packet::Value(value)=>*value}} fn main(){let packet=Box::new(Packet::Value(29));println!("{}",read(&*copied(&packet)));}', ['29']],
  ['forward-imported-field-type', 'mod nodes{pub enum Tree{Leaf(u64),Branch(Box<T>)} use crate::nodes::Tree as T;} fn sum(tree:nodes::Tree)->u64{match tree{nodes::Tree::Leaf(value)=>value,nodes::Tree::Branch(child)=>sum(*child)}} fn main(){println!("{}",sum(nodes::Tree::Branch(Box::new(nodes::Tree::Leaf(31)))));}', ['31']],
  ['boxed-data-branch-origin', 'enum Packet{Value(u64)} fn choose(left:Box<Packet>,right:Box<Packet>,flag:bool)->Box<Packet>{if flag{left}else{right}} fn read(packet:Packet)->u64{match packet{Packet::Value(value)=>value}} fn main(){println!("{}",read(*choose(Box::new(Packet::Value(37)),Box::new(Packet::Value(41)),true)));}', ['37']],
  ['outer-reference-box-coercion', 'fn read(value:& &u64)->u64{**value} fn main(){let value=43u64;let boxed=Box::new(&value);println!("{}",read(&boxed));}', ['43']],
  ['partial-outer-box-coercion', 'fn read(value:&Box<u64>)->u64{**value} fn main(){let boxed=Box::new(Box::new(53u64));println!("{}",read(&boxed));}', ['53']],
  ['diverging-box-branch', 'fn choose(value:Box<u64>,flag:bool)->Box<u64>{if flag{value}else{panic!("unused")}} fn main(){println!("{}",*choose(Box::new(59),true));}', ['59']],
  ['boxed-numeric-method-autodereference', 'fn read(value:Box<u64>)->u64{value.div_euclid(2)} fn main(){println!("{}",read(Box::new(61)));}', ['30']],
  ['boxed-compatible-equality', 'fn same(left:Box<u64>,right:Box<u64>)->bool{left==right} fn main(){println!("{}",same(Box::new(67),Box::new(67)));}', ['true']],
  ['boxed-display-value', 'fn text(value:Box<u64>)->String{value.to_string()} fn main(){println!("{}",text(Box::new(71)));}', ['71']],
  ['explicit-box-literal-pattern', 'fn read(value:Box<u64>)->u64{match *value{1=>1,_=>2}} fn main(){println!("{}",read(Box::new(1)));}', ['1']],
  ['explicit-box-fieldless-constructor-pattern', 'enum Flag{One,Two} fn read(value:Box<Flag>)->u64{match *value{Flag::One=>1,Flag::Two=>2}} fn main(){println!("{}",read(Box::new(Flag::One)));}', ['1']],
  ['declared-allocation-crate', 'extern crate alloc; fn take(value:alloc::boxed::Box<i32>)->i32{*value} fn main(){println!("{}",take(alloc::boxed::Box::new(79)));}', ['79']],
  ['standard-box-import', 'use std::boxed::Box; fn take(value:Box<i32>)->i32{*value} fn main(){println!("{}",take(Box::new(83)));}', ['83']],
  ['standard-box-import-alias', 'use std::boxed::Box as Heap; fn take(value:Heap<i32>)->i32{*value} fn main(){println!("{}",take(Heap::new(89)));}', ['89']],
  ['allocation-box-import-alias', 'extern crate alloc; use alloc::boxed::Box as Heap; fn take(value:Heap<i32>)->i32{*value} fn main(){println!("{}",take(Heap::new(97)));}', ['97']],
  ['borrowed-reference-wildcard-pattern', 'fn read(value:&u64)->u64{match value{&_=>73}} fn main(){println!("{}",read(&1));}', ['73']],
];
const main = ' fn main(){println!("x");}';
const negatives = [
  ['undeclared-allocation-crate', 'fn take(value:alloc::boxed::Box<i32>)->i32{*value}'],
  ['undeclared-allocation-import', 'use alloc::boxed::Box as Heap; fn take(value:Heap<i32>)->i32{*value}'],
  ['foreign-crate-declaration', 'extern crate other; fn value()->u64{1}'],
  ['allocation-crate-owner-collision', 'mod alloc{} extern crate alloc; fn take(value:alloc::boxed::Box<i32>)->i32{*value}'],
  ['duplicate-allocation-crate-declaration', 'extern crate alloc; extern crate alloc; fn value()->u64{1}'],
  ['shadowed-standard-box-import', 'mod std{pub mod boxed{pub enum Box{Value(u64)}}} use std::boxed::Box as Heap; fn take(value:Heap<u64>)->u64{*value}'],
  ['scalar-dereference', 'fn read(value:u64)->u64{*value}'],
  ['literal-dereference', 'fn read()->u64{*1}'],
  ['unknown-dereference', 'fn read()->u64{*missing}'],
  ['shadowed-let-origin', 'fn read(value:Box<u64>)->u64{let value=1;*value}'],
  ['shadowed-reference-let-origin', 'fn read(value:&u64)->u64{let value=1;*value}'],
  ['extra-dereference', 'fn read(value:Box<u64>)->u64{**value}'],
  ['forward-shadowed-box-owner', 'fn read(value:Box<u64>)->u64{*value} enum Box{Value(u64)}'],
  ['aliased-box-owner', 'enum Actual{Value(u64)} use crate::Actual as Box; fn read(value:Box<u64>)->u64{*value}'],
  ['shadowed-standard-box-owner', 'mod std{pub mod boxed{pub enum Box{Value(u64)}}} fn read(value:std::boxed::Box<u64>)->u64{*value}'],
  ['unproven-box-initializer', 'fn read()->u64{let value:Box<u64> =1;*value}'],
  ['unproven-reference-initializer', 'fn read()->u64{let value:&u64=1;*value}'],
  ['unproven-box-result', 'fn boxed()->Box<u64>{1} fn read()->u64{*boxed()}'],
  ['missing-owned-box-dereference', 'enum Tree{Leaf(u64),Branch(Box<Tree>)} fn sum(tree:Tree)->u64{match tree{Tree::Leaf(value)=>value,Tree::Branch(child)=>sum(child)}}'],
  ['case-origin-does-not-leak', 'enum Choice{Some(Box<u64>),None} fn read(choice:Choice,value:u64)->u64{match choice{Choice::Some(value)=>*value,Choice::None=>*value}}'],
  ['mutable-reference', 'fn read(value:&mut u64)->u64{*value}'],
  ['inner-reference-is-not-coercion-site', 'fn read(value:& &u64)->u64{**value} fn other()->u64{let boxed=Box::new(47u64);read(& &boxed)}'],
  ['or-pattern-ownership-disagreement', 'enum Choice{Boxed(Box<u64>),Plain(u64)} fn read(choice:Choice)->u64{match choice{Choice::Boxed(value)|Choice::Plain(value)=>value}}'],
  ['if-result-ownership-disagreement', 'fn read(value:Box<u64>,flag:bool)->u64{if flag{value}else{1}}'],
  ['match-result-ownership-disagreement', 'enum Flag{One,Two} fn read(value:Box<u64>,flag:Flag)->u64{match flag{Flag::One=>value,Flag::Two=>1}}'],
  ['or-pattern-missing-owned-binder', 'enum Choice{Boxed(Box<u64>),Empty} fn read(choice:Choice,value:u64)->u64{match choice{Choice::Empty|Choice::Boxed(value)=>value}}'],
  ['or-pattern-first-empty-alternative', 'fn read(value:u64)->u64{match value{_|other=>1}}'],
  ['boxed-arithmetic-operand', 'fn read(value:Box<u64>)->u64{value+1}'],
  ['boxed-negation-operand', 'fn read(value:Box<i64>)->i64{-value}'],
  ['boxed-numeric-cast-source', 'fn read(value:Box<u64>)->u64{value as u64}'],
  ['boxed-numeric-cast-target', 'fn read(value:u64)->u64{value as Box<u64>}'],
  ['boxed-incompatible-comparison', 'fn read(value:Box<u64>)->bool{value==1}'],
  ['boxed-boolean-condition', 'fn read(value:Box<bool>)->u64{if value{1}else{2}}'],
  ['boxed-zero-field-constructor-pattern', 'enum Flag{One,Two} fn read(value:Box<Flag>)->u64{match value{Flag::One=>1,Flag::Two=>2}}'],
  ['boxed-literal-pattern', 'fn read(value:Box<u64>)->u64{match value{1=>1,_=>2}}'],
  ['operator-uncertainty-through-display', 'fn read(value:Box<u64>)->String{(value+1).to_string()}'],
  ['owned-reference-wildcard-pattern', 'fn read(value:u64)->u64{match value{&_=>1}}'],
  ['owned-reference-constructor-pattern', 'enum Packet{Value(u64)} fn read(packet:Packet)->String{match packet{&Packet::Value(value)=>value.to_string()}}'],
  ['boxed-standard-string-conversion', 'fn read(value:Box<u64>)->String{String::from(value)}'],
  ['operator-uncertainty-through-numeric-method', 'fn read(value:Box<u64>)->u64{(value+1).div_euclid(2)}'],
  ['operator-uncertainty-through-string-reference', 'fn read(value:Box<u64>)->String{(value+1).as_str().to_string()}'],
];

export function rustSourceIndirectionFixture() {
  const cases = positives.map(([id, source, output]) => {
    const surface = parseRust(source); const program = checkProgram(surface);
    return { id, source, output, surface: JSON.parse(JSON.stringify(surface)), javascript: emitJavaScript(program).text, rust: emitRust(program).text };
  });
  const invalid = negatives.map(([id, body]) => {
    const source = body + main;
    try { checkProgram(parseRust(source)); } catch (error) {
      return { id, source, kind: error.kind, reason: error.reason, span: error.span ?? null };
    }
    throw new Error(`source admission unexpectedly accepted ${id}`);
  });
  return { schemaVersion: 1, id: 'rust-source-indirection', cases, invalid };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const file = new URL('../../parity/fixtures/rust-source-indirection.json', import.meta.url);
  const value = rustSourceIndirectionFixture();
  if (process.argv.includes('--check')) {
    checkProjectData(file, value);
  } else writeProjectData(file, value);
}
