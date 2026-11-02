// Deferred lossless/budget controls; do not execute during source drafting.
import test from 'node:test';import assert from 'node:assert/strict';
import {LinkNetwork} from '../src/network.js';
import {readBoundedCanonicalNetwork} from '../src/canonical-network-reader.js';
test('bounded asset reading retains exact records registered terms and self links',()=>{
 const original=new LinkNetwork(),term=original.insertField('source field'),value=original.insertObject('source value');original.insertFieldRelation(value,'source field',term);const text=original.toLino(),restored=readBoundedCanonicalNetwork(text);
 assert.equal(restored.toLino(),text);assert.equal(restored.insertField('source field').asU64(),term.asU64());assert.equal(LinkNetwork.fromLino(text).toLino(),text);
});
test('node and reference budgets stop before the excess record is inserted',()=>{
 const original=new LinkNetwork(),first=original.insertObject('first'),second=original.insertObject('second');original.insertLink([first,second]);const text=original.toLino(),insert=LinkNetwork.prototype.insertLinkWithOptionalId;let inserted=0;
 LinkNetwork.prototype.insertLinkWithOptionalId=function(...args){inserted+=1;return insert.apply(this,args);};
 try{assert.throws(()=>readBoundedCanonicalNetwork(text,{nodeLimit:1}),/node budget/);assert.equal(inserted,1);inserted=0;assert.throws(()=>readBoundedCanonicalNetwork(text,{referenceLimit:1}),/reference budget/);assert.equal(inserted,1);}finally{LinkNetwork.prototype.insertLinkWithOptionalId=insert;}
});
test('duplicates dangling links and invalid budgets refuse without partial results',()=>{
 const original=new LinkNetwork();original.insertObject('actual');const text=original.toLino();assert.throws(()=>readBoundedCanonicalNetwork(text+text),/duplicate/);
 const missing=new LinkNetwork();missing.insertLink([999]);assert.throws(()=>readBoundedCanonicalNetwork(missing.toLino()),/dangling/);
 assert.throws(()=>readBoundedCanonicalNetwork(text,{byteLimit:1}),/byte budget/);for(const limits of [{nodeLimit:0},{referenceLimit:Infinity},{byteLimit:-1}])assert.throws(()=>readBoundedCanonicalNetwork(text,limits),/positive safe integer/);
});
