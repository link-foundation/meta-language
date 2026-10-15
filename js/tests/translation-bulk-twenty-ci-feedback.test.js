import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync, readFileSync} from 'node:fs';
import {posix} from 'node:path';
import test from 'node:test';
import {readProjectData} from '../scripts/project-data.mjs';
import {readCompilerFeedbackCorrections} from '../scripts/compiler-style-feedback.mjs';

const root = new URL('../../', import.meta.url);
const receipt = readProjectData(new URL('parity/self-translation/ci-38698267304-reviewed-diagnostics.lino', root));

test('reviewed bulk twenty diagnostics retain the exact failed source and no invented machine suggestions', () => {
  assert.equal(createHash('sha256').update(readFileSync(new URL('parity/self-translation/ci-38698267304-reviewed-diagnostics.lino', root))).digest('hex'), '32d3b46dcd3e38a5241ea843a7c4dcced520012a153e43dd5d4222b49851ec5d');
  assert.equal(receipt.head, '21b09298ea44723fc79f8621ec6e22dcc15c9d37');
  assert.equal(receipt.execution, '50bb2b4d0f33fc1ab5ace38dc6d2029c6167c2e0');
  assert.equal(receipt.diagnostics.length, 18);
  assert.equal(receipt.sources.length, 15);
  assert.deepEqual(receipt.logs.map(log => log.sha256), [
    'c946c1b77cb3d3aa822d8ee4ad1ebc21f7a88677a2534b141ae02fb3b4cf9a8d',
    '3b7a9c5d9944b17c8e15ce48f66894a6f20df7216dcbb0ad17533a00dd07994d',
  ]);
  for (const source of receipt.sources) {
    const bytes = Buffer.from(source.text, 'utf8');
    assert.equal(source.bytes, bytes.length);
    assert.equal(source.sha256, createHash('sha256').update(bytes).digest('hex'));
    assert.equal(source.text, bytes.toString('utf8'));
  }
  const corrections = readCompilerFeedbackCorrections({feedback: receipt, allowedPaths: receipt.sources.map(source => source.path)});
  assert.deepEqual(corrections.actions, []);
  assert.equal(corrections.spans, 0);
  assert.match(receipt.formatDiagnostic, /failed to resolve mod `strings`/u);
});

