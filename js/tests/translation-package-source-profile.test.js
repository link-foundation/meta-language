import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {generateRelativeCharacterSelfTranslation} from '../../experiments/grammar-outcome-network/native-transport/inverse/generate-relative-character-self-translation.mjs';
const root=new URL('../../',import.meta.url);

test('version qualified translator package provenance preserves every original byte when Windows enables automatic line endings',()=>{
  const fixture=generateRelativeCharacterSelfTranslation({check:true});
  const source=readFileSync(new URL(fixture.package.file,root));
  assert.equal(createHash('sha256').update(source).digest('hex'),fixture.package.sha256);
  const windows=spawnSync('git',['-c','core.autocrlf=true','cat-file','--filters','HEAD:'+fixture.package.file],
    {cwd:root,maxBuffer:1024*1024});
  const original=spawnSync('git',['cat-file','blob','HEAD:'+fixture.package.file],{cwd:root,maxBuffer:1024*1024});
  assert.equal(windows.status,0,String(windows.stderr));assert.equal(original.status,0,String(original.stderr));
  assert.deepEqual(windows.stdout,original.stdout);
  assert.deepEqual(windows.stdout,source);
  assert.notEqual(createHash('sha256').update(Buffer.from(source.toString().replaceAll('\n','\r\n'))).digest('hex'),fixture.package.sha256);
});

test('registered native storage ownership remains exact when Windows enables automatic source line endings',()=>{
  const files=['rust/tests/unit/grammar_outcome_static_borrows.rs','rust/tests/unit/mod.rs'];
  const sources=files.map(file=>{
    const original=spawnSync('git',['cat-file','blob','HEAD:'+file],{cwd:root,maxBuffer:1024*1024});
    const windows=spawnSync('git',['-c','core.autocrlf=true','cat-file','--filters','HEAD:'+file],{cwd:root,maxBuffer:1024*1024});
    assert.equal(original.status,0,String(original.stderr));assert.equal(windows.status,0,String(windows.stderr));
    assert.deepEqual(windows.stdout,original.stdout);assert.deepEqual(windows.stdout,readFileSync(new URL(file,root)));
    return windows.stdout.toString();
  });
  assert.ok(sources[0].includes('#[path = "../../../experiments/grammar-outcome-network/native-transport/inverse/static-borrows.rs"]\nmod storage;'));
  assert.ok(sources[1].split('\n').includes('mod grammar_outcome_static_borrows;'));
});
