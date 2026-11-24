import { createHash } from 'node:crypto';
import { mkdirSync, writeFileSync } from 'node:fs';
import { tokenize } from '../src/translation/lexer.js';
import { formatProductRuleSource } from './product-rule-style-corrections.mjs';

const maximumFileLines = 1000;
const maximumFunctionLines = 900;
const codecDeclaration = /^pub\(super\) fn ([a-z][a-z_0-9]*)\(/gmu;
const helperDeclaration = /^fn ([a-z][a-z_0-9]*)[<(]/gmu;

function lineCount(source) {
  return source.trimEnd().split('\n').length;
}

function partitionCodecSource(source, directory) {
  if (lineCount(source) <= maximumFileLines) return [{ path: '', source }];

  // The emitter creates top-level functions only. rustfmt puts their headers
  // in column one and leaves nested items and match arms indented.
  const declarations = [...source.matchAll(codecDeclaration)];
  if (!declarations.length) throw new TypeError('generated codec needs its declared transport functions');
  const firstHelperAfterCodecs = [...source.matchAll(helperDeclaration)]
    .find(declaration => declaration.index > declarations.at(-1).index);
  const end = firstHelperAfterCodecs?.index ?? source.length;
  const shared = source.slice(0, declarations[0].index) + source.slice(end);
  const exported = new Set(declarations.map(([, name]) => name));
  const available = new Set(['Data', 'json', 'native', ...exported,
    ...[...shared.matchAll(helperDeclaration)].map(([, name]) => name)]);
  const parts = [];
  let group;
  for (const [index, declaration] of declarations.entries()) {
    const fn = source.slice(declaration.index, declarations[index + 1]?.index ?? end).trimEnd();
    const lines = lineCount(fn);
    if (lines > maximumFunctionLines) {
      throw new RangeError(`generated codec function ${declaration[1]} is ${lines} lines`);
    }
    if (!group || group.lines + lines + 1 > maximumFunctionLines) {
      group = { names: new Set(), functions: [], lines: 0 };
      parts.push(group);
    }
    group.names.add(declaration[1]);
    group.functions.push(fn.replace(/^pub\(super\)/u, 'pub(in super::super)'));
    group.lines += lines + 1;
  }
  const files = parts.map(({ names, functions }, index) => {
    const part = functions.join('\n\n');
    const imports = new Set();
    for (const token of tokenize(part, 'Rust').tokens) {
      if (token.kind === 'identifier' && available.has(token.value) && !names.has(token.value)) {
        imports.add(token.value);
      }
    }
    return {
      path: `${directory}/part_${index + 1}.rs`,
      source: formatProductRuleSource(`use super::{${[...imports].join(',')}};\n\n${part}\n`),
    };
  });
  const modules = files.map(({ path }, index) =>
    `#[path = "${path}"]\nmod part_${index + 1};\npub(super) use part_${index + 1}::*;\n`);
  return [{ path: '', source: formatProductRuleSource(shared.trimEnd() + '\n\n' + modules.join('')) }, ...files];
}

export function publishNativeModelCodecFiles(root, output, source) {
  const [, base, name] = /^(rust\/tests\/unit\/)([a-z][a-z0-9_]*)\.rs$/u.exec(output) ?? [];
  if (!base) throw new TypeError('native codec output must name an owned Rust module');
  const files = partitionCodecSource(source, name).map(file => ({
    path: file.path ? base + file.path : output,
    source: file.source,
  }));
  for (const file of files) {
    if (lineCount(file.source) > maximumFileLines) {
      throw new RangeError(`generated codec file exceeds its source limit: ${file.path}`);
    }
  }
  // Publish the entry module after all child modules are available.
  for (const { path, source: text } of [...files.slice(1), files[0]]) {
    const target = new URL(path, root);
    mkdirSync(new URL('.', target), { recursive: true });
    writeFileSync(target, text);
  }
  const manifest = files.map(({ path, source: text }) => ({
    path, bytes: Buffer.byteLength(text), sha256: createHash('sha256').update(text).digest('hex'),
  }));
  const [{ bytes, sha256 }] = manifest;
  return { bytes, sha256, ...(manifest.length > 1 ? { files: manifest } : {}) };
}