test('the actual frontend rules imported by the test shim resolve every generated child explicitly', () => {
  const file = new URL('rust/src/translation/frontend_rules.rs', root);
  const source = readFileSync(file, 'utf8');
  const children = [...source.matchAll(/(?:#\[path = "([^"]+)"\]\n)?mod (\w+);/gu)];
  assert.deepEqual(children.map(match => match[2]), ['strings', 'rust_strings', 'continuations', 'continuations_2']);
  for (const [, path, name] of children) {
    assert.equal(path, `frontend_rules/${name}.rs`);
    assert.ok(existsSync(new URL(path, file)), `the actual generated ${name} child exists`);
  }
});

test('the full authentic native suite receipt resolves four module failures and every missing declaration through actual child reexports', () => {
  const bytes = readFileSync(new URL('parity/self-translation/ci-38698267304-native-suite-diagnostics.lino', root));
  assert.equal(createHash('sha256').update(bytes).digest('hex'), '44fa3ef71734ba9fc29bf5e0848c5df5233e212b2f25d41516564abe09387ac7');
  const suite = readProjectData(new URL('parity/self-translation/ci-38698267304-native-suite-diagnostics.lino', root));
  assert.equal(suite.head, receipt.head);
  assert.equal(suite.execution, receipt.execution);
  assert.equal(suite.runId, '38698267304');
  assert.equal(suite.jobId, '116341491230');
  assert.equal(suite.sourceContext.receiptFile, 'parity/self-translation/ci-38698267304-reviewed-diagnostics.lino');
  assert.equal(suite.sourceContext.receiptSha256, '32d3b46dcd3e38a5241ea843a7c4dcced520012a153e43dd5d4222b49851ec5d');
  assert.equal(suite.sourceContext.receiptBytes, readFileSync(new URL(suite.sourceContext.receiptFile, root)).length);
  assert.deepEqual(suite.logs.map(log => [log.bytes, log.sha256]), [
    [109323, 'b8a4138f9fa4b55c3ea17d634fd02f834f2f72ffa28662e75325b0c8f0ae8440'],
    [58750, 'eafda4419e228590d40aff5d423569f2ec327055c36c522ec0f3d24aa6eae6e2'],
  ]);
  assert.equal(suite.logs[1].artifact.id, '11961455826');
  assert.equal(suite.logs[1].artifact.bytes, 8454);
  assert.equal(suite.logs[1].artifact.sha256, '43c874deb73999c30e0c7800bbf63ccf687b3dd975293ab07e407e01ccfce30a');
  const logBytes = Buffer.from(suite.logs[1].text, 'utf8');
  assert.equal(logBytes.length, suite.logs[1].bytes);
  assert.equal(createHash('sha256').update(logBytes).digest('hex'), suite.logs[1].sha256);
  const plain = suite.logs[1].text.replace(/\u001b\[[0-?]*[ -/]*[@-~]/gu, '');
  assert.equal([...plain.matchAll(/^error\[E\d+\]:/gmu)].length, 57);
  assert.equal(suite.diagnostics.length, 57);
  assert.deepEqual(suite.sources[0], receipt.sources.find(source => source.path === 'rust/src/translation/frontend_rules.rs'));
  assert.equal(suite.sources.length, 3);
  for (const source of suite.sources) {
    const bytes = Buffer.from(source.text, 'utf8');
    assert.equal(bytes.length, source.bytes);
    assert.equal(createHash('sha256').update(bytes).digest('hex'), source.sha256);
  }
  const currentFile = new URL(suite.reviewedCorrection.root, root);
  const current = readFileSync(currentFile, 'utf8');
  const children = new Map([...current.matchAll(/#\[path = "([^"]+)"\]\s*mod (\w+);\s*pub use \2::\*;/gu)]
    .map(([, path, name]) => [name, {path, text: readFileSync(new URL(path, currentFile), 'utf8')}]));
  assert.deepEqual([...children.keys()], ['strings', 'rust_strings', 'continuations', 'continuations_2']);
  const counts = {E0583: 0, E0433: 0, E0425: 0};
  const symbols = new Set();
  for (const diagnostic of suite.diagnostics) {
    assert.ok(plain.includes(diagnostic.block), 'every retained diagnostic block occurs verbatim in the ANSI-decoded authentic log');
    assert.equal(diagnostic.block.split('\n')[0], diagnostic.message);
    assert.equal(/^error\[(E\d+)\]:/u.exec(diagnostic.message)[1], diagnostic.code);
    assert.ok(diagnostic.block.includes(`${diagnostic.file}:${diagnostic.line}:${diagnostic.column}`));
    const name = /`([^`]+)`/u.exec(diagnostic.message)[1];
    const source = suite.sources.find(source => source.path === posix.normalize('rust/' + diagnostic.file));
    assert.ok(source.text.split('\n')[diagnostic.line - 1].includes(name), 'the original head source retains the diagnosed reference at its exact line');
    counts[diagnostic.code] += 1;
    if (diagnostic.code === 'E0583') {
      assert.ok(children.has(name), `the actual explicit child path repairs ${name}`);
      assert.equal(children.get(name).path, `frontend_rules/${name}.rs`);
    } else {
      const declaration = diagnostic.code === 'E0433' ? '(?:enum|struct)' : '(?:const )?fn';
      const matches = [...children].filter(([, child]) => new RegExp(`^pub ${declaration} ${name}\\b`, 'mu').test(child.text));
      assert.equal(matches.length, 1, `${name} must resolve uniquely through an actual loaded public child declaration`);
      symbols.add(name);
    }
  }
  assert.deepEqual(counts, {E0583: 4, E0433: 40, E0425: 13});
  assert.equal(counts.E0433 + counts.E0425, suite.cascadingMissingDeclarations.diagnostics);
  assert.equal(symbols.size, 26);
  assert.ok(symbols.has('accept_lexical_cache_context'));
  assert.ok(symbols.has('accept_lexical_cache_reservation'));
  const corrections = readCompilerFeedbackCorrections({feedback: suite, allowedPaths: suite.sources.map(source => source.path)});
  assert.deepEqual(corrections.actions, []);
  assert.equal(corrections.spans, 0, 'rendered module failures do not invent machine-applicable edits');
});
