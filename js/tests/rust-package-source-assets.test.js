// Future source-author controls; intentionally not executed during drafting.
import assert from 'node:assert/strict';
import test from 'node:test';
import {readRustPackageSourceIncludes} from '../scripts/rust-package-source-assets.mjs';
test('asset ownership comes from actual Rust macro operands',()=>{
 const source='// include_str!("comment")\nconst TEXT:&str=include_str!("data/value.lino");\nconst BYTES:&[u8]=std::include_bytes!(r#"data/value.bin"#);\n';
 const rows=readRustPackageSourceIncludes(source);
 assert.deepEqual(rows.map(({macro,path})=>[macro,path]),[['include_str','data/value.lino'],['include_bytes','data/value.bin']]);
 for(const row of rows)assert.ok(source.slice(row.start,row.end).includes(row.path));
});
test('macro-looking source text and byte data remain opaque',()=>{
 const source=String.raw`const A:&str=r##"include_str!("foreign")"##; const B:&[u8]=br##"include_bytes!("other")"##; /* include_str!("comment") */`;
 assert.deepEqual(readRustPackageSourceIncludes(source),[]);
});
test('unrepresented paths and computed macro operands refuse',()=>{
 for(const source of ['include_str!(concat!("data/","unknown"))','include_bytes!(path)','include_str!("/absolute")','include_str!("a\\\\b")'])assert.throws(()=>readRustPackageSourceIncludes(source),TypeError);
});
