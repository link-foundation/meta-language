import test from 'node:test';
import assert from 'node:assert/strict';
import {createTypedProgramLinks,decodeTypedProgramLinks,encodeTypedProgramLinks,emitTypedProgramLinks} from '../src/typed-program-links.js';
import {LinkNetwork} from '../src/network.js';
import {validateSourceSpans} from '../src/source-spans.js';

test('legacy main false metadata survives without authorizing Boolean source spans',()=>{
  const archive=createTypedProgramLinks('enum Color{Red}fn keep(value:Color)->Color{value}fn main(){}','Rust');
  const program=decodeTypedProgramLinks(archive.network,archive.root);
  program.main.span=false;
  const encoded=encodeTypedProgramLinks(program),network=LinkNetwork.fromLino(encoded.network.toLino());
  assert.equal(decodeTypedProgramLinks(network,encoded.root).main.span,false);
  for(const target of ['JavaScript','Rust'])assert.ok(emitTypedProgramLinks(network,encoded.root,target).code);
  assert.throws(()=>validateSourceSpans({span:false}),/source span is invalid/u);
  assert.throws(()=>validateSourceSpans({main:{span:true}}),/source span is invalid/u);
  assert.throws(()=>validateSourceSpans({main:{span:0}}),/source span is invalid/u);
  assert.throws(()=>validateSourceSpans({main:{span:{start:-0,end:1}}}),/source span is invalid/u);
});
