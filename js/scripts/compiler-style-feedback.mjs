// Retain the actual compiler diagnostics and their exact generated inputs.
// Offline feedback consumes only explicit machine-applicable suggestions;
// rendered log excerpts never acquire invented applicability metadata.
import { createHash } from 'node:crypto';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { DecoratorSet } from '../src/decorators.js';

const digest = (bytes) => createHash('sha256').update(bytes).digest('hex');

export function retainCompilerFeedback({ root, group, round, stderr, inputs }) {
  const diagnostics = stderr.trim().split('\n').filter(Boolean).map((line) => JSON.parse(line));
  const sources = Object.fromEntries(inputs.map(({ path, relativePath }) => {
    const bytes = readFileSync(path);
    return [path, { relativePath, sha256: digest(bytes), text: bytes.toString('utf8') }];
  }));
  const output = join(root, 'issue-195-artifacts/compiler-feedback', group, `round-${round}.json`);
  mkdirSync(dirname(output), { recursive: true });
  writeFileSync(output, `${JSON.stringify({ schemaVersion: 1, head: process.env.GITHUB_SHA ?? null,
    runId: process.env.GITHUB_RUN_ID ?? null, job: process.env.GITHUB_JOB ?? null,
    compiler: 'clippy-driver', diagnostics, sources }, null, 2)}\n`);
  return diagnostics;
}

