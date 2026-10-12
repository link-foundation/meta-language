import assert from 'node:assert/strict';
import { appendFileSync, mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { spawn } from 'node:child_process';
import test from 'node:test';
import { Parser } from 'links-notation';
import { createStructuredDataRecordReader, readStructuredDataRecords, structuredDataRecordToLino } from '../src/structured-data-records.js';

const first = '00000000-0000-4000-8000-000000000001';
const second = '00000000-0000-4000-8000-000000000002';

test('records have explicit distinct data namespaces and preserve values incrementally', () => {
  const values = [{ runtime: 'javascript', commit: 'a'.repeat(40), value: [1, 'λ', -0] }, { runtime: 'rust', value: [1, false, null] }];
  const text = structuredDataRecordToLino(values[0], first) + structuredDataRecordToLino(values[1], second);
  const identities = new Set();
  for (const frame of new Parser({ comments: false }).parse(text)) {
    assert.match(frame.id, /\/record$/u);
    for (const declaration of frame.values.filter(value => value.values.length && value.id !== 'meta')) {
      assert.equal(identities.has(declaration.id), false);
      identities.add(declaration.id);
      assert.ok(declaration.id.startsWith(frame.id.slice(0, -'record'.length)));
    }
  }
  const records = [];
  const reader = createStructuredDataRecordReader(value => records.push(value));
  for (let offset = 0; offset < text.length; offset += 17) reader.write(text.slice(offset, offset + 17));
  reader.end();
  assert.deepEqual(records, values);
  assert.throws(() => readStructuredDataRecords(text.replace(`${second}/1:`, `${first}/1:`)), /namespace/u);
  assert.throws(() => readStructuredDataRecords(text.replace(`${second}/2 `, `${first}/2 `)), /namespace/u);
  assert.throws(() => readStructuredDataRecords(structuredDataRecordToLino(1, first).repeat(2)), /duplicate namespace/u);
  assert.throws(() => readStructuredDataRecords(text, { nodeLimit: 2 }), /resource limit/u);
  const compact = structuredDataRecordToLino([0, 0], first) + structuredDataRecordToLino([0, 0], second);
  assert.throws(() => readStructuredDataRecords(compact, { nodeLimit: 5 }), /eager collection resource/u);
  let count = 0;
  const incremental = createStructuredDataRecordReader(() => { count += 1; }, { nodeLimit: 5 });
  incremental.write(compact); incremental.end();
  assert.equal(count, 2, 'incremental consumers release complete values between bounded records');
});

test('concurrent atomic appends retain two complete records with identical local identifiers', async () => {
  const directory = mkdtempSync(path.join(tmpdir(), 'structured-records-'));
  const file = path.join(directory, 'records.lino');
  try {
    const module = new URL('../src/structured-data-records.js', import.meta.url).href;
    const write = (value, namespace) => new Promise((resolve, reject) => {
      const child = spawn(process.execPath, ['--input-type=module', '-e',
        `import {appendFileSync} from 'node:fs';import {structuredDataRecordToLino} from ${JSON.stringify(module)};
        appendFileSync(process.argv[1],structuredDataRecordToLino({value:Number(process.argv[2])},process.argv[3]));`, file, String(value), namespace]);
      child.once('error', reject); child.once('exit', code => code === 0 ? resolve() : reject(Error(`append exited ${code}`)));
    });
    await Promise.all([write(1, first), write(2, second)]);
    assert.deepEqual(readStructuredDataRecords(readFileSync(file, 'utf8')).map(record => record.value).sort(), [1, 2]);
    appendFileSync(file, structuredDataRecordToLino({ value: 3 }));
    assert.equal(readStructuredDataRecords(readFileSync(file, 'utf8')).length, 3);
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test('incremental namespace bookkeeping rejects a third record before its callback', () => {
  const third = '00000000-0000-4000-8000-000000000003';
  const callbacks = [];
  const reader = createStructuredDataRecordReader(value => callbacks.push(value), { nodeLimit: 2 });
  const text = [first, second, third].map(namespace => structuredDataRecordToLino(null, namespace)).join('');
  assert.throws(() => { reader.write(text); reader.end(); }, /namespace resource limit exceeded/u);
  assert.deepEqual(callbacks, [null, null]);
});
