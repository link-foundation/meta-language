import { readFileSync, writeFileSync } from 'node:fs';
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
];
const main = ' fn main(){println!("x");}';
const negatives = [
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
];

export function rustSourceIndirectionFixture() {
  const cases = positives.map(([id, source, output]) => {
    const surface = parseRust(source); const program = checkProgram(surface);
    return { id, source, output, surface: JSON.parse(JSON.stringify(surface)), javascript: emitJavaScript(program).text, rust: emitRust(program).text };
  });
  const invalid = negatives.map(([id, body]) => {
    const source = body + main;
    try { parseRust(source); } catch (error) {
      return { id, source, reason: error.reason, span: error.span };
    }
    throw new Error(`source admission unexpectedly accepted ${id}`);
  });
  return { schemaVersion: 1, id: 'rust-source-indirection', cases, invalid };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const file = new URL('../../parity/fixtures/rust-source-indirection.json', import.meta.url);
  const text = JSON.stringify(rustSourceIndirectionFixture(), null, 2) + '\n';
  if (process.argv.includes('--check')) {
    if (readFileSync(file, 'utf8') !== text) throw new Error('Rust source indirection fixtures are stale');
  } else writeFileSync(file, text);
}
