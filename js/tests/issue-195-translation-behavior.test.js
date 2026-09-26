import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdtemp, readFile, rm, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';

import { translateProgram } from '../src/program-translation.js';
import { ISSUE_195_FIXTURE_FILES, recordIssue195Observations } from './support/issue-195-observations.js';

const corpusBytes = await readFile(new URL('../../parity/fixtures/four-language-conformance.json', import.meta.url));
const cases = JSON.parse(corpusBytes).translationBehaviorCases;

test('Rust function translation exports an executable JavaScript function', async () => {
  const fixture = cases.find(({ sourceLanguage, targetLanguage }) =>
    sourceLanguage === 'Rust' && targetLanguage === 'JavaScript');
  const translation = translateProgram(fixture.source, 'Rust', 'JavaScript');
  const module = await import(`data:text/javascript,${encodeURIComponent(translation.code)}`);
  assert.equal(typeof module[fixture.export], 'function');
  assert.equal(module[fixture.export](), fixture.expectedResult);
  const constantMutation = translation.code.replace('return 42;', 'return 1;');
  const mutatedModule = await import(`data:text/javascript,${encodeURIComponent(constantMutation)}`);
  assert.notEqual(mutatedModule[fixture.export](), fixture.expectedResult);
  recordIssue195Observations({
    requirementId: 'I195-TRANSLATE-rust-to-javascript',
    suffix: 'positive',
    fixtureId: 'planned:translation:Rust:JavaScript',
    fixtureFile: ISSUE_195_FIXTURE_FILES.fourLanguage,
    assertions: ['realTargetArtifact', 'nativeTargetValidation', 'semanticPreservationChecked'],
    testName: 'Rust function translation exports an executable JavaScript function',
  });
});

test('JavaScript console output translation executes in Rust and detects effect erasure', async () => {
  const fixture = cases.find(({ sourceLanguage, targetLanguage }) =>
    sourceLanguage === 'JavaScript' && targetLanguage === 'Rust');
  const translation = translateProgram(fixture.source, 'JavaScript', 'Rust');
  assert.match(translation.code, /pub fn main\(\)/u);
  assert.match(translation.code, /println!\("42"\)/u);
  const directory = await mkdtemp(path.join(tmpdir(), 'issue-195-js-to-rust-'));
  try {
    const source = path.join(directory, 'translated.rs');
    const executable = path.join(directory, `translated${process.platform === 'win32' ? '.exe' : ''}`);
    await writeFile(source, translation.code);
    execFileSync('rustc', ['--edition', '2024', '--crate-type', 'bin', '-o', executable, source]);
    assert.equal(execFileSync(executable).toString(), fixture.expectedStdout);
    await writeFile(source, translation.code.replace('println!("42")', 'println!("0")'));
    execFileSync('rustc', ['--edition', '2024', '--crate-type', 'bin', '-o', executable, source]);
    assert.notEqual(execFileSync(executable).toString(), fixture.expectedStdout);
    recordIssue195Observations({
      requirementId: 'I195-TRANSLATE-javascript-to-rust',
      suffix: 'positive',
      fixtureId: 'planned:translation:JavaScript:Rust',
      fixtureFile: ISSUE_195_FIXTURE_FILES.fourLanguage,
      assertions: ['realTargetArtifact', 'nativeTargetValidation', 'semanticPreservationChecked'],
      testName: 'JavaScript console output translation executes in Rust and detects effect erasure',
    });
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});

test('unimplemented source forms remain marked as transport only', () => {
  const translation = translateProgram('console.log(9007199254740993);', 'JavaScript', 'Rust');
  assert.equal(translation.contract.support, 'portable-encoding');
  assert.match(translation.contract.obligation, /semantic translation is not implemented/u);
  assert.doesNotMatch(translation.code, /pub fn main\(\)/u);
});

test('Rust identifiers reserved by strict JavaScript stay transport only', () => {
  const translation = translateProgram('pub fn public() -> u32 { 42 }', 'Rust', 'JavaScript');
  assert.equal(translation.contract.support, 'portable-encoding');
  assert.doesNotMatch(translation.code, /export function public/u);
});
