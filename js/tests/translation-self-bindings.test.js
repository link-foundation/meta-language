import { readProjectData, resolveProjectDataPath } from '../scripts/project-data.mjs';

import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { selfTranslate } from '../src/self-translation.js';
import { nativeRustChecksEnabled } from './support/native-rust-checks.js';

const fixture = readProjectData(new URL('../../parity/fixtures/self-translation-bindings.json', import.meta.url));
fixture.cases = fixture.cases.map((entry) => entry.module ? { ...entry, source: readFileSync(resolveProjectDataPath(new URL(`../../${entry.module}`, import.meta.url)), 'utf8') } : entry);
const [major,minor]=process.versions.node.split('.').map(Number);
assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node+'; >=22.15 <23 or >=24 <25');

test('sibling bindings share a checked scope, execute faithfully and restore their source', () => {
  const directory = nativeRustChecksEnabled() ? mkdtempSync(path.join(tmpdir(), 'self-translation-bindings-')) : null;
  try {
    for (const entry of fixture.cases) {
      const translated = selfTranslate(entry.source, 'JavaScript', 'Rust');
      assert.ok(translated.items.every(({ status }) => status === 'translated'), entry.name);
      assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, entry.source, entry.name);
      for (const observation of [entry, ...(entry.additionalCalls ?? [])]) {
        const name = observation === entry ? entry.name : `${entry.name}-${observation.name}`;
        const expected = execFileSync(process.execPath, ['--input-type=module', '-e', `${entry.source}\nconsole.log(${observation.call});`], { encoding: 'utf8', timeout: 10_000 });
        assert.equal(expected, observation.expected, name);
        if (!directory) continue;
        const source = path.join(directory, `${name}.rs`);
        const binary = path.join(directory, process.platform === 'win32' ? `${name}.exe` : name);
        writeFileSync(source, `${translated.code}\nfn main() { println!("{}", ${observation.rustCall}); }\n`);
        execFileSync('rustc', ['--edition', '2024', source, '-o', binary], { stdio: 'pipe', timeout: 60_000 });
        const actual = execFileSync(binary, [], { encoding: 'utf8', timeout: 10_000 });
        assert.equal(actual, expected, name);
      }
    }
  } finally { if (directory) rmSync(directory, { recursive: true, force: true }); }
});

test('a carried sibling and its caller stay carried without poisoning an independent function', () => {
  const source = '/** @returns {number} */\nfunction good() { return 7; }\n\nfunction unavailable() { return new Date(); }\n\n/** @returns {number} */\nexport function caller() { return unavailable(); }\n';
  const translated = selfTranslate(source, 'JavaScript', 'Rust');
  const functions = translated.items.filter(({ term }) => term === 'function_declaration' || term === 'export_statement');
  assert.deepEqual(functions.map(({ status }) => status), ['translated', 'carried', 'carried']);
  assert.ok(!translated.code.includes('pub fn unavailable('));
  assert.ok(!translated.code.includes('pub fn caller('));
  assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, source);
});

test('mutable and temporal-dead-zone bindings stay unbound while pure Number arithmetic translates', () => {
  for (const entry of fixture.refusals) {
    const translated = selfTranslate(entry.source, 'JavaScript', 'Rust');
    const functions = translated.items.filter(({ term }) => term === 'function_declaration' || term === 'export_statement');
    // CI 39960718444 exposed the obsolete refusal for the unchanged 1 + 2
    // input. Numeric module values now supply a checked source-owned static.
    assert.deepEqual(functions.map(({ status }) => status), [entry.name==='nonliteral-global'?'translated':'carried'], entry.name);
    assert.equal(selfTranslate(translated.code, 'Rust', 'JavaScript').code, entry.source, entry.name);
  }
});

test('the retained arithmetic binding input executes as three in both runtimes',()=>{
  const entry=fixture.refusals.find(row=>row.name==='nonliteral-global');assert.ok(entry);
  assert.equal(entry.source,'const LIMIT = 1 + 2;\n\n/** @returns {number} */\nexport function capture() { return LIMIT; }\n');
  const translated=selfTranslate(entry.source,'JavaScript','Rust');
  assert.ok(translated.items.every(item=>item.status==='translated'||item.status==='comment'));
  const expected=execFileSync(process.execPath,['--input-type=module','-e',entry.source+'\nconsole.log(capture());'],{encoding:'utf8',timeout:10000});
  assert.equal(expected,'3\n');
  if(!nativeRustChecksEnabled())return;
  const version=/^rustc (\d+)\.(\d+)\./u.exec(execFileSync('rustc',['--version'],{encoding:'utf8',timeout:10000}));
  assert.ok(version&&+version[1]===1&&+version[2]>=90&&+version[2]<100,'uncovered Rust; >=1.90 <1.100');
  const directory=mkdtempSync(path.join(tmpdir(),'self-translation-arithmetic-binding-'));
  try{
    const source=path.join(directory,'capture.rs'),binary=path.join(directory,process.platform==='win32'?'capture.exe':'capture');
    writeFileSync(source,translated.code+'\nfn main(){println!("{}",capture());}\n');
    execFileSync('rustc',['--edition','2024',source,'-o',binary],{stdio:'pipe',timeout:60000});
    assert.equal(execFileSync(binary,[],{encoding:'utf8',timeout:10000}),expected);
  }finally{rmSync(directory,{recursive:true,force:true});}
});