export function applyRecordedCompilerFeedback({ root, feedbackFile, decoratorFile, allowedPaths }) {
  const feedback = JSON.parse(readFileSync(feedbackFile, 'utf8'));
  if (feedback.schemaVersion !== 1 || !Array.isArray(feedback.diagnostics) || !feedback.sources) throw new Error('compiler feedback requires raw diagnostics and exact source snapshots');
  const suggestions = [];
  const visit = (diagnostic) => {
    const spans = (diagnostic.spans ?? []).filter(({ suggested_replacement: replacement }) => replacement !== null && replacement !== undefined);
    if (spans.length && spans.every((span) => span.suggestion_applicability === 'MachineApplicable'
      && allowedPaths.includes(feedback.sources[span.file_name]?.relativePath))) {
      suggestions.push(spans);
    }
    for (const child of diagnostic.children ?? []) visit(child);
  };
  for (const diagnostic of feedback.diagnostics) visit(diagnostic.reason === 'compiler-message' ? diagnostic.message : diagnostic);
  const inputs = new Map();
  for (const spans of suggestions) for (const span of spans) {
    const input = feedback.sources[span.file_name];
    if (!inputs.has(span.file_name)) {
      const source = Buffer.from(input.text);
      if (digest(source) !== input.sha256 || !readFileSync(join(root, input.relativePath)).equals(source)) throw new Error(`compiler feedback is stale for ${input.relativePath}`);
      inputs.set(span.file_name, source);
    }
    if (!Number.isSafeInteger(span.byte_start) || !Number.isSafeInteger(span.byte_end)
      || span.byte_start < 0 || span.byte_end < span.byte_start || span.byte_end > inputs.get(span.file_name).length) throw new Error('compiler feedback has an invalid source range');
  }
  const overlaps = (left, right) => {
    if (left.file_name !== right.file_name) return false;
    if (left.byte_start === left.byte_end && right.byte_start === right.byte_end) return left.byte_start === right.byte_start;
    if (left.byte_start === left.byte_end) return right.byte_start <= left.byte_start && left.byte_start < right.byte_end;
    if (right.byte_start === right.byte_end) return left.byte_start <= right.byte_start && right.byte_start < left.byte_end;
    return left.byte_start < right.byte_end && right.byte_start < left.byte_end;
  };
  const edits = new Map();
  const accepted = [];
  let deferred = 0;
  let recordedSpans = 0;
  // Multipart suggestions are indivisible. A conflicting alternative must
  // never leave a binding deletion or a replacement expression half applied.
  for (const spans of suggestions) {
    if (spans.some((span, index) => spans.slice(index + 1).some((other) => overlaps(span, other)))) throw new Error('compiler suggestion contains overlapping source ranges');
    // One literal action cannot span independently generated source files.
    // Keep that transaction for review rather than recording only one side.
    if (new Set(spans.map((span) => span.file_name)).size !== 1) { deferred += 1; continue; }
    const envelope = { file_name: spans[0].file_name,
      byte_start: Math.min(...spans.map((span) => span.byte_start)),
      byte_end: Math.max(...spans.map((span) => span.byte_end)) };
    if (accepted.some((other) => overlaps(envelope, other))) { deferred += 1; continue; }
    accepted.push(envelope);
    const list = edits.get(envelope.file_name) ?? [];
    list.push({ ...envelope, spans }); edits.set(envelope.file_name, list);
    recordedSpans += spans.length;
  }
  const actions = [];
  for (const [path, transactions] of edits) {
    let source = inputs.get(path);
    for (const transaction of transactions.sort((a, b) => b.byte_start - a.byte_start || b.byte_end - a.byte_end)) {
      const before = source;
      let replacement = before.subarray(transaction.byte_start, transaction.byte_end);
      for (const span of [...transaction.spans].sort((a, b) => b.byte_start - a.byte_start || b.byte_end - a.byte_end)) {
        replacement = Buffer.concat([replacement.subarray(0, span.byte_start - transaction.byte_start),
          Buffer.from(span.suggested_replacement), replacement.subarray(span.byte_end - transaction.byte_start)]);
      }
      if (before.subarray(transaction.byte_start, transaction.byte_end).equals(replacement)) continue;
      source = Buffer.concat([before.subarray(0, transaction.byte_start), replacement, before.subarray(transaction.byte_end)]);
      let start = before.lastIndexOf(10, Math.max(0, transaction.byte_start - 1)) + 1;
      const next = before.indexOf(10, transaction.byte_end);
      let end = next < 0 ? before.length : next + 1;
      const delta = replacement.length - (transaction.byte_end - transaction.byte_start);
      for (;;) {
        const from = before.subarray(start, end).toString('utf8');
        const to = source.subarray(start, end + delta).toString('utf8');
        if (before.toString('utf8').split(from).length === 2 && !to.includes(from)) break;
        if (start > 0) start = before.lastIndexOf(10, Math.max(0, start - 2)) + 1;
        else if (end < before.length) {
          const following = before.indexOf(10, end);
          end = following < 0 ? before.length : following + 1;
        } else throw new Error('compiler suggestion has no unique convergent literal context');
      }
      actions.push({ op: 'replace', field: 'source', from: before.subarray(start, end).toString('utf8'),
        to: source.subarray(start, end + delta).toString('utf8').replace(/[ \t]+$/gmu, '') });
    }
  }
  if (!actions.length) throw new Error('compiler feedback contains no applicable machine suggestions; rendered logs require separate reviewed corrections');
  const path = join(root, decoratorFile);
  const existing = readFileSync(path, 'utf8');
  const previous = DecoratorSet.fromLino(existing).decorators.find(({ id }) => id === 'compiler-feedback-style');
  const data = new DecoratorSet([{ id: 'compiler-feedback-style', level: 'emitter', order: 1000,
    when: [['format', 'Rust'], ['scope', 'formatted-module']], actions: [...(previous?.actions ?? []), ...actions] }]).toLino();
  const lines = existing.split('\n').filter((line) => !line.startsWith('(decorator compiler-feedback-style '));
  writeFileSync(path, `${lines.join('\n').trimEnd()}\n\n${data}`);
  process.stdout.write(`Recorded ${actions.length} atomic compiler transactions covering ${recordedSpans} machine-applicable spans; deferred ${deferred} complete suggestions. CI must verify regenerated output.\n`);
  return { actions: actions.length, spans: recordedSpans, deferred };
}
